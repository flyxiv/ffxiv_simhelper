use crate::damage_rdps_profile::{FfxivRaidDamageTable, RaidDamageTable};
use crate::simulation_result::RotationLog;
use crate::skill_simulator::{FfxivSkillSimulator, SkillSimulationResult, SkillSimulator};
use crate::turn_calculator::{FfxivTurnCalculator, TurnCalculator};
use ffxiv_simbot_combat_components::owner_tracker::OwnerTracker;
use ffxiv_simbot_combat_components::{DamageProfileTable, DamageType, DpsType, IdType, TimeType};

use ffxiv_simbot_combat_components::id_entity::IdEntity;
use ffxiv_simbot_combat_components::live_objects::player::ffxiv_player::FfxivPlayer;
use ffxiv_simbot_combat_components::live_objects::player::Player;
use ffxiv_simbot_combat_components::live_objects::target::ffxiv_target::FfxivTarget;
use ffxiv_simbot_combat_components::live_objects::target::Target;
use ffxiv_simbot_combat_components::rotation::cooldown_timer::CooldownTimer;
use ffxiv_simbot_combat_components::rotation::priority_table::SkillResult;
use ffxiv_simbot_combat_components::skill::attack_skill::{AttackSkill, SkillInfo};
use ffxiv_simbot_combat_components::skill::Skill;
use ffxiv_simbot_combat_components::status::buff_status::BuffStatus;
use ffxiv_simbot_combat_components::status::debuff_status::DebuffStatus;
use ffxiv_simbot_combat_components::status::status_holder::StatusHolder;
use ffxiv_simbot_combat_components::status::status_timer::StatusTimer;
use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::rc::Rc;

pub static SIMULATION_START_TIME_MILLISECOND: TimeType = -5000;

#[derive(Clone)]
enum EventType {
    PlayerTurn,
    InflictDamage(Vec<SkillInfo<AttackSkill>>),
}

struct CombatEvent {
    event_type: EventType,
    event_time_millisecond: TimeType,
}

/// Simulate DPS for a job based on Priority System
/// 1) Read the priority table and get the next skill
/// 2) Calculate the expected damage of the skill, and distribute RDPS to the applied buffs
/// 3) After simulation is done, return the DPS and Simulation Log of the job.
/// Simulating all 8 Jobs at once is really tough, so we're gonna only count for 1 Job first.
pub trait SimulationBoard<T, P, S>
where
    T: Target + Sized,
    P: Player + Sized,
    S: Skill + Sized,
{
    fn run_simulation(&self, target_combat_time_millsecond: TimeType);
    /// Gets the RDPS Profile by each buff. Raw Damage is id 0.
    fn get_simulation_result(&self) -> HashMap<IdType, DpsType>;
}

pub struct PlayerSimulationData<P, S>
where
    P: Player,
    S: Skill,
{
    pub player: P,
    pub skill: S,
    pub rdps_table: HashMap<IdType, DpsType>,
}

/// The main party combat simluation board for FFXIV. Think of this simulation of one instance of combat.
/// The DpsSimulator does the following:
/// Keeps track of the party and target data.
/// Keeps RDPS contribution for all buffs that are in the party.
pub struct FfxivSimulationBoard {
    skill_simulator: FfxivSkillSimulator,
    turn_calculator: FfxivTurnCalculator,

    pub(crate) damage_profiles: Vec<Rc<RefCell<DamageProfileTable>>>,
    pub(crate) rotation_logs: Rc<RefCell<HashMap<IdType, Vec<RotationLog>>>>,

    current_turn_player_id: RefCell<IdType>,
    pub(crate) party: Vec<Rc<RefCell<FfxivPlayer>>>,
    target: Rc<RefCell<FfxivTarget>>,
    pub(crate) rdps_table: RefCell<FfxivRaidDamageTable>,

    current_combat_time_millisecond: RefCell<TimeType>,
    pub(crate) finish_combat_time_millisecond: TimeType,
    pub(crate) skill_queue: RefCell<HashMap<TimeType, Vec<SkillInfo<AttackSkill>>>>,
}

impl SimulationBoard<FfxivTarget, FfxivPlayer, AttackSkill> for FfxivSimulationBoard {
    fn run_simulation(&self, target_combat_time_millsecond: TimeType) {
        loop {
            println!(
                "combat_time: {}",
                *self.current_combat_time_millisecond.borrow()
            );
            println!("turn: {}", *self.current_turn_player_id.borrow());

            if self.combat_time_exceeded_finish_time(target_combat_time_millsecond) {
                break;
            }

            let next_event = self.find_next_event();
            self.update_time_related_informations(next_event.event_time_millisecond);

            self.simulate_event(next_event);
        }
    }

    fn get_simulation_result(&self) -> HashMap<IdType, DpsType> {
        todo!()
    }
}

impl FfxivSimulationBoard {
    fn simulate_event(&self, combat_event: CombatEvent) {
        match combat_event.event_type {
            EventType::PlayerTurn => {
                self.use_skill();
            }
            EventType::InflictDamage(skills) => {
                for skill in skills {
                    if skill.is_auto_attack() {
                        self.add_next_auto_attack_to_queue(
                            &skill,
                            combat_event.event_time_millisecond,
                        );
                    }

                    self.simulate_skill(skill);
                }
            }
        }
    }

    fn add_next_auto_attack_to_queue(
        &self,
        skill_info: &SkillInfo<AttackSkill>,
        event_time_millisecond: TimeType,
    ) {
        let mut next_auto_attack = skill_info.clone();
        let auto_attack_player = self.get_player_data(next_auto_attack.skill.get_owner_id());

        next_auto_attack.damage_inflict_time_millisecond = Some(
            event_time_millisecond
                + auto_attack_player.get_gcd_delay_millisecond(&next_auto_attack.skill),
        );

        self.add_to_skill_queue(next_auto_attack);
    }

    #[inline]
    fn combat_time_exceeded_finish_time(&self, target_combat_time_millsecond: TimeType) -> bool {
        *self.current_combat_time_millisecond.borrow() >= target_combat_time_millsecond
    }

    fn add_buff(&self, buff: BuffStatus) {
        if buff.is_raidwide {
            for player in self.party.clone().into_iter() {
                player.borrow_mut().add_status(buff.clone());
            }
        } else {
            let owner_player_id = self.current_turn_player_id.clone();
            let owner_player_id = *owner_player_id.borrow();

            for player in self.party.clone() {
                if player.borrow().get_id() == owner_player_id {
                    player.borrow_mut().add_status(buff.clone());
                }
            }
        }
    }

    fn add_debuff(&self, debuff: DebuffStatus) {
        self.target.borrow_mut().add_status(debuff);
    }

    fn update_damage_profile(&self, skill_id: IdType, raw_damage: DamageType) {
        let current_turn_player_id = self.current_turn_player_id.clone();
        let current_turn_player_id = current_turn_player_id.borrow();

        let damage_profile = self.damage_profiles[*current_turn_player_id].clone();
        let mut damage_profile = damage_profile.borrow_mut();

        if let Some(cumulative_damage) = damage_profile.get_mut(&skill_id) {
            *cumulative_damage += raw_damage;
        } else {
            damage_profile.insert(skill_id, raw_damage);
        }
    }

    fn update_rdps_table(&self, raid_damage_profile: FfxivRaidDamageTable) {
        let mut rdps_table = self.rdps_table.borrow_mut();

        for (skill_key, damage_contribution) in raid_damage_profile.rdps_table {
            if let Some(current_damage) = rdps_table.rdps_table.get_mut(&skill_key) {
                *current_damage += damage_contribution;
            } else {
                rdps_table.insert(skill_key, damage_contribution);
            }
        }
    }

    fn get_current_player(&self) -> Rc<RefCell<FfxivPlayer>> {
        let party = &self.party;
        let owner_player_id = self.current_turn_player_id.clone();

        let player = party[*owner_player_id.borrow()].clone();
        player
    }

    fn get_player_data(&self, player_id: IdType) -> Ref<FfxivPlayer> {
        let party_ref = &self.party;
        let player_search = party_ref.get(player_id).unwrap();

        player_search.borrow()
    }

    fn get_target(&self) -> Rc<RefCell<FfxivTarget>> {
        self.target.clone()
    }

    fn update_skill_simulation_result(&self, skill_id: IdType, result: SkillSimulationResult) {
        let skill_damage_result = result.skill_damage_result;
        let buff = result.buff;
        let debuff = result.debuff;

        if let Some(buff) = buff {
            self.add_buff(buff);
        }

        if let Some(debuff) = debuff {
            self.add_debuff(debuff);
        }

        self.update_damage_profile(skill_id, skill_damage_result.raw_damage);
        self.update_rdps_table(skill_damage_result.raid_damage_profile);
    }

    fn find_next_event(&self) -> CombatEvent {
        let next_turn_player = self.get_current_player();
        let next_turn_time_millisecond = next_turn_player.borrow().get_next_turn_time_millisecond();

        let next_damage_time = self.skill_queue.borrow();
        let next_damage_time = next_damage_time.keys().min().unwrap();

        if *next_damage_time < next_turn_time_millisecond {
            let skills = self
                .skill_queue
                .borrow_mut()
                .remove(next_damage_time)
                .unwrap();
            CombatEvent {
                event_type: EventType::InflictDamage(skills),
                event_time_millisecond: *next_damage_time,
            }
        } else {
            CombatEvent {
                event_time_millisecond: next_turn_time_millisecond,
                event_type: EventType::PlayerTurn,
            }
        }
    }

    fn find_next_player(&self) {
        let next_turn_player_id = self.turn_calculator.find_next_turn_player_id(&self.party);

        *self.current_turn_player_id.borrow_mut() = next_turn_player_id;
    }

    fn request_skill_simulation(
        &self,
        skill_info: SkillInfo<AttackSkill>,
    ) -> SkillSimulationResult {
        let current_turn_player_id = self.current_turn_player_id.clone();

        let skill_damage_result = self.skill_simulator.make_skill_simulation_result(
            *current_turn_player_id.borrow(),
            &self.party,
            self.target.clone(),
            &skill_info,
        );

        let skill = skill_info.skill;
        let buff = skill.buff;
        let debuff = skill.debuff;
        let skill_user_id = *current_turn_player_id.borrow();

        SkillSimulationResult {
            skill_damage_result,
            buff,
            debuff,
            skill_user_id,
        }
    }

    fn get_next_skill(&self) -> Option<SkillResult<AttackSkill>> {
        let current_turn_player = self.get_current_player();
        let current_turn_player = current_turn_player.borrow();
        let target = self.get_target();
        let target = target.borrow();

        current_turn_player.get_next_skill(target.get_status_list())
    }

    fn use_skill(&self) {
        let skill_result = self.get_next_skill();
        let mut skill_info = None;

        if let Some(skill_result) = skill_result {
            match skill_result {
                SkillResult::Delay(delay_time) => {
                    let current_player = self.get_current_player();
                    current_player.borrow_mut().delay_turn_by(delay_time);
                    return;
                }
                SkillResult::UseSkill(skill_info_of_result) => {
                    let current_player = self.get_current_player();
                    current_player
                        .borrow_mut()
                        .calculate_next_turn(skill_info_of_result[0].skill.get_delay_millisecond());
                    skill_info = Some(skill_info_of_result);
                }
            }
        } else {
            return;
        }

        let skill_infos = skill_info.unwrap();

        for skill_info in skill_infos {
            self.update_current_player_gcd_time(&skill_info.skill);

            if skill_info.damage_inflict_time_millisecond.is_some() {
                self.add_to_skill_queue(skill_info);
            } else {
                self.simulate_skill(skill_info);
            }
        }
    }

    fn update_current_player_gcd_time(&self, skill: &AttackSkill) {
        if !skill.is_gcd() {
            return;
        }

        let current_turn_player = self.get_current_player();
        let gcd_time_millisecond = current_turn_player.borrow().get_next_gcd_time_millisecond();
        let mut current_turn_player = current_turn_player.borrow_mut();

        current_turn_player.set_last_gcd_time_millisecond(gcd_time_millisecond);
        current_turn_player.set_next_gcd_time_millisecond(skill);
    }

    fn simulate_skill(&self, skill_info: SkillInfo<AttackSkill>) {
        let skill_id = skill_info.skill.get_id();
        let simulation_result = self.request_skill_simulation(skill_info);

        self.update_skill_simulation_result(skill_id, simulation_result);
        self.add_skill_to_rotation_log(skill_id);
    }

    fn add_to_skill_queue(&self, skill_info: SkillInfo<AttackSkill>) {
        let mut skill_queue = self.skill_queue.borrow_mut();
        let time_millisecond = skill_info.damage_inflict_time_millisecond.unwrap();

        if let Some(skill_list) = skill_queue.get_mut(&time_millisecond) {
            skill_list.push(skill_info);
        } else {
            skill_queue.insert(time_millisecond, vec![skill_info]);
        }
    }

    fn add_skill_to_rotation_log(&self, skill_id: IdType) {
        if let Some(skill_rotation_list) = self
            .rotation_logs
            .borrow_mut()
            .get_mut(&self.current_turn_player_id.borrow())
        {
            skill_rotation_list.push(RotationLog {
                casted_time_millisecond: *self.current_combat_time_millisecond.borrow(),
                skill_id,
            })
        }
    }

    fn update_time_related_informations(&self, next_event_time: TimeType) {
        self.update_player_target_time_informations(next_event_time);
        self.update_combat_time(next_event_time);
    }

    #[inline]
    fn update_combat_time(&self, elapsed_time: TimeType) {
        *self.current_combat_time_millisecond.borrow_mut() += elapsed_time;
    }

    fn get_elapsed_time(&self) -> TimeType {
        let current_turn_player = self.get_current_player();

        let next_turn_time_millisecond = current_turn_player
            .borrow()
            .get_next_turn_time_millisecond();

        next_turn_time_millisecond - *self.current_combat_time_millisecond.borrow()
    }

    fn update_player_target_time_informations(&self, elapsed_time: TimeType) {
        for player in self.party.clone() {
            player.borrow_mut().update_status_time(elapsed_time);
            player.borrow_mut().update_cooldown(elapsed_time);
        }

        self.target.borrow_mut().update_status_time(elapsed_time);
    }

    fn get_final_rotation_log(&self) -> Vec<PlayerSimulationData<FfxivPlayer, AttackSkill>> {
        todo!()
    }

    fn get_final_rdps_table(&self) -> HashMap<IdType, DpsType> {
        todo!()
    }
    pub fn new(party: Vec<Rc<RefCell<FfxivPlayer>>>, target: Rc<RefCell<FfxivTarget>>) -> Self {
        let mut damage_profiles = vec![];

        for _ in 0..party.len() {
            damage_profiles.push(Rc::new(RefCell::new(DamageProfileTable::new())));
        }

        FfxivSimulationBoard {
            skill_simulator: FfxivSkillSimulator::default(),
            turn_calculator: FfxivTurnCalculator::default(),
            damage_profiles,
            rotation_logs: Rc::new(RefCell::new(HashMap::new())),
            current_turn_player_id: RefCell::new(0),
            party,
            target,
            rdps_table: RefCell::new(FfxivRaidDamageTable::default()),
            current_combat_time_millisecond: RefCell::new(-SIMULATION_START_TIME_MILLISECOND),
            finish_combat_time_millisecond: 0,
            skill_queue: Default::default(),
        }
    }
}

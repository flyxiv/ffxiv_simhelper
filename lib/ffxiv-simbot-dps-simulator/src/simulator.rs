use crate::damage_rdps_profile::{FfxivRaidDamageTable, RaidDamageTable};
use crate::skill_simulator::{FfxivSkillSimulator, SkillSimulationResult, SkillSimulator};
use crate::turn_calculator::{FfxivTurnCalculator, TurnCalculator};
use crate::util::calculate_gcd;
use ffxiv_simbot_combat_components::player::{FfxivPlayer, Player};
use ffxiv_simbot_combat_components::skill::{
    AttackSkill, Skill, SkillInfo, NON_GCD_DELAY_MILLISECOND,
};
use ffxiv_simbot_combat_components::status::{BuffStatus, DebuffStatus, StatusHolder, StatusTimer};
use ffxiv_simbot_combat_components::target::{FfxivTarget, Target};
use ffxiv_simbot_combat_components::{DamageProfileTable, DamageType, DpsType, IdType, TimeType};
use ffxiv_simbot_db::DamageMultiplierType;
use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::rc::Rc;

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

    damage_profiles: Vec<Rc<RefCell<DamageProfileTable>>>,

    current_turn_player_id: RefCell<IdType>,
    party: Vec<Rc<RefCell<FfxivPlayer>>>,
    target: Rc<RefCell<FfxivTarget>>,
    rdps_table: RefCell<FfxivRaidDamageTable>,

    current_combat_time_millisecond: RefCell<TimeType>,
}

impl SimulationBoard<FfxivTarget, FfxivPlayer, AttackSkill> for FfxivSimulationBoard {
    fn run_simulation(&self, target_combat_time_millsecond: TimeType) {
        loop {
            println!(
                "combat_time: {}",
                *self.current_combat_time_millisecond.borrow()
            );
            println!("turn: {}", *self.current_turn_player_id.borrow());
            if *self.current_combat_time_millisecond.borrow() >= target_combat_time_millsecond {
                break;
            }

            self.update_turn();
            let skill_info = self.get_next_skill();

            if let Some(skill_info) = skill_info {
                self.use_skill(skill_info);
            }

            let current_combat_time_millisecond = *self.current_combat_time_millisecond.borrow();
            let current_player = self.get_current_player();

            current_player.borrow_mut().calculate_next_turn();
        }
    }

    fn get_simulation_result(&self) -> HashMap<IdType, DpsType> {
        todo!()
    }
}

impl FfxivSimulationBoard {
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
        let owner_player_id = owner_player_id.borrow();

        party[*owner_player_id].clone()
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

    fn update_turn(&self) {
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

    fn get_next_skill(&self) -> Option<SkillInfo<AttackSkill>> {
        let current_turn_player = self.get_current_player();
        let current_turn_player = current_turn_player.borrow();
        let target = self.get_target();
        let target = target.borrow();

        current_turn_player.get_next_skill(target.get_status_list())
    }

    fn use_skill(&self, skill_info: SkillInfo<AttackSkill>) {
        let skill = &skill_info.skill;

        let current_turn_player = self.get_current_player();
        let next_turn_time_millisecond = current_turn_player
            .borrow()
            .get_next_turn_time_milliseconds();

        {
            let mut current_turn_player = current_turn_player.borrow_mut();

            if skill.is_gcd() {
                let gcd_delay_millisecond = calculate_gcd(
                    skill.get_cooldown_millisecond(),
                    current_turn_player.get_player_power().speed_multiplier,
                );

                current_turn_player.set_next_gcd_time_milliseconds(gcd_delay_millisecond);
            }
        }

        let elapsed_time =
            *self.current_combat_time_millisecond.borrow() - next_turn_time_millisecond;

        for player_pointer in self.party.clone() {
            player_pointer.borrow_mut().update_status_time(elapsed_time)
        }

        self.target.borrow_mut().update_status_time(elapsed_time);

        *self.current_combat_time_millisecond.borrow_mut() = next_turn_time_millisecond;

        let skill_id = skill_info.skill.get_id();
        let simulation_result = self.request_skill_simulation(skill_info);
        self.update_skill_simulation_result(skill_id, simulation_result);
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
            current_turn_player_id: RefCell::new(0),
            party,
            target,
            rdps_table: RefCell::new(FfxivRaidDamageTable::default()),
            current_combat_time_millisecond: RefCell::new(0),
        }
    }
}

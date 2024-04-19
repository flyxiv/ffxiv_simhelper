use crate::damage_rdps_profile::{FfxivRaidDamageTable, RaidDamageTable};
use crate::skill_simulator::{FfxivSkillSimulator, SkillSimulationResult, SkillSimulator};
use crate::turn_calculator::{FfxivTurnCalculator, TurnCalculator};
use crate::util::round_to;
use ffxiv_simbot_combat_components::player::{FfxivPlayer, Player};
use ffxiv_simbot_combat_components::skill::{
    AttackSkill, Skill, SkillInfo, NON_GCD_DELAY_MILLISECOND,
};
use ffxiv_simbot_combat_components::status::{
    BuffStatus, DebuffStatus, Status, StatusHolder, StatusTimer,
};
use ffxiv_simbot_combat_components::target::{FfxivTarget, Target};
use ffxiv_simbot_combat_components::{DamageProfileTable, DamageType, DpsType, IdType, TimeType};
use ffxiv_simbot_db::DamageMultiplierType;
use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;
use std::rc::Rc;
use std::thread::current;

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
    fn get_player_data(&self, player_id: IdType) -> Ref<P>;
    fn get_target(&self) -> Ref<T>;
    fn update_skill_simulation_result(&self, skill_id: IdType, result: SkillSimulationResult);
    fn update_turn(&self);
    fn request_skill_simulation(&self, skill: SkillInfo<S>);
    fn use_skill(&mut self, skill: &AttackSkill);
    fn get_final_rotation_log(&self) -> Vec<PlayerSimulationData<P, S>>;
    /// Gets the RDPS Profile by each buff. Raw Damage is id 0.
    fn get_final_rdps_table(&self) -> HashMap<IdType, DpsType>;
    fn get_finish_combat_time_millisecond(&self) -> TimeType;
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
    turn_simulator: FfxivTurnCalculator,

    damage_profiles: Vec<DamageProfileTable>,

    current_turn_player_id: IdType,
    party: Vec<Rc<RefCell<FfxivPlayer>>>,
    target: Rc<RefCell<FfxivTarget>>,
    rdps_table: RefCell<FfxivRaidDamageTable>,

    current_combat_time_millisecond: RefCell<TimeType>,
    finish_combat_time_millisecond: TimeType,
}

impl SimulationBoard<FfxivTarget, FfxivPlayer, AttackSkill> for FfxivSimulationBoard {
    fn get_player_data(&self, player_id: IdType) -> Ref<FfxivPlayer> {
        let party_ref = &self.party;
        let player_search = party_ref.get(player_id).unwrap();

        player_search.borrow()
    }

    fn get_target(&self) -> Ref<FfxivTarget> {
        self.target.borrow()
    }

    fn update_skill_simulation_result(&self, skill_id: IdType, result: SkillSimulationResult) {
        let skill_damage_result = result.skill_damage_result;
        let skill_user_id = result.skill_user_id;
        let buff = result.buff;
        let debuff = result.debuff;

        if let Some(buff) = buff {
            self.add_buff(buff);
        }

        if let Some(debuff) = debuff {
            self.add_debuff(debuff);
        }

        let party = &self.party;
        let owner_player = party.get(skill_user_id).unwrap();
        owner_player.update_damage_profile(skill_id, skill_damage_result.raw_damage);

        self.update_rdps_table(skill_damage_result.raid_damage_profile);
    }

    fn update_turn(&mut self) {
        let next_turn_player_id = self.turn_simulator.find_next_turn_player_id(&self.party);
        let mut current_turn_player_id = self.current_turn_player_id;

        *current_turn_player_id = next_turn_player_id;
    }

    fn request_skill_simulation(
        &self,
        skill_info: SkillInfo<AttackSkill>,
    ) -> SkillSimulationResult {
        let skill_damage_result = self.skill_simulator.make_skill_simulation_result(
            *self.current_turn_player_id,
            &self.party,
            self.target.clone(),
            &skill_info,
        );

        let skill = skill_info.skill;
        let buff = skill.buff;
        let debuff = skill.debuff;

        SkillSimulationResult {
            skill_damage_result,
            buff,
            debuff,
            skill_user_id: *self.current_turn_player_id,
        }
    }

    fn use_skill(&self, skill: &AttackSkill) {
        let mut current_turn_player = self.get_current_player_mut();
        let next_turn_time_millisecond = current_turn_player.get_next_turn_time_milliseconds();
        let mut total_delay = current_turn_player.get_delay();

        if skill.is_gcd() {
            let gcd_delay_millisecond = round_to(
                skill.get_cooldown_millisecond() as DamageMultiplierType
                    / current_turn_player.get_player_power().speed_multiplier,
                1,
            ) as TimeType;

            current_turn_player.set_next_gcd_time_milliseconds(gcd_delay_millisecond);
            total_delay = NON_GCD_DELAY_MILLISECOND;
        } else {
            total_delay += NON_GCD_DELAY_MILLISECOND;
        }

        let mut current_combat_time_millsecond = self.current_combat_time_millisecond.borrow_mut();
        let elapsed_time = *current_combat_time_millsecond - next_turn_time_millisecond;

        for player_pointer in self.party.clone() {
            let mut player = player_pointer.borrow_mut();
            player.update_status_time(elapsed_time)
        }

        let mut target = self.target.borrow_mut();
        target.update_status_time(elapsed_time);

        *current_combat_time_millsecond = next_turn_time_millisecond;

        let mut current_player = self.get_current_player_mut();
        let current_combat_time_millisecond = *self.current_combat_time_millisecond.borrow();
        current_player.calculate_next_turn(current_combat_time_millisecond);

        let current_player = self.get_current_player();
        let skill_info = current_player.get_next_skill(target.get_status_list());
        let skill_id = skill_info.skill.get_id();

        let skill_simulation_result = self.request_skill_simulation(skill_info);
        self.update_skill_simulation_result(skill_id, skill_simulation_result);
    }

    fn get_final_rotation_log(&self) -> Vec<PlayerSimulationData<FfxivPlayer, AttackSkill>> {
        todo!()
    }

    fn get_final_rdps_table(&self) -> HashMap<IdType, DpsType> {
        todo!()
    }

    fn get_finish_combat_time_millisecond(&self) -> TimeType {
        self.finish_combat_time_millisecond
    }
}

impl FfxivSimulationBoard {
    fn add_buff(&self, buff: BuffStatus) {
        if buff.is_raidwide {
            for player in self.party.clone().into_iter() {
                let mut player = player.borrow_mut();
                player.add_status(buff.clone());
            }
        } else {
            let owner_player_id = self.current_turn_player_id;

            for player in self.party {
                let mut player = player.borrow_mut();

                if player.get_id() == owner_player_id {
                    player.add_status(buff.clone());
                }
            }
        }
    }

    fn add_debuff(&self, debuff: DebuffStatus) {
        let mut target = self.target.borrow_mut();

        target.add_status(debuff);
    }

    fn update_damage_profile(&mut self, skill_id: IdType, raw_damage: DamageType) {
        let damage_profile = &mut self.damage_profiles[skill_id];

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

    fn get_current_player(&self) -> Ref<FfxivPlayer> {
        let party = &self.party;
        let owner_player_id = self.current_turn_player_id;

        let owner_player = party[owner_player_id].clone();

        owner_player.borrow()
    }

    fn get_current_player_mut(&self) -> RefMut<FfxivPlayer> {
        let party = &self.party;
        let owner_player_id = self.current_turn_player_id;

        let owner_player = party[owner_player_id].clone();

        owner_player.borrow_mut()
    }
}

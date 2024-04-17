use crate::player::{FfxivPlayer, Player};
use crate::priority_table::PriorityTable;
use crate::skill::Skill;
use crate::target::Target;
use crate::{DamageType, DpsType, IdType, TimeType};
use ffxiv_simbot_lib_db::stat_calculator::CharacterPower;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use crate::multiplier_calculator::FfxivMultiplierCalculator;
use crate::status::Status;

/// Simulate DPS for a job based on Priority System
/// 1) Read the priority table and get the next skill
/// 2) Calculate the expected damage of the skill, and distribute RDPS to the applied buffs
/// 3) After simulation is done, return the DPS and Simulation Log of the job.
/// Simulating all 8 Jobs at once is really tough, so we're gonna only count for 1 Job first.
pub trait DpsSimulator<T, P, S>
where
    T: Target + Sized,
    P: Player + Sized,
    S: Skill + Sized,
{
    fn get_player_data(&self, player_id: IdType) -> P;
    fn get_target(&self) -> T;
    fn get_rotation_log(&self) -> &Vec<S>;
    fn get_players(&self) -> &Vec<P>;
    fn get_rotation_log_mut(&self) -> &mut Vec<S>;
    fn get_players_mut(&self) -> &mut Vec<P>;
    fn use_skill(&self, player_id: IdType, skill: S);
    fn get_final_rotation_log(&self) -> Vec<PlayerSimulationData<P, S>>;
    /// Gets the RDPS Profile by each buff. Raw Damage is id 0.
    fn get_final_rdps_table(&self) -> HashMap<IdType, DpsType>;
    fn get_finish_combat_time(&self) -> TimeType;
    fn record_damage_contributions<S>(
        &self,
        status_list: Vec<S>,
        damage: DamageType,
    ) where S: Status + Sized {
        for status in status_list {
            self.update_buff_score(status.get_id(), damage);
        }
    }
}

pub(crate) struct PlayerSimulationData<P, S>
where
    P: Player + Sized,
    S: Skill + Sized,
{
    rotation_log: Vec<S>,
    player: P,
}

/// The main party combat simluation board for FFXIV. Think of this simulation of one instance of combat.
/// The DpsSimulator does the following:
/// Keeps track of the party and target data.
/// Keeps RDPS contribution for all buffs that are in the party.
pub struct FfxivDpsSimulator<T, P, S>
where
    T: Target,
    P: Player,
    S: Skill,
{
    multiplier_calculator: FfxivMultiplierCalculator,
    rdps_distributor: FfxivRdpsDistributor,
    turn_simulator: FfxivTurnSimulator

    player_simulation_data: Rc<RefCell<Vec<PlayerSimulationData<P, S>>>>,
    target: Rc<RefCell<T>>,
    rdps_table: HashMap<IdType, DpsType>,
    finish_combat_time: TimeType,
}

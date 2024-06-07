use crate::simulation_result::SimulationResult;
use ffxiv_simbot_combat_components::live_objects::player::Player;
use ffxiv_simbot_combat_components::live_objects::target::Target;
use ffxiv_simbot_combat_components::skill::Skill;
use ffxiv_simbot_combat_components::{DpsType, IdType, TimeType};
use std::collections::HashMap;

pub mod ffxiv_simulation_board;
pub static SIMULATION_START_TIME_MILLISECOND: TimeType = -5000;
static INFINITE_TIME: TimeType = 10000000;

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
    fn run_simulation(&self);
    /// Gets the RDPS Profile by each buff. Raw Damage is id 0.
    fn create_simulation_result(&self) -> SimulationResult;
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

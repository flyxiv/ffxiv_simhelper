use ffxiv_simbot_combat_components::live_objects::player::logs::{DamageLog, SkillLog};
use ffxiv_simbot_combat_components::{IdType, TimeType};

/// Saves all the raw data from the simulation
/// and aggregates raw data to needed format depending on the requested query.
#[derive(Debug, Clone)]
pub struct SimulationResult {
    pub main_player_id: IdType,
    pub combat_time_millisecond: TimeType,
    pub party_simulation_results: Vec<PartySimulationResult>,
}

#[derive(Debug, Clone)]
pub struct PartySimulationResult {
    pub player_id: IdType,
    pub job: String,
    pub role: String,
    pub skill_log: Vec<SkillLog>,
    pub damage_log: Vec<DamageLog>,
}

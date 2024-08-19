use ffxiv_simbot_combat_components::live_objects::player::player_power::PlayerPower;
use ffxiv_simbot_combat_components::types::{IdType, PlayerIdType, StatType, TimeType};
use serde::Deserialize;

/// The main request body for the simulation API
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SimulationApiRequest {
    pub main_player_id: PlayerIdType,
    pub combat_time_millisecond: TimeType,
    pub party: Vec<PlayerInfoRequest>,
}

/// Data of individual players needed to simulate their DPS
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlayerInfoRequest {
    pub player_id: PlayerIdType,
    pub partner1_id: Option<PlayerIdType>,
    pub partner2_id: Option<PlayerIdType>,
    pub job_abbrev: String,
    pub power: PlayerPower,
}

#[derive(Debug, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct StatsInfo {
    pub weapon_damage: StatType,
    pub main_stat: StatType,
    pub critical_strike: StatType,
    pub direct_hit: StatType,
    pub determination: StatType,
    pub speed: StatType,
    pub tenacity: StatType,
}

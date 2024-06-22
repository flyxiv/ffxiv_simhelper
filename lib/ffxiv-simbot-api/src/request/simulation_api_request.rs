use ffxiv_simbot_combat_components::{IdType, TimeType};
use ffxiv_simbot_db::stat::StatType;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SimulationApiRequest {
    pub main_player_id: IdType,
    pub combat_time_millisecond: TimeType,
    pub party: Vec<PlayerInfoRequest>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlayerInfoRequest {
    pub player_id: IdType,
    pub job: String,
    pub stats: StatsRequest,
}

#[derive(Debug, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct StatsRequest {
    pub weapon_damage: StatType,
    pub main_stat: StatType,
    pub critical_strike: StatType,
    pub direct_hit: StatType,
    pub determination: StatType,
    pub speed: StatType,
    pub tenacity: StatType,
}

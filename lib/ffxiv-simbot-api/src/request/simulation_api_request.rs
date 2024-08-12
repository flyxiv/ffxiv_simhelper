use crate::{MultiplierType, StatType};
use ffxiv_simbot_combat_components::{IdType, TimeType};
use serde::{Deserialize, Serialize};

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
    pub partner1_id: Option<IdType>,
    pub partner2_id: Option<IdType>,
    pub jobAbbrev: String,
    pub stats: StatsInfo,
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

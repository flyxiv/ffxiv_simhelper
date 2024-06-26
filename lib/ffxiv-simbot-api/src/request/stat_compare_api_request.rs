use crate::request::simulation_api_request::{PlayerInfoRequest, StatsRequest};
use ffxiv_simbot_combat_components::{IdType, TimeType};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StatCompareApiRequest {
    pub main_player_id: IdType,
    pub main_player_job: String,
    pub main_player_partner1_id: Option<IdType>,
    pub main_player_partner2_id: Option<IdType>,
    pub combat_time_millisecond: TimeType,
    pub main_player_stat1: StatsRequest,
    pub main_player_stat2: StatsRequest,
    pub party: Vec<PlayerInfoRequest>,
}

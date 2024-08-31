use crate::request::simulation_api_request::{PlayerInfoRequest, SimulationApiRequest};
use ffxiv_simbot_combat_components::types::{PlayerIdType, StatType, TimeType};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StatWeightsApiRequest {
    pub main_player_id: PlayerIdType,
    pub combat_time_millisecond: TimeType,
    pub party: Vec<PlayerInfoRequest>,
    pub stat_name: String,
    pub augment_amount: StatType,
}

impl From<&StatWeightsApiRequest> for SimulationApiRequest {
    fn from(request: &StatWeightsApiRequest) -> Self {
        SimulationApiRequest {
            main_player_id: request.main_player_id,
            combat_time_millisecond: request.combat_time_millisecond,
            party: request.party.clone(),
        }
    }
}

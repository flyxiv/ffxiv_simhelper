use crate::request::simulation_api_request::{PlayerInfoRequest, SimulationApiRequest};
use ffxiv_simhelper_combat_components::types::{PlayerIdType, StatType, TimeType};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BestStatsApiRequest {
    pub main_player_id: PlayerIdType,
    pub combat_time_millisecond: TimeType,
    pub party: Vec<PlayerInfoRequest>,
    pub stat_name: String,
    pub augment_amount: StatType,
    pub party_ilvl_adjustment: f64,
    pub use_pot: bool,
}

impl From<&BestStatsApiRequest> for SimulationApiRequest {
    fn from(request: &BestStatsApiRequest) -> Self {
        SimulationApiRequest {
            main_player_id: request.main_player_id,
            combat_time_millisecond: request.combat_time_millisecond,
            party: request.party.clone(),
            use_pot: request.use_pot,
            party_ilvl_adjustment: request.party_ilvl_adjustment,
        }
    }
}

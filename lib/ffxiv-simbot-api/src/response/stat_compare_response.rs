use crate::response::simulation_api_response::{SimulationApiResponse, SimulationSummaryResponse};
use serde::Serialize;

/// API Response Format for quicksim/advancedsim API
/// Given as a GraphQL response
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StatCompareApiResponse {
    pub simulation_gear1: SimulationSummaryResponse,
    pub simulation_gear2: SimulationSummaryResponse,
}

impl From<(SimulationApiResponse, SimulationApiResponse)> for StatCompareApiResponse {
    fn from((gear1, gear2): (SimulationApiResponse, SimulationApiResponse)) -> Self {
        StatCompareApiResponse {
            simulation_gear1: gear1.simulation_data[0].simulation_summary.clone(),
            simulation_gear2: gear2.simulation_data[0].simulation_summary.clone(),
        }
    }
}

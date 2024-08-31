use crate::response::simulation_api_response::SimulationApiResponse;
use serde::Serialize;

/// API Response Format for quicksim/advancedsim API
/// Given as a GraphQL response
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GearCompareApiResponse {
    pub simulation_gear1: SimulationApiResponse,
    pub simulation_gear2: SimulationApiResponse,
}

impl From<(SimulationApiResponse, SimulationApiResponse)> for GearCompareApiResponse {
    fn from((gear1, gear2): (SimulationApiResponse, SimulationApiResponse)) -> Self {
        GearCompareApiResponse {
            simulation_gear1: gear1,
            simulation_gear2: gear2,
        }
    }
}

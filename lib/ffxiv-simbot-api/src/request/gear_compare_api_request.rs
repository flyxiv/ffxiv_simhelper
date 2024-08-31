use crate::request::simulation_api_request::SimulationApiRequest;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GearCompareApiRequest {
    pub gear1_request: SimulationApiRequest,
    pub gear2_request: SimulationApiRequest,
}

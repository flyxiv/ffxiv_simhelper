use ffxiv_simbot_combat_components::types::DpsType;
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BestPartnerApiResponse {
    pub contributed_dps: DpsType,
}

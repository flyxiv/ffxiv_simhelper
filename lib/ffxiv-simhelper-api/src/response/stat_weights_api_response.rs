use ffxiv_simhelper_combat_components::types::{DpsType, StatType};
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StatWeightsApiResponse {
    pub stat_name: String,
    pub augment_amount: StatType,
    pub dps: DpsType,
}

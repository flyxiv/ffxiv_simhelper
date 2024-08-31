use crate::request::simulation_api_request::{PlayerInfoRequest, StatsInfo};
use ffxiv_simbot_combat_components::types::{IdType, TimeType};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StatCompareApiRequest {}

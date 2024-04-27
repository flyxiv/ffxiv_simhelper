use ffxiv_simbot_db::stat::StatType;
use ffxiv_simbot_db::StatModifierType;
use serde::Deserialize;

/// Saves Constants needed for running the Engine.
#[derive(Deserialize, Clone)]
pub struct EngineConfig {
    pub(crate) equipment_json_file_name: String,
    pub(crate) jobs_json_file_name: String,
    pub(crate) food_json_file_name: String,
    pub(crate) clans_json_file_name: String,
}

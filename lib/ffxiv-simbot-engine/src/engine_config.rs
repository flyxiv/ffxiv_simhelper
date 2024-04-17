use ffxiv_simbot_lib_db::job::StatModifierType;
use ffxiv_simbot_lib_db::stat::StatType;
use serde::Deserialize;

/// Saves Constants needed for running the Engine.
#[derive(Deserialize)]
pub struct EngineConfig {
    pub(crate) equipment_json_file_name: String,
    pub(crate) jobs_json_file_name: String,
    pub(crate) food_json_file_name: String,
    pub(crate) clans_json_file_name: String,
    pub(crate) max_level_main_stat_modifier: StatModifierType,
    pub(crate) max_level_hp_modifier: StatModifierType,
    pub(crate) max_level_div: StatModifierType,
    pub(crate) hp_per_vitality_non_tank: StatModifierType,
    pub(crate) hp_per_vitality_tank: StatModifierType,
    pub(crate) max_level_base_vitality: StatType,
    pub(crate) max_level_base_piety: StatType,
    pub(crate) max_level_base_direct_hit: StatType,
    pub(crate) max_level_base_critical_hit: StatType,
    pub(crate) max_level_base_determination: StatType,
    pub(crate) max_level_base_skill_speed: StatType,
    pub(crate) max_level_base_spell_speed: StatType,
    pub(crate) max_level_base_tenacity: StatType,
}

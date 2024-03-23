use crate::ffxivcontext::FfxivContext;
use itertools::Itertools;

/// Saves Constants needed for running the Engine.
pub struct EngineConfig {
    equipment_json_file_name: String,
    jobs_json_file_name: String,
    food_json_file_name: String,
    clans_json_file_name: String,
    max_level_main_stat_modifier: f64,
    max_level_sub_stat_modifier: f64,
    max_level_hp_modifier: f64,
    max_level_div: f64,
    hp_per_vitality_non_tank: f64,
    hp_per_vitality_tank: f64,
}

/// The Main Engine of FFXIV Simbot.
/// Loads Needed Data, Simulates User DPS.
/// Singleton Entity. Only one instance of Engine is created.
pub struct Engine {
    config: EngineConfig,
}

impl Engine {
    /// Creates New Engine.
    /// 1. Makes EngineConfig by reading engine_config.yml in config directory
    /// 2. Loads FFXIV data by using the etro data file paths + constants in the EngineConfig
    /// 3. Starts Service.
    pub fn new() -> Self {
        // root = "ffxivsimbot" directory
        let root = project_root::get_project_root().unwrap();
        let config_dir = root.join("config");
        let config_file_path = config_dir.join("engine_config.yml");
        let engine_config =
            EngineConfig::load(config_file_path.to_str().unwrap()).expect("failed to get config");

        Engine {
            config: engine_config,
        }
    }
}

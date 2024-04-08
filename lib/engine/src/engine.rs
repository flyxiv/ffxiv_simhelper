use crate::engine_config::EngineConfig;
use crate::ffxivcontext::FfxivContext;
use crate::Result;
use ffxiv_simbot_lib_db::clan::ClanFactory;
use ffxiv_simbot_lib_db::equipment::EquipmentFactory;
use ffxiv_simbot_lib_db::food::FoodFactory;
use ffxiv_simbot_lib_db::job::JobFactory;
use itertools::Itertools;
use std::fs::File;

/// The Main Engine of FFXIV Simbot.
/// Loads Needed Data, Simulates User DPS.
/// Singleton Entity. Only one instance of Engine is created.
pub struct Engine {
    config: EngineConfig,
    context: FfxivContext,
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

        let data_dir = root.join("data");

        let jobs = JobFactory::new()
            .parse_jobs_json_file(&data_dir, &engine_config.jobs_json_file_name)
            .expect("failed to parse jobs json file");

        let equipments = EquipmentFactory::new()
            .parse_equipment_json_file(&data_dir, &engine_config.equipment_json_file_name)
            .expect("failed to parse equipment json file");

        let clans = ClanFactory::new()
            .parse_clans_json_file(&data_dir, &engine_config.clans_json_file_name)
            .expect("failed to parse clans json file");

        let foods = FoodFactory::new()
            .parse_food_json_file(&data_dir, &engine_config.food_json_file_name)
            .expect("failed to parse food json file");

        Engine {
            config: engine_config,
            context: FfxivContext {
                jobs,
                equipments,
                clans,
                foods,
            },
        }
    }
}

impl EngineConfig {
    fn load(file_path: &str) -> Result<Self> {
        let file = File::open(file_path)?;
        let config: EngineConfig = serde_yaml::from_reader(file)?;
        Ok(config)
    }
}

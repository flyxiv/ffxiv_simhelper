use crate::ffxivcontext::FfxivContext;
use ffxiv_simbot_lib_db::data::*;
use ffxiv_simbot_lib_db::database_manager::*;
use itertools::Itertools;

pub struct Engine {
    context: FfxivContext,
}

impl Engine {
    pub fn new() -> Self {
        let root = project_root::get_project_root().unwrap();
        let cmd_dir = root.join("cmd");
        let config_dir = cmd_dir.join("config");
        let config_path = config_dir.join("db_config.yml");
        let database_config =
            DatabaseConfig::load(config_path.to_str().unwrap()).expect("failed to get config");

        let mut database_manager = FfxivDatabaseManager::create(database_config);

        let equipment = database_manager
            .get_equipment()
            .expect("failed to get equipment");
        let races_db_data = database_manager.get_races().expect("failed to get stats");
        let jobclass = database_manager
            .get_jobclass()
            .expect("failed to get jobclass");

        let database_data: Vec<DatabaseData> = vec![
            DatabaseData::EquipmentDb(equipment),
            DatabaseData::RaceDb(races_db_data),
            DatabaseData::JobClassDb(jobclass),
        ];

        let engine_data = database_data
            .into_iter()
            .map(|data| make_data_table(data))
            .collect_vec();

        let context = FfxivContext::new(engine_data);

        Engine { context }
    }
}

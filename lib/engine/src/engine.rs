use crate::ffxivcontext::FfxivContext;
use itertools::Itertools;

pub struct Engine {
    context: FfxivContext,
}

impl Engine {
    pub fn new() -> Self {
        let root = project_root::get_project_root().unwrap();
        let cmd_dir = root.join("cmd");
        let config_dir = cmd_dir.join("config");
        let config_path = config_dir.join("engine_config.yml");
        let engine_config =
            EngineConfig::load(config_path.to_str().unwrap()).expect("failed to get config");

        Engine { context }
    }
}

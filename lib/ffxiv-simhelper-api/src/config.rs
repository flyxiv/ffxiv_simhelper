use crate::errors::Result;
use serde::Serialize;
use std::path::PathBuf;
use std::sync::Arc;

/// loads backend config file

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<FfxivSimhelperConfig>,
}

#[derive(serde::Deserialize, Serialize, Debug)]
pub struct FfxivSimhelperConfig {
    pub best_partner_request_count: usize,
    pub best_stats_simulation_count: usize,
}

impl FfxivSimhelperConfig {
    pub fn try_new(config_yaml_file_directory: PathBuf) -> Result<Self> {
        let config_file = std::fs::File::open(config_yaml_file_directory)?;
        let config: FfxivSimhelperConfig = serde_yaml::from_reader(config_file)?;
        Ok(config)
    }
}

impl From<FfxivSimhelperConfig> for AppState {
    fn from(config: FfxivSimhelperConfig) -> Self {
        AppState {
            config: Arc::new(config),
        }
    }
}

use crate::errors::Result;
use serde::Serialize;
use std::path::PathBuf;
use std::sync::Arc;

/// Shared state of the backend server that each handler process will share
#[derive(Clone)]
pub struct AppState {
    pub config: Arc<FfxivSimhelperConfig>,
}

/// Configuration for the FFXIV Simhelper Backend Server
/// Values are typically read from yaml files in the config/ directory.
#[derive(serde::Deserialize, Serialize, Debug)]
pub struct FfxivSimhelperConfig {
    /// The number of iterations to run for each partner in BestPartner API simulation
    pub best_partner_request_count: usize,

    /// The number of iterations to run for each augmented stat in BestStats API simulation
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

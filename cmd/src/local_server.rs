mod profile_beststats;
mod profile_dpsanalysis;

use ffxiv_simhelper_api::api_server::api_router::create_ffxiv_simhelper_service_router;
use ffxiv_simhelper_api::config::{AppState, FfxivSimhelperConfig};
use log::LevelFilter::Info;
use log::{info, Level, LevelFilter, Metadata, Record, SetLoggerError};
use std::path::PathBuf;

struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

static LOGGER: SimpleLogger = SimpleLogger;
pub fn init(log_level: LevelFilter) -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(log_level))
}

const PORT_NUMBER: i32 = 13406;

#[tokio::main]
async fn main() {
    info!("Loading Logger");
    init(Info).expect("failed to load logger");

    let config_directory = PathBuf::from("./config/backend_local_config.yml");

    let backend_config = FfxivSimhelperConfig::try_new(config_directory)
        .expect("Failed to load backend config file");

    let app_state = AppState::from(backend_config);

    let app = create_ffxiv_simhelper_service_router(app_state);

    info!("Started Server at port {}", PORT_NUMBER);
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", PORT_NUMBER))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

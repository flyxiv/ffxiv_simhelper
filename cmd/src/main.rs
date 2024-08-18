mod profile;

use ffxiv_simbot_api::api_server::api_router::create_ffxiv_simbot_service_router;
use log::LevelFilter::{Debug, Error, Info};
use log::{info, Level, LevelFilter, Metadata, Record, SetLoggerError};

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

    let app = create_ffxiv_simbot_service_router();

    info!("Started Server at port {}", PORT_NUMBER);
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", PORT_NUMBER))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

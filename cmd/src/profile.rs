use ffxiv_simbot_api::api_handler::simulate::quicksim;
use ffxiv_simbot_api::api_server::api_router::create_ffxiv_simbot_service_router;
use ffxiv_simbot_api::request::simulation_api_request::{
    PlayerInfoRequest, SimulationApiRequest, StatsInfo,
};
use ffxiv_simbot_combat_components::IdType;
use ffxiv_simbot_engine::engine::Engine;
use itertools::Itertools;
use log::LevelFilter::{Debug, Error, Info};
use log::{info, Level, LevelFilter, Metadata, Record, SetLoggerError};
use serde::__private::de::Content::String;

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

fn main() {
    let party_members = vec!["NIN", "BRD", "DRG", "SGE", "WHM", "PLD", "WAR", "BLM"];
    let party = party_members
        .iter()
        .enumerate()
        .map(|(i, job)| PlayerInfoRequest {
            player_id: i as IdType,
            jobAbbrev: job.to_string(),
            stats: StatsInfo {
                weapon_damage: 132,
                main_stat: 3300,
                critical_strike: 2560,
                direct_hit: 2500,
                determination: 2500,
                speed: 400,
                tenacity: 400,
            },
        })
        .collect_vec();

    let request = SimulationApiRequest {
        main_player_id: 0,
        combat_time_millisecond: 150000,
        party,
    };

    let response = quicksim(request);
}

use ffxiv_simbot_api::api_handler::simulate::quicksim;
use ffxiv_simbot_api::request::simulation_api_request::{PlayerInfoRequest, SimulationApiRequest};
use ffxiv_simbot_combat_components::live_objects::player::player_power::PlayerPower;
use ffxiv_simbot_combat_components::types::IdType;
use itertools::Itertools;
use log::{Level, LevelFilter, Metadata, Record, SetLoggerError};

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
            partner1_id: None,
            partner2_id: None,
            job_abbrev: "".to_string(),
            power: PlayerPower {
                auto_attack_delays: 3.0,
                critical_strike_rate: 3.0,
                critical_strike_damage: 3.0,
                direct_hit_rate: 3.0,
                determination_multiplier: 3.0,
                tenacity_multiplier: 3.0,
                speed_multiplier: 3.0,
                weapon_damage_multiplier: 3.0,
                main_stat_multiplier: 3.0,
                weapon_damage: 132,
                main_stat: 3300,
                critical_strike: 2560,
                direct_hit: 2500,
                determination: 2500,
                skill_speed: 0,

                tenacity: 400,
                spell_speed: 0,
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

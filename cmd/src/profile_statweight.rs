use ffxiv_simbot_api::api_handler::statweights::stat_weights;
use ffxiv_simbot_api::request::simulation_api_request::PlayerInfoRequest;
use ffxiv_simbot_api::request::stat_weights_api_request::StatWeightsApiRequest;
use ffxiv_simbot_combat_components::live_objects::player::player_power::PlayerPower;
use ffxiv_simbot_combat_components::types::PlayerIdType;
use itertools::Itertools;
use log::{Level, LevelFilter, Metadata, Record, SetLoggerError};
use tokio::time::Instant;

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
    let party_members = vec!["PLD", "NIN", "WAR", "WHM", "SGE", "DRG", "BRD", "PCT"];
    let party = party_members
        .iter()
        .enumerate()
        .map(|(i, job)| PlayerInfoRequest {
            player_id: i as PlayerIdType,
            partner1_id: None,
            partner2_id: None,
            job_abbrev: job.to_string(),
            power: PlayerPower {
                auto_attack_delays: 3.0,
                critical_strike_rate: 0.15,
                critical_strike_damage: 1.5,
                direct_hit_rate: 0.23,
                auto_direct_hit_increase: 0.1,
                determination_multiplier: 1.06,
                tenacity_multiplier: 1.060,
                speed_multiplier: 1.06,
                weapon_damage_multiplier: 1.60,
                main_stat_multiplier: 16.0,
                weapon_damage: 132,
                main_stat: 3300,
                critical_strike: 2560,
                direct_hit: 2500,
                determination: 2500,
                skill_speed: 2500,

                tenacity: 400,
                spell_speed: 2500,
            },
        })
        .collect_vec();

    let request = StatWeightsApiRequest {
        main_player_id: 0,
        combat_time_millisecond: 600000,
        party,
        stat_name: "DET".to_string(),
        augment_amount: 500,
        party_ilvl_adjustment: 100.0,
        use_pot: true,
    };

    let new = Instant::now();

    for _ in 0..8 {
        let response = stat_weights(request.clone());
        println!("{:?}", response.unwrap().dps);
    }

    println!("{}ms elapsed", new.elapsed().as_millis());
}

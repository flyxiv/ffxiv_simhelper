use axum::ServiceExt;
/*use ffxiv_simbot_combat_components::player::FfxivPlayer;
use ffxiv_simbot_combat_components::target::FfxivTarget;
use ffxiv_simbot_db::stat_calculator::CharacterPower;
use ffxiv_simbot_dps_simulator::simulator::{FfxivSimulationBoard, SimulationBoard}; */
use ffxiv_simbot_combat_components::live_objects::player::ffxiv_player::FfxivPlayer;
use ffxiv_simbot_combat_components::live_objects::target::ffxiv_target::FfxivTarget;
use ffxiv_simbot_db::stat_calculator::CharacterPower;
use ffxiv_simbot_dps_simulator::combat_simulator::ffxiv_simulation_board::FfxivSimulationBoard;
use ffxiv_simbot_dps_simulator::combat_simulator::SimulationBoard;
use ffxiv_simbot_dps_simulator::simulation_result::SimulationResponse;
use ffxiv_simbot_engine::engine::Engine;
use log::{Level, LevelFilter, Metadata, Record, SetLoggerError};
use serde::Deserialize;
use std::cell::RefCell;
use std::rc::Rc;

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
pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Info))
}

#[tokio::main]
async fn main() {
    let mut engine = Engine::new();
    init().expect("failed to load logger");

    let power = CharacterPower {
        critical_strike_rate: 1.271,
        critical_strike_damage: 1.621,
        direct_hit_rate: 1.177,
        determination_damage_multiplier: 1.112,
        tenacity_damage_multiplier: 1.0,
        speed_multiplier: 1.018,
        weapon_damage_multiplier: 1.76,
        main_stat_multiplier: 15.89,
    };

    let event_queue = Rc::new(RefCell::new(Default::default()));

    let mut party = vec![];
    party.push(Rc::new(RefCell::new(FfxivPlayer::new_ninja(
        0,
        power.clone(),
        &engine.context,
        event_queue.clone(),
    ))));

    for i in 1..2 {
        party.push(Rc::new(RefCell::new(FfxivPlayer::new_sage(
            i,
            power.clone(),
            &engine.context,
            event_queue.clone(),
        ))));
    }

    let target = Rc::new(RefCell::new(FfxivTarget {
        debuff_list: Rc::new(RefCell::new(Default::default())),
        event_queue: event_queue.clone(),
        combat_time_millisecond: 0,
    }));

    let mut simulator = FfxivSimulationBoard::new(target, event_queue.clone(), 100000);
    for player in party.into_iter() {
        simulator.register_player(player.clone());
    }
    simulator.run_simulation();

    /*    let result = SimulationResponse::from(simulator);

    for (player_id, player_result) in result.iter() {
        println!(
            "Player Id: {}, RDPS: {}, RDPS Earned: {}, RDPS Contributed: {}",
            player_id,
            player_result.raw_damage_total / 90,
            player_result.rdps_earned * 100.0,
            player_result.rdps_contributed * 100.0
        );
        for rotation_log in player_result.rotation_log.iter() {
            println!(
                "time(ms): {}, Skill Id: {}",
                rotation_log.casted_time_millisecond, rotation_log.skill_id,
            );
        }
    }*/

    /*    let app = Router::new()
        .route("/getequipment", get(get_equipment_handler))
        .with_state(engine);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap(); */
}

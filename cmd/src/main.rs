use ffxiv_simbot_combat_components::player::FfxivPlayer;
use ffxiv_simbot_combat_components::target::FfxivTarget;
use ffxiv_simbot_db::stat_calculator::CharacterPower;
use ffxiv_simbot_dps_simulator::simulator::{FfxivSimulationBoard, SimulationBoard};
use ffxiv_simbot_engine::engine::Engine;
use std::cell::{Ref, RefCell};
use std::rc::Rc;

fn main() {
    let mut engine = Engine::new();
    let power = CharacterPower {
        critical_strike_rate: 0.50,
        critical_strike_damage: 1.50,
        direct_hit_rate: 1.50,
        determination_damage_multiplier: 1.10,
        tenacity_damage_multiplier: 1.0,
        speed_multiplier: 1.09,
        weapon_damage_multiplier: 3.40,
        main_stat_multiplier: 15.30,
    };

    let mut party = vec![];
    for i in 0..8 {
        party.push(Rc::new(RefCell::new(FfxivPlayer::new_sage(
            i,
            CharacterPower {
                critical_strike_rate: 1.271,
                critical_strike_damage: 1.621,
                direct_hit_rate: 1.177,
                determination_damage_multiplier: 1.112,
                tenacity_damage_multiplier: 1.0,
                speed_multiplier: 1.018,
                weapon_damage_multiplier: 1.76,
                main_stat_multiplier: 15.89,
            },
            &engine.context,
        ))));
    }

    let target = Rc::new(RefCell::new(FfxivTarget {
        debuff_list: Rc::new(RefCell::new(vec![])),
        combat_time_millisecond: 0,
    }));

    let simulator = FfxivSimulationBoard::new(party, target);
    simulator.run_simulation(100000);

    println!("check");
}

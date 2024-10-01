use crate::errors::Result;
use crate::request::convert_to_simulation_board::create_player;
use crate::request::simulation_api_request::{PlayerInfoRequest, SimulationApiRequest};
use ffxiv_simhelper_combat_components::live_objects::player::role::job_abbrev_to_role;
use ffxiv_simhelper_combat_components::live_objects::target::ffxiv_target::FfxivTarget;
use ffxiv_simhelper_combat_components::types::{BuffIncreasePercentType, IncreaseType};
use ffxiv_simhelper_dps_simulator::combat_simulator::ffxiv_simulation_board::FfxivSimulationBoard;
use itertools::Itertools;
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

pub mod bestpartner;
pub mod gearcompare;
pub mod quicksim;
pub mod statweights;

fn create_simulation_board(request: SimulationApiRequest) -> Result<FfxivSimulationBoard> {
    let combat_time_millisecond = request.combat_time_millisecond;
    let main_player_id = request.main_player_id;

    let event_queue = Rc::new(RefCell::new(Default::default()));

    let target = Rc::new(RefCell::new(FfxivTarget {
        debuff_list: Rc::new(RefCell::new(Default::default())),
        event_queue: event_queue.clone(),
        combat_time_millisecond: 0,
    }));

    let mut simulation_board = FfxivSimulationBoard::new(
        main_player_id,
        target,
        event_queue.clone(),
        combat_time_millisecond,
        request.party_ilvl_adjustment,
    );

    let player_jobs = request
        .party
        .iter()
        .map(|player_info_request| {
            (
                player_info_request.player_id,
                player_info_request.job_abbrev.clone(),
            )
        })
        .collect_vec();

    for player_info_request in request.party {
        let player = create_player(
            player_info_request,
            &player_jobs,
            event_queue.clone(),
            request.use_pot,
        )?;

        simulation_board.register_player(Rc::new(RefCell::new(player)));
    }

    Ok(simulation_board)
}

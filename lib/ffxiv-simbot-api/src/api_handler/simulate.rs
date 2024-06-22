use crate::api_handler::get_composition_buff;
use crate::errors::Result;
use crate::request::convert_to_simulation_board::create_player;
use crate::request::simulation_api_request::{PlayerInfoRequest, SimulationApiRequest};
use crate::response::convert_simulation_result::create_response_from_simulation_result;
use crate::response::simulation_api_response::SimulationApiResponse;
use axum::Json;
use ffxiv_simbot_combat_components::live_objects::target::ffxiv_target::FfxivTarget;
use ffxiv_simbot_db::constants::{job_abbrev_to_role, FFXIV_STAT_MODIFIER};
use ffxiv_simbot_db::IncreaseType;
use ffxiv_simbot_dps_simulator::combat_simulator::ffxiv_simulation_board::FfxivSimulationBoard;
use ffxiv_simbot_dps_simulator::combat_simulator::SimulationBoard;
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

pub(crate) async fn simulate_api_handler(
    Json(request): Json<SimulationApiRequest>,
) -> Result<Json<SimulationApiResponse>> {
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
    );

    let composition_buff = get_composition_buff(&request.party);

    for player_info_request in request.party {
        let player = create_player(
            player_info_request,
            composition_buff,
            FFXIV_STAT_MODIFIER.clone(),
            event_queue.clone(),
        )?;

        simulation_board.register_player(Rc::new(RefCell::new(player)));
    }

    simulation_board.run_simulation();
    let simulation_result = simulation_board.create_simulation_result();

    Ok(Json(create_response_from_simulation_result(
        simulation_result,
    )))
}

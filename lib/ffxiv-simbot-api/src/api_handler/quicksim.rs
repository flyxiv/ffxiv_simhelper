use crate::api_handler::get_composition_buff_percent;
use crate::errors::Result;
use crate::request::convert_to_simulation_board::create_player;
use crate::request::simulation_api_request::SimulationApiRequest;
use crate::response::convert_simulation_result::create_response_from_simulation_result;
use crate::response::simulation_api_response::SimulationApiResponse;
use axum::Json;
use ffxiv_simbot_combat_components::live_objects::target::ffxiv_target::FfxivTarget;
use ffxiv_simbot_combat_components::types::BuffIncreasePercentType;
use ffxiv_simbot_dps_simulator::combat_simulator::ffxiv_simulation_board::FfxivSimulationBoard;
use ffxiv_simbot_dps_simulator::combat_simulator::SimulationBoard;
use itertools::Itertools;
use std::cell::RefCell;
use std::rc::Rc;

pub(crate) async fn simulate_api_handler(
    Json(request): Json<SimulationApiRequest>,
) -> Result<Json<SimulationApiResponse>> {
    Ok(Json(quicksim(request)?))
}

pub fn quicksim(request: SimulationApiRequest) -> Result<SimulationApiResponse> {
    let combat_time_millisecond = request.combat_time_millisecond;
    let main_player_id = request.main_player_id;
    let main_player_power = request.party[main_player_id as usize].power.clone();
    let main_player_job_abbrev = request.party[main_player_id as usize].job_abbrev.clone();

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

    let composition_buff_percent = get_composition_buff_percent(&request.party);
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
            composition_buff_percent as BuffIncreasePercentType,
            &player_jobs,
            event_queue.clone(),
        )?;

        simulation_board.register_player(Rc::new(RefCell::new(player)));
    }

    simulation_board.run_simulation();
    let simulation_result = simulation_board.create_simulation_result();

    Ok(create_response_from_simulation_result(
        simulation_result,
        main_player_power,
        main_player_job_abbrev,
    ))
}

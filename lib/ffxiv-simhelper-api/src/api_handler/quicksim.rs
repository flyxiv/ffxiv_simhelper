use crate::api_handler::create_simulation_board;
use crate::errors::Result;
use crate::request::simulation_api_request::SimulationApiRequest;
use crate::response::convert_simulation_result::create_response_from_simulation_result;
use crate::response::simulation_api_response::SimulationApiResponse;
use axum::Json;
use ffxiv_simhelper_dps_simulator::combat_simulator::SimulationBoard;

const NUMBER_OF_ITERATIONS_PER_REQUEST_QUICKSIM: usize = 2;

/// QuickSim API Request Handler
/// The most basic simulation of FFXIV SimHelper
/// 1) In-depth analysis of character's DPS
pub(crate) async fn quicksim_api_handler(
    Json(request): Json<SimulationApiRequest>,
) -> Result<Json<SimulationApiResponse>> {
    Ok(Json(quicksim(
        request,
        NUMBER_OF_ITERATIONS_PER_REQUEST_QUICKSIM,
    )?))
}

pub fn quicksim(
    request: SimulationApiRequest,
    number_of_iterations: usize,
) -> Result<SimulationApiResponse> {
    let main_player_id = request.main_player_id;
    let main_player_power = request.party[main_player_id as usize].power.clone();
    let main_player_job_abbrev = request.party[main_player_id as usize].job_abbrev.clone();

    let simulation_board = create_simulation_board(request.clone())?;
    simulation_board.run_simulation();
    let simulation_result = simulation_board.create_simulation_result();
    let mut response = create_response_from_simulation_result(
        simulation_result,
        main_player_power.clone(),
        main_player_job_abbrev.clone(),
    );

    for _ in 1..number_of_iterations {
        let simulation_board = create_simulation_board(request.clone())?;
        simulation_board.run_simulation();

        let simulation_result = simulation_board.create_simulation_result();
        let iteration_response = create_response_from_simulation_result(
            simulation_result,
            main_player_power.clone(),
            main_player_job_abbrev.clone(),
        );

        response.simulation_data[0]
            .simulation_summary
            .drain(&iteration_response.simulation_data[0].simulation_summary);
    }

    Ok(response)
}

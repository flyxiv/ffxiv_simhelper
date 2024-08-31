use crate::api_handler::create_simulation_board;
use crate::errors::Result;
use crate::request::simulation_api_request::SimulationApiRequest;
use crate::response::convert_simulation_result::create_response_from_simulation_result;
use crate::response::simulation_api_response::SimulationApiResponse;
use axum::Json;
use ffxiv_simbot_dps_simulator::combat_simulator::SimulationBoard;

pub(crate) async fn quicksim_api_handler(
    Json(request): Json<SimulationApiRequest>,
) -> Result<Json<SimulationApiResponse>> {
    Ok(Json(quicksim(request)?))
}

pub fn quicksim(request: SimulationApiRequest) -> Result<SimulationApiResponse> {
    let main_player_id = request.main_player_id;
    let main_player_power = request.party[main_player_id as usize].power.clone();
    let main_player_job_abbrev = request.party[main_player_id as usize].job_abbrev.clone();

    let simulation_board = create_simulation_board(request)?;
    simulation_board.run_simulation();

    let simulation_result = simulation_board.create_simulation_result();

    Ok(create_response_from_simulation_result(
        simulation_result,
        main_player_power,
        main_player_job_abbrev,
    ))
}

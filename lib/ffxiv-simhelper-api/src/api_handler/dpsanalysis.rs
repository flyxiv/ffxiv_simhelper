use crate::api_handler::create_simulation_board;
use crate::errors::Result;
use crate::request::simulation_api_request::SimulationApiRequest;
use crate::response::convert_simulation_result::create_response_from_simulation_result;
use crate::response::simulation_api_response::SimulationApiResponse;
use axum::Json;
use ffxiv_simhelper_dps_simulator::combat_simulator::SimulationBoard;

const NUMBER_OF_ITERATIONS_PER_REQUEST_DPS_ANALYSIS: usize = 2;

pub(crate) async fn dps_analysis_api_handler(
    Json(request): Json<SimulationApiRequest>,
) -> Result<Json<SimulationApiResponse>> {
    Ok(Json(dps_analysis(
        request,
        NUMBER_OF_ITERATIONS_PER_REQUEST_DPS_ANALYSIS,
    )?))
}

/// DpsAnalysis API Request Handler
/// The most basic and important simulation of FFXIV SimHelper. All other simulation APIs are technically a syntactic sugar
/// that uses this Simulation multiple times to get the desired result.
///
/// # Features
/// In-depth analysis of main player's DPS -
/// 1) Damage profile of each skill,
/// 2) main player's buffs given/taken,
/// 3) Sample rotation log.
///
/// ## App Usage
/// 1) Dps Analysis Button requests this API multiple times(so that they can record the progression bar).
/// 2) Server simulates the DPS number_of_iterations times for each request.
///     * This is done because React Apps can only send about 1000 requests at a time, so we had to split the number of requests sent and number of iterations for each request.
/// 3) App receives the simulation result, aggregates all the response they got and displays the DPS Analysis Results.
pub fn dps_analysis(
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
            .extend(&iteration_response.simulation_data[0].simulation_summary);
    }

    Ok(response)
}

use crate::api_handler::create_simulation_board;
use crate::errors::Result;
use crate::request::simulation_api_request::SimulationApiRequest;
use crate::response::convert_simulation_result::create_response_from_simulation_result;
use crate::response::simulation_api_response::SimulationApiResponse;
use axum::Json;
use ffxiv_simhelper_combat_components::types::DpsType;
use ffxiv_simhelper_dps_simulator::combat_simulator::SimulationBoard;

const NUMBER_OF_ITERATIONS_PER_REQUEST: usize = 4;

pub(crate) async fn quicksim_api_handler(
    Json(request): Json<SimulationApiRequest>,
) -> Result<Json<SimulationApiResponse>> {
    Ok(Json(quicksim(request)?))
}

pub fn quicksim(request: SimulationApiRequest) -> Result<SimulationApiResponse> {
    let main_player_id = request.main_player_id;
    let main_player_power = request.party[main_player_id as usize].power.clone();
    let main_player_job_abbrev = request.party[main_player_id as usize].job_abbrev.clone();

    let mut simulation_results = Vec::with_capacity(NUMBER_OF_ITERATIONS_PER_REQUEST);

    for _ in 0..NUMBER_OF_ITERATIONS_PER_REQUEST {
        let simulation_board = create_simulation_board(request.clone(), true)?;
        simulation_board.run_simulation();

        let simulation_result = simulation_board.create_simulation_result();
        simulation_results.push(create_response_from_simulation_result(
            simulation_result,
            main_player_power.clone(),
            main_player_job_abbrev.clone(),
        ));
    }

    let mut avg_pdps = 0.0;
    let mut avg_adps = 0.0;
    let mut avg_rdps = 0.0;
    let mut avg_edps = 0.0;

    simulation_results.iter().for_each(|simulation_result| {
        let simulation_summary = &simulation_result.simulation_data[0].simulation_summary;
        avg_pdps += simulation_summary.pdps;
        avg_adps += simulation_summary.adps;
        avg_rdps += simulation_summary.rdps;
        avg_edps += simulation_summary.edps;
    });

    avg_pdps = avg_pdps / NUMBER_OF_ITERATIONS_PER_REQUEST as DpsType;
    avg_adps = avg_adps / NUMBER_OF_ITERATIONS_PER_REQUEST as DpsType;
    avg_rdps = avg_rdps / NUMBER_OF_ITERATIONS_PER_REQUEST as DpsType;
    avg_edps = avg_edps / NUMBER_OF_ITERATIONS_PER_REQUEST as DpsType;

    simulation_results[0].simulation_data[0]
        .simulation_summary
        .pdps = avg_pdps;
    simulation_results[0].simulation_data[0]
        .simulation_summary
        .adps = avg_adps;
    simulation_results[0].simulation_data[0]
        .simulation_summary
        .rdps = avg_rdps;
    simulation_results[0].simulation_data[0]
        .simulation_summary
        .edps = avg_edps;

    Ok(simulation_results[0].clone())
}

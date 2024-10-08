use crate::api_handler::create_simulation_board;
use crate::errors::Result;
use crate::request::best_stats_api_request::BestStatsApiRequest;
use crate::request::simulation_api_request::SimulationApiRequest;
use crate::response::convert_simulation_result::create_response_from_simulation_result;
use crate::response::stat_weights_api_response::StatWeightsApiResponse;
use axum::Json;
use ffxiv_simhelper_combat_components::types::DpsType;
use ffxiv_simhelper_dps_simulator::combat_simulator::SimulationBoard;
use itertools::Itertools;

const BEST_STATS_SIMULATION_COUNT: usize = 2000;
const WANTED_CONTRIBUTION_PERCENTILE: f64 = 0.50;

pub(crate) async fn best_stats_api_handler(
    Json(request): Json<BestStatsApiRequest>,
) -> Result<Json<StatWeightsApiResponse>> {
    Ok(Json(stat_weights(request)?))
}

pub fn stat_weights(request: BestStatsApiRequest) -> Result<StatWeightsApiResponse> {
    let main_player_id = request.main_player_id;
    let main_player_power = request.party[main_player_id as usize].power.clone();
    let main_player_job_abbrev = request.party[main_player_id as usize].job_abbrev.clone();

    let mut dps_results: [i32; BEST_STATS_SIMULATION_COUNT] = [0; BEST_STATS_SIMULATION_COUNT];

    for simulation_idx in 0..BEST_STATS_SIMULATION_COUNT {
        let simulation_board = create_simulation_board(SimulationApiRequest::from(&request))?;
        simulation_board.run_simulation();

        let simulation_result = simulation_board.create_simulation_result();
        let response = create_response_from_simulation_result(
            simulation_result,
            main_player_power.clone(),
            main_player_job_abbrev.clone(),
        );

        let dps_result = response.simulation_data[main_player_id as usize]
            .simulation_summary
            .rdps[0];
        dps_results[simulation_idx] = dps_result as i32;
    }

    Ok(StatWeightsApiResponse {
        stat_name: request.stat_name.clone(),
        augment_amount: request.augment_amount,
        dps: dps_results
            .into_iter()
            .sorted()
            .nth((WANTED_CONTRIBUTION_PERCENTILE * BEST_STATS_SIMULATION_COUNT as f64) as usize)
            .unwrap() as DpsType,
    })
}

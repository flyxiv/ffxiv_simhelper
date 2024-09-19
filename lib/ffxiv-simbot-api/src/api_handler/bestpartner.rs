use crate::api_handler::create_simulation_board;
use crate::errors::Result;
use crate::request::simulation_api_request::SimulationApiRequest;
use crate::response::best_partner_api_response::BestPartnerApiResponse;
use crate::response::convert_simulation_result::create_response_from_simulation_result;
use axum::Json;
use ffxiv_simbot_combat_components::types::DpsType;
use ffxiv_simbot_dps_simulator::combat_simulator::SimulationBoard;
use itertools::Itertools;

const BEST_PARTNER_SIMULATION_COUNT: usize = 100;
const WANTED_CONTRIBUTION_PERCENTILE: f64 = 0.95;

pub(crate) async fn best_partner_api_handler(
    Json(request): Json<SimulationApiRequest>,
) -> Result<Json<BestPartnerApiResponse>> {
    Ok(Json(best_partner(request)?))
}

const ARRAY_REPEAT_VALUE: Vec<i32> = Vec::new();

pub fn best_partner(request: SimulationApiRequest) -> Result<BestPartnerApiResponse> {
    let main_player_id = request.main_player_id;
    let main_player_power = request.party[main_player_id as usize].power.clone();
    let main_player_job_abbrev = request.party[main_player_id as usize].job_abbrev.clone();

    /// first contains total, after that contains contribution at every burst phase
    let mut partner_contributions: [Vec<i32>; BEST_PARTNER_SIMULATION_COUNT] =
        [ARRAY_REPEAT_VALUE; BEST_PARTNER_SIMULATION_COUNT];

    for simulation_idx in 0..BEST_PARTNER_SIMULATION_COUNT {
        let simulation_board = create_simulation_board(request.clone(), false)?;
        simulation_board.run_simulation();

        let simulation_result = simulation_board.create_simulation_result();
        let response = create_response_from_simulation_result(
            simulation_result,
            main_player_power.clone(),
            main_player_job_abbrev.clone(),
        );

        let partner_contribution = response.simulation_data[1].simulation_summary.edps
            - response.simulation_data[1].simulation_summary.rdps;
        let partner_contribution_each_burst = response.simulation_data[1]
            .party_burst_contribution_table
            .iter()
            .map(|response| response.contributed_rdps as i32)
            .collect_vec();

        partner_contributions[simulation_idx].push(partner_contribution as i32);
        partner_contributions[simulation_idx].extend(partner_contribution_each_burst);
    }

    Ok(BestPartnerApiResponse {
        contributed_dps: partner_contributions
            .into_iter()
            .sorted()
            .nth((WANTED_CONTRIBUTION_PERCENTILE * BEST_PARTNER_SIMULATION_COUNT as f64) as usize)
            .unwrap(),
        partner_job_abbrev: request.party[1].job_abbrev.clone(),
    })
}

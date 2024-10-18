use crate::api_handler::create_simulation_board;
use crate::config::AppState;
use crate::errors::Result;
use crate::request::simulation_api_request::SimulationApiRequest;
use crate::response::best_partner_api_response::BestPartnerApiResponse;
use crate::response::convert_simulation_result::create_response_from_simulation_result;
use axum::extract::State;
use axum::Json;
use ffxiv_simhelper_dps_simulator::combat_simulator::SimulationBoard;
use itertools::Itertools;

const WANTED_CONTRIBUTION_PERCENTILE: f64 = 0.50;

pub(crate) async fn best_partner_api_handler(
    State(app_state): State<AppState>,
    Json(request): Json<SimulationApiRequest>,
) -> Result<Json<BestPartnerApiResponse>> {
    Ok(Json(best_partner(request, app_state)?))
}

pub fn best_partner(
    request: SimulationApiRequest,
    app_state: AppState,
) -> Result<BestPartnerApiResponse> {
    let best_partner_simulation_count = app_state.config.best_partner_request_count;
    let main_player_id = request.main_player_id;
    let main_player_power = request.party[main_player_id as usize].power.clone();
    let main_player_job_abbrev = request.party[main_player_id as usize].job_abbrev.clone();

    // first contains total, after that contains contribution at every burst phase
    let mut partner_contributions: Vec<Vec<i32>> =
        Vec::with_capacity(best_partner_simulation_count);

    for _ in 0..best_partner_simulation_count {
        let mut contribution =
            Vec::with_capacity((request.combat_time_millisecond as usize / 120000) + 1);

        let simulation_board = create_simulation_board(request.clone())?;
        simulation_board.run_simulation();

        let simulation_result = simulation_board.create_simulation_result();
        let response = create_response_from_simulation_result(
            simulation_result,
            main_player_power.clone(),
            main_player_job_abbrev.clone(),
        );

        let partner_contribution_each_burst = response.simulation_data[1]
            .party_burst_contribution_table
            .iter()
            .map(|response| response.contributed_damage)
            .collect_vec();

        contribution.push(partner_contribution_each_burst.iter().sum());
        contribution.extend(partner_contribution_each_burst);

        partner_contributions.push(contribution);
    }

    let mut max_len = 0;

    partner_contributions.iter().for_each(|contributions| {
        if contributions.len() > max_len {
            max_len = contributions.len();
        }
    });

    let burst_count = max_len;

    let mut contributed_damage: Vec<i32> = Vec::with_capacity(burst_count);

    for burst_idx in 0..burst_count {
        let burst_contributions: Vec<i32> = partner_contributions
            .iter()
            .map(|contributions| {
                if contributions.len() > burst_idx {
                    contributions[burst_idx]
                } else {
                    0
                }
            })
            .collect();

        let burst_contribution_top_nth_percentile = burst_contributions
            .into_iter()
            .sorted()
            .nth((WANTED_CONTRIBUTION_PERCENTILE * best_partner_simulation_count as f64) as usize)
            .unwrap();

        contributed_damage.push(burst_contribution_top_nth_percentile);
    }

    Ok(BestPartnerApiResponse {
        contributed_damage,
        partner_job_abbrev: request.party[1].job_abbrev.clone(),
    })
}

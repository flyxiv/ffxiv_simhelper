use crate::api_handler::create_simulation_board;
use crate::errors::Result;
use crate::request::gear_compare_api_request::GearCompareApiRequest;
use crate::response::convert_simulation_result::create_response_from_simulation_result;
use crate::response::stat_compare_response::GearCompareApiResponse;
use axum::Json;
use ffxiv_simbot_dps_simulator::combat_simulator::SimulationBoard;

pub(crate) async fn gear_compare_api_handler(
    Json(request): Json<GearCompareApiRequest>,
) -> Result<Json<GearCompareApiResponse>> {
    let main_player_id = request.gear1_request.main_player_id;
    let power1 = request.gear1_request.party[main_player_id as usize]
        .power
        .clone();
    let power2 = request.gear2_request.party[main_player_id as usize]
        .power
        .clone();
    let main_player_job_abbrev = request.gear1_request.party[main_player_id as usize]
        .job_abbrev
        .clone();

    let simulation_board1 = create_simulation_board(request.gear1_request)?;
    let simulation_board2 = create_simulation_board(request.gear2_request)?;

    simulation_board1.run_simulation();
    simulation_board2.run_simulation();

    let simulation_result1 = simulation_board1.create_simulation_result();
    let simulation_result2 = simulation_board2.create_simulation_result();

    let simulation_response1 = create_response_from_simulation_result(
        simulation_result1,
        power1,
        main_player_job_abbrev.clone(),
    );
    let simulation_response2 =
        create_response_from_simulation_result(simulation_result2, power2, main_player_job_abbrev);

    Ok(Json(GearCompareApiResponse::from((
        simulation_response1,
        simulation_response2,
    ))))
}

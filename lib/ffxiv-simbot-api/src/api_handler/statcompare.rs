use crate::api_handler::get_composition_buff_percent;
use crate::errors::Result;
use crate::request::convert_to_simulation_board::create_player;
use crate::request::simulation_api_request::{PlayerInfoRequest, StatsInfo};
use crate::request::stat_compare_api_request::StatCompareApiRequest;
use crate::response::convert_simulation_result::create_response_from_simulation_result;
use crate::response::stat_compare_response::StatCompareApiResponse;
use axum::Json;
use ffxiv_simbot_combat_components::live_objects::target::ffxiv_target::FfxivTarget;
use ffxiv_simbot_dps_simulator::combat_simulator::ffxiv_simulation_board::FfxivSimulationBoard;
use ffxiv_simbot_dps_simulator::combat_simulator::SimulationBoard;
use itertools::Itertools;
use std::cell::RefCell;
use std::rc::Rc;

pub(crate) async fn stat_compare_api_handler(
    Json(request): Json<StatCompareApiRequest>,
) -> Result<Json<StatCompareApiResponse>> {
    let simulation_board1 =
        create_stat_compare_simulation_board(&request, request.main_player_stat1)?;

    let simulation_board2 =
        create_stat_compare_simulation_board(&request, request.main_player_stat2)?;

    simulation_board1.run_simulation();
    simulation_board2.run_simulation();

    let simulation_result1 = simulation_board1.create_simulation_result();
    let simulation_result2 = simulation_board2.create_simulation_result();

    let simulation_response1 = create_response_from_simulation_result(simulation_result1);
    let simulation_response2 = create_response_from_simulation_result(simulation_result2);

    Ok(Json(StatCompareApiResponse::from((
        simulation_response1,
        simulation_response2,
    ))))
}

fn create_stat_compare_simulation_board(
    request: &StatCompareApiRequest,
    main_player_stats: StatsInfo,
) -> Result<FfxivSimulationBoard> {
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

    let composition_buff_percent = get_composition_buff_percent(&request.party);
    let mut party_jobs = vec![(main_player_id, request.main_player_job.clone())];
    party_jobs.extend(
        request
            .party
            .iter()
            .map(|player_info_request| {
                (
                    player_info_request.player_id,
                    player_info_request.job_abbrev.clone(),
                )
            })
            .collect_vec(),
    );

    let main_player = create_player(
        PlayerInfoRequest {
            player_id: main_player_id,
            partner1_id: request.main_player_partner1_id,
            partner2_id: request.main_player_partner2_id,
            job_abbrev: request.main_player_job.clone(),
            stats: main_player_stats,
            power: request.party[0].power.clone(),
        },
        composition_buff_percent,
        &party_jobs,
        event_queue.clone(),
    )?;

    simulation_board.register_player(Rc::new(RefCell::new(main_player)));

    for player_info_request in &request.party {
        let player = create_player(
            player_info_request.clone(),
            composition_buff_percent,
            &party_jobs,
            event_queue.clone(),
        )?;

        simulation_board.register_player(Rc::new(RefCell::new(player)));
    }

    return Ok(simulation_board);
}

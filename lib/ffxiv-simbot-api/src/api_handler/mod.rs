use crate::request::simulation_api_request::{PlayerInfoRequest, SimulationApiRequest};
use crate::response::simulation_api_response::SimulationApiResponse;
use axum::extract::State;
use axum::Json;
use ffxiv_simbot_db::constants::job_abbrev_to_role;
use ffxiv_simbot_db::IncreaseType;
use ffxiv_simbot_engine::engine::Engine;
use std::collections::HashSet;

pub mod simulate;
pub(crate) mod statcompare;

fn get_composition_buff(party: &Vec<PlayerInfoRequest>) -> IncreaseType {
    if party.len() == 1 {
        return 0;
    }

    let mut roles = HashSet::new();

    for player_info_request in party {
        roles.insert(job_abbrev_to_role(&player_info_request.job));
    }

    return roles.len() as IncreaseType;
}

use crate::request::simulation_api_request::PlayerInfoRequest;
use crate::role::job_abbrev_to_role;
use ffxiv_simbot_combat_components::types::IncreaseType;
use std::collections::HashSet;

pub mod simulate;
pub(crate) mod statcompare;

fn get_composition_buff(party: &Vec<PlayerInfoRequest>) -> IncreaseType {
    if party.len() == 1 {
        return 0;
    }

    let mut roles = HashSet::new();

    for player_info_request in party {
        roles.insert(job_abbrev_to_role(&player_info_request.jobAbbrev));
    }

    return roles.len();
}

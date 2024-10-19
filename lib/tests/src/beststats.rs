#[cfg(test)]
mod tests {
    use crate::{create_best_stats_request, create_config_for_simulation_test, create_party_info};
    use ffxiv_simhelper_api::api_handler::beststats::best_stats;

    #[test]
    fn best_stats_basic_test() {
        // Best Stats inputs are all manufactured in the app, so it is hard to validity check from the backend. We just do one simple simulation
        // to confirm request doesn't crash

        // given: a player with decent stats
        let party = vec!["NIN", "DRK", "WHM", "AST", "BRD", "PCT", "WAR", "VPR"];
        let party_info = create_party_info(&party);

        let request = create_best_stats_request(0, party_info, "crit".to_string(), 100);

        // when:
        let app_state = create_config_for_simulation_test(50, 50);
        let response = best_stats(request, app_state);

        assert!(response.is_ok());
    }
}

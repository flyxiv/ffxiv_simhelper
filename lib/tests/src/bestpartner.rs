#[cfg(test)]
mod tests {
    use std::vec;

    use crate::{
        assert_test_value_is_in_range, create_config_for_simulation_test, create_party_info,
        create_simulation_api_request_for_testing,
    };
    use ffxiv_simhelper_api::api_handler::bestpartner::best_partner;

    #[test]
    fn test_best_partner_consistency_in_different_combat_times() {
        let test_iteration_count: i32 = 100;

        // https://github.com/flyxiv/ffxiv_simhelper_public/issues/33
        // Viper's opener contribution differs when there's more buff timers.
        // ex) its contribution proportion was 67% for 30s simulation, but its opener contribution proportion
        // rises to 80% for 150s simulation.
        // check that opener contribution is similar even when burst times increase.

        let party_members = vec!["NIN", "VPR"];
        let party = create_party_info(&party_members);

        // Given: Only opener encounters and opener + 2min burst encounters

        // 30 second fight. Only opener
        let request_30_second_fight =
            create_simulation_api_request_for_testing(30000, party.clone());

        // 110 second fight. Only Opener
        let request_110_second_fight =
            create_simulation_api_request_for_testing(110000, party.clone());

        // 150 second fight. Opener + 2min burst
        let request_150_second_fight =
            create_simulation_api_request_for_testing(150000, party.clone());

        // then: the opener contribution should be the same no matter how long the fight is.
        let app_state = create_config_for_simulation_test(1, 1);

        let mut request_30_opener_contribution_of_vpr = vec![];

        for _ in 0..test_iteration_count {
            let response =
                best_partner(request_30_second_fight.clone(), app_state.clone()).unwrap();
            assert_eq!(response.contributed_damage.len(), 2, "30 second encounter must have total contribution + opener contribution = 2 contribution data, but it has {} contributions", response.contributed_damage.len());
            request_30_opener_contribution_of_vpr.push(response.contributed_damage[1])
        }

        let request_30_mean_opener_contribution = request_30_opener_contribution_of_vpr
            .into_iter()
            .sum::<i32>()
            / test_iteration_count;

        println!(
            "30 second fight opener contribution mean: {}",
            request_30_mean_opener_contribution
        );

        let lower_bound = request_30_mean_opener_contribution as f64 * 0.95;
        let upper_bound = request_30_mean_opener_contribution as f64 * 1.05;

        let mut request_110_opener_contribution_of_vpr = vec![];
        for _ in 0..test_iteration_count {
            let response =
                best_partner(request_110_second_fight.clone(), app_state.clone()).unwrap();
            assert_eq!(response.contributed_damage.len(), 2, "110 second encounter must have total contribution + opener contribution = 2 contribution data, but it has {} contributions", response.contributed_damage.len());
            request_110_opener_contribution_of_vpr.push(response.contributed_damage[1])
        }

        let request_110_mean_opener_contribution = request_110_opener_contribution_of_vpr
            .into_iter()
            .sum::<i32>()
            / test_iteration_count;
        println!(
            "110 second fight opener contribution mean: {}",
            request_110_mean_opener_contribution
        );

        assert_test_value_is_in_range(
            request_110_mean_opener_contribution as f64,
            lower_bound,
            upper_bound,
        );

        // 150 second fight. Opener + 2min burst
        let mut request_150_opener_contribution_of_vpr = vec![];
        for _ in 0..test_iteration_count {
            let response =
                best_partner(request_150_second_fight.clone(), app_state.clone()).unwrap();
            assert_eq!(response.contributed_damage.len(), 3, "150 second encounter must have total contribution + opener contribution + 2min contribution = 3 contribution data, but it has {} contributions", response.contributed_damage.len());
            request_150_opener_contribution_of_vpr.push(response.contributed_damage[1])
        }

        let request_150_mean_opener_contribution = request_150_opener_contribution_of_vpr
            .into_iter()
            .sum::<i32>()
            / test_iteration_count;
        println!(
            "150 second fight opener contribution mean: {}",
            request_150_mean_opener_contribution
        );

        assert_test_value_is_in_range(
            request_150_mean_opener_contribution as f64,
            lower_bound,
            upper_bound,
        );
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        assert_test_value_is_in_range, create_party_info, create_simulation_api_request_for_testing,
    };
    use ffxiv_simhelper_api::api_handler::dpsanalysis::dps_analysis;
    use ffxiv_simhelper_combat_components::{
        combat_resources::ffxiv_combat_resources::ALL_FFXIV_COMBAT_JOBS, types::MultiplierType,
    };
    use itertools::Itertools;

    #[test]
    fn dps_analysis_validity_test() {
        // Given: two exactly same player powers
        let iteration_count = 100;
        let difference_threshold = 0.2;

        let party_members = vec!["WAR", "DRK", "WHM", "AST", "BRD", "PCT", "NIN", "VPR"];
        let party1 = create_party_info(&party_members);
        let party2 = party1.clone();

        let request1 = create_simulation_api_request_for_testing(180000, party1);
        let request2 = create_simulation_api_request_for_testing(180000, party2);

        let response1 = dps_analysis(request1, iteration_count).unwrap();
        let response2 = dps_analysis(request2, iteration_count).unwrap();

        let request1_median_dps = response1.simulation_data[0]
            .simulation_summary
            .pdps
            .iter()
            .map(|dps| *dps as usize)
            .sorted()
            .nth(iteration_count / 2)
            .unwrap() as f64;

        let request2_median_dps = response2.simulation_data[0]
            .simulation_summary
            .pdps
            .iter()
            .map(|dps| *dps as usize)
            .sorted()
            .nth(iteration_count / 2)
            .unwrap() as f64;

        // Then: the result of the two simulation should be within <0.2% difference
        let upper_bound = request1_median_dps * (1.0 + difference_threshold);
        let lower_bound = request1_median_dps * (1.0 - difference_threshold);

        assert_test_value_is_in_range(request2_median_dps, lower_bound, upper_bound);
    }

    #[test]
    fn test_dps_analysis_consistency_in_different_combat_times() {
        // Connection to test_best_partner_consistency_in_different_combat_times.
        // best_partner inside uses dps analysis to calculate the best partner data,
        // so we check if it is the analysis that's causing the inconsistency in contribution for
        // combat periods of different lengths.

        let test_iteration_count = 100;

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

        let mut request_30_opener_contribution_of_vpr = vec![];

        for _ in 0..test_iteration_count {
            let response = dps_analysis(request_30_second_fight.clone(), 1).unwrap();
            let contribution_table = &response.simulation_data[1].party_burst_contribution_table;
            assert_eq!(contribution_table.len(), 1, "30 second encounter must have opener contribution = 1 contribution data, but it has {} contributions", contribution_table.len());
            request_30_opener_contribution_of_vpr.push(contribution_table[0].contributed_damage);
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
            let response = dps_analysis(request_110_second_fight.clone(), 1).unwrap();
            let contribution_table = &response.simulation_data[1].party_burst_contribution_table;
            assert_eq!(contribution_table.len(), 1, "110 second encounter must have opener contribution = 1 contribution data, but it has {} contributions", contribution_table.len());
            request_110_opener_contribution_of_vpr.push(contribution_table[0].contributed_damage);
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
            let response = dps_analysis(request_150_second_fight.clone(), 1).unwrap();
            let contribution_table = &response.simulation_data[1].party_burst_contribution_table;
            assert_eq!(contribution_table.len(), 2, "150 second encounter must have opener contribution + 2min burst = 2 contribution data, but it has {} contributions", contribution_table.len());
            request_150_opener_contribution_of_vpr.push(contribution_table[0].contributed_damage);
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

    #[test]
    fn test_pot_option() {
        // https://github.com/flyxiv/ffxiv_simhelper_public/issues/41
        // DPS with and without pots should have at least a 1.2% difference.
        let iteration_count = 100;

        let jobs = ALL_FFXIV_COMBAT_JOBS;
        let combat_time_millisecond = 390000;

        let minimum_ratio_bound = 1.2;

        for job in jobs {
            let party_members = vec![job];
            let party = create_party_info(&party_members);

            let mut request_with_pot =
                create_simulation_api_request_for_testing(combat_time_millisecond, party);
            let mut request_without_pot = request_with_pot.clone();

            request_with_pot.use_pot = true;
            request_without_pot.use_pot = false;

            let mut dps_with_pot = Vec::with_capacity(iteration_count);
            let mut dps_without_pot = Vec::with_capacity(iteration_count);

            for _ in 0..iteration_count {
                let response_with_pot = dps_analysis(request_with_pot.clone(), 1).unwrap();
                let response_without_pot = dps_analysis(request_without_pot.clone(), 1).unwrap();

                dps_with_pot
                    .push(response_with_pot.simulation_data[0].simulation_summary.pdps[0] as usize);
                dps_without_pot.push(
                    response_without_pot.simulation_data[0]
                        .simulation_summary
                        .pdps[0] as usize,
                );
            }

            let dps_with_pot_median = dps_with_pot
                .iter()
                .sorted()
                .nth(iteration_count / 2)
                .unwrap();
            let dps_without_pot_median = dps_without_pot
                .iter()
                .sorted()
                .nth(iteration_count / 2)
                .unwrap();

            assert!(
                (*dps_with_pot_median as MultiplierType)
                    > (*dps_without_pot_median as MultiplierType
                        * (1.0 + minimum_ratio_bound / 100.0)),
                "job {}'s median dps with pot {} is not at least {}% higher than without pot {}",
                job,
                dps_with_pot_median,
                minimum_ratio_bound,
                dps_without_pot_median
            );
        }
    }

    #[test]
    fn test_various_gcds() {
        // Job's dps rotation changes on different GCDs.
        // Test all jobs in serveral GCDs once to assure the logic is safe.
        let jobs = ALL_FFXIV_COMBAT_JOBS;
        let longest_possible_combat_time_millisecond = 600000;

        let testing_gcd_count = 90;

        for job in jobs {
            let party_members = vec![job];
            let party = create_party_info(&party_members);

            for speed_multiplier_increase in 0..testing_gcd_count {
                let mut party_with_testing_gcd = party.clone();
                party_with_testing_gcd[0].power.speed_multiplier =
                    1.0 + 0.002 * speed_multiplier_increase as f64;

                let request = create_simulation_api_request_for_testing(
                    longest_possible_combat_time_millisecond,
                    party_with_testing_gcd,
                );

                let response = dps_analysis(request, 1);

                assert!(
                    response.is_ok(),
                    "job {}'s simulation returns error at speed multiplier {}",
                    job,
                    speed_multiplier_increase
                );
            }
        }
    }
}

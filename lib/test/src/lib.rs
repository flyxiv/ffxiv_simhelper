use std::fmt::Display;
use std::sync::Arc;
use itertools::Itertools;
use ffxiv_simhelper_api::config::{AppState, FfxivSimhelperConfig};
use ffxiv_simhelper_api::request::simulation_api_request::{PlayerInfoRequest, SimulationApiRequest};
use ffxiv_simhelper_combat_components::live_objects::player::player_power::PlayerPower;
use ffxiv_simhelper_combat_components::types::{PlayerIdType, TimeType};

/// Integration/End-to-End test module
/// Unit Tests are written at the file where the function unit testing is defined.

mod bestpartner;
mod dpsanalysis;

#[cfg(test)]
fn create_party_info(party_members: &[&str]) -> Vec<PlayerInfoRequest> {
    party_members
        .iter()
        .enumerate()
        .map(|(i, job)| PlayerInfoRequest {
            player_id: i as PlayerIdType,
            partner1_id: None,
            partner2_id: None,
            job_abbrev: job.to_string(),
            power: PlayerPower {
                auto_attack_delays: 3.0,
                critical_strike_rate: 0.15,
                critical_strike_damage: 1.5,
                direct_hit_rate: 0.23,
                auto_direct_hit_increase: 0.1,
                determination_multiplier: 1.06,
                tenacity_multiplier: 1.060,
                speed_multiplier: 1.06,
                weapon_damage_multiplier: 1.60,
                main_stat_multiplier: 16.0,
                weapon_damage: 132,
                main_stat: 3300,
                critical_strike: 2560,
                direct_hit: 2500,
                determination: 2500,
                skill_speed: 2500,

                tenacity: 400,
                spell_speed: 2500,
            },
        })
        .collect_vec()
}

#[cfg(test)]
fn create_config_for_simulation_test(best_stats_simulation_count: usize, best_partner_request_count: usize) -> AppState {
    let config = FfxivSimhelperConfig{
        best_partner_request_count,
        best_stats_simulation_count,
    };

    AppState {
        config: Arc::new(config)
    }
}

#[cfg(test)]
fn create_simulation_api_request_for_testing(combat_time_millisecond: TimeType, party: Vec<PlayerInfoRequest>) -> SimulationApiRequest {
    SimulationApiRequest {
        main_player_id: 0,
        combat_time_millisecond,
        party,
        party_ilvl_adjustment: 100.0,
        use_pot: true,
    }
}

#[cfg(test)]
fn assert_test_value_is_in_range<T>(value: T, lower_bound: T, upper_bound: T)
where T: PartialOrd + Display
{
    assert!(value >= lower_bound, "Value {} is lower than lower bound {}", value, lower_bound);
    assert!(value <= upper_bound, "Value {} is higher than upper bound {}", value, upper_bound);
}
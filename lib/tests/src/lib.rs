use ffxiv_simhelper_api::config::{AppState, FfxivSimhelperConfig};
use ffxiv_simhelper_api::request::best_stats_api_request::BestStatsApiRequest;
use ffxiv_simhelper_api::request::gear_compare_api_request::GearCompareApiRequest;
use ffxiv_simhelper_api::request::simulation_api_request::{
    PlayerInfoRequest, SimulationApiRequest,
};
use ffxiv_simhelper_combat_components::live_objects::player::player_power::PlayerPower;
use ffxiv_simhelper_combat_components::types::{PlayerIdType, TimeType};
use itertools::Itertools;
/// Integration/End-to-End test module
/// Unit Tests are written at the file where the function unit testing is defined.
use std::fmt::Display;
use std::sync::Arc;

mod bestpartner;
mod beststats;
mod dpsanalysis;

#[cfg(test)]
fn create_party_info(party_members: &[&str]) -> Vec<PlayerInfoRequest> {
    use ffxiv_simhelper_combat_components::live_objects::player::role::{job_abbrev_to_role, Role};

    let normal_power = PlayerPower {
        auto_attack_delays: 3.0,
        critical_strike_rate: 0.244,
        critical_strike_damage: 1.594,
        direct_hit_rate: 0.338,
        auto_direct_hit_increase: 0.067,
        determination_multiplier: 1.086,
        tenacity_multiplier: 1.00,
        speed_multiplier: 1.00,
        weapon_damage_multiplier: 1.96,
        main_stat_multiplier: 24.92,
        weapon_damage: 132,
        main_stat: 4882,
        piety: 440,
        critical_strike: 2560,
        direct_hit: 2500,
        determination: 2500,
        skill_speed: 2500,
        gcd: 2.5,

        tenacity: 400,
        spell_speed: 2500,
    };

    let tank_power = PlayerPower {
        auto_attack_delays: 3.0,
        critical_strike_rate: 0.248,
        critical_strike_damage: 1.598,
        direct_hit_rate: 0.207,
        auto_direct_hit_increase: 0.067,
        determination_multiplier: 1.094,
        tenacity_multiplier: 1.018,
        speed_multiplier: 1.00,
        weapon_damage_multiplier: 1.92,
        main_stat_multiplier: 20.00,
        weapon_damage: 132,
        main_stat: 4882,
        piety: 440,
        critical_strike: 2560,
        direct_hit: 2500,
        determination: 2500,
        skill_speed: 2500,
        gcd: 2.5,

        tenacity: 400,
        spell_speed: 2500,
    };

    party_members
        .iter()
        .enumerate()
        .map(|(i, job)| {
            let job_abbrev = String::from(job.to_uppercase());
            let istank = job_abbrev_to_role(&job_abbrev) == Role::Tank;

            PlayerInfoRequest {
                player_id: i as PlayerIdType,
                partner1_id: None,
                partner2_id: None,
                job_abbrev: job.to_string(),
                power: if istank {
                    tank_power.clone()
                } else {
                    normal_power.clone()
                },
            }
        })
        .collect_vec()
}

#[cfg(test)]
fn create_config_for_simulation_test(
    best_stats_simulation_count: usize,
    best_partner_request_count: usize,
) -> AppState {
    let config = FfxivSimhelperConfig {
        best_partner_request_count,
        best_stats_simulation_count,
    };

    AppState {
        config: Arc::new(config),
    }
}

#[cfg(test)]
fn create_simulation_api_request_for_testing(
    combat_time_millisecond: TimeType,
    party: Vec<PlayerInfoRequest>,
) -> SimulationApiRequest {
    SimulationApiRequest {
        main_player_id: 0,
        combat_time_millisecond,
        party,
        party_ilvl_adjustment: 100.0,
        use_pot: true,
    }
}

#[cfg(test)]
#[allow(unused)]
fn create_gear_compare_api_for_testing(
    combat_time_millisecond: TimeType,
    party1: Vec<PlayerInfoRequest>,
    party2: Vec<PlayerInfoRequest>,
) -> GearCompareApiRequest {
    let request1 = SimulationApiRequest {
        main_player_id: 0,
        combat_time_millisecond,
        party: party1,
        party_ilvl_adjustment: 100.0,
        use_pot: true,
    };

    let request2 = SimulationApiRequest {
        main_player_id: 0,
        combat_time_millisecond,
        party: party2,
        party_ilvl_adjustment: 100.0,
        use_pot: true,
    };

    GearCompareApiRequest {
        gear1_request: request1,
        gear2_request: request2,
    }
}

#[cfg(test)]
fn create_best_stats_request(
    main_player_id: PlayerIdType,
    party: Vec<PlayerInfoRequest>,
    stat_name: String,
    augment_amount: i32,
) -> BestStatsApiRequest {
    BestStatsApiRequest {
        main_player_id,
        party,
        stat_name,
        augment_amount: augment_amount as u16,
        use_pot: true,
        combat_time_millisecond: 30000,
        party_ilvl_adjustment: 100.0,
    }
}

#[cfg(test)]
fn assert_test_value_is_in_range<T>(value: T, lower_bound: T, upper_bound: T)
where
    T: PartialOrd + Display,
{
    assert!(
        value >= lower_bound,
        "Value {} is lower than lower bound {}",
        value,
        lower_bound
    );
    assert!(
        value <= upper_bound,
        "Value {} is higher than upper bound {}",
        value,
        upper_bound
    );
}

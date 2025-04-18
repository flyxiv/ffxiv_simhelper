use crate::response::aggregate_damage_simulation_data::{
    aggregate_contribution, aggregate_player_damage_statistics, aggregate_skill_damage,
    aggregate_status_damages, RaidbuffDamageAggregate, TotalDamageAggregateForEachPlayer,
    TotalDamageAggregateForEachSkill,
};
use crate::response::from_with_time::FromWithTime;
use crate::response::simulation_api_response::{
    DamageProfileResponse, PartyBurstContributionResponse, PartyContributionResponse,
    SimulationApiResponse, SimulationDataResponse, SimulationSummaryResponse, SkillLogResponse,
    SKILL_ENTITY_STRING, STATUS_ENTITY_STRING,
};
use ffxiv_simhelper_combat_components::jobs_skill_data::ninja::combat_resources::BUNSHIN_CLONE_ID;
use ffxiv_simhelper_combat_components::live_objects::player::logs::SkillLog;
use ffxiv_simhelper_combat_components::live_objects::player::player_power::PlayerPower;
use ffxiv_simhelper_combat_components::types::{DpsType, MultiplierType, PlayerIdType};
use ffxiv_simhelper_combat_components::types::{SkillIdType, TimeType};
use ffxiv_simhelper_dps_simulator::simulation_result::SimulationResult;
use itertools::{izip, Itertools};
use std::collections::HashMap;

pub(crate) fn convert_to_skill_log_response(skill_log: &SkillLog) -> SkillLogResponse {
    SkillLogResponse {
        time: skill_log.time,
        skill_id: skill_log.skill_id,
        target: skill_log.target_id,
        buffs: skill_log.buffs.clone(),
        debuffs: skill_log.debuffs.clone(),
    }
}

#[inline]
fn damage_to_dps(damage: DpsType, time: TimeType) -> DpsType {
    damage / (time / 1000) as DpsType
}

impl FromWithTime<TotalDamageAggregateForEachPlayer> for SimulationSummaryResponse {
    fn from_with_time(
        player_damage_aggregate: TotalDamageAggregateForEachPlayer,
        combat_time_millisecond: TimeType,
    ) -> Self {
        let given_contributions = player_damage_aggregate
            .total_rdps_contributions
            .values()
            .sum::<DpsType>();

        SimulationSummaryResponse {
            rdps: vec![damage_to_dps(
                (player_damage_aggregate.total_raw_damage
                    + player_damage_aggregate.total_contributions_received)
                    as DpsType,
                combat_time_millisecond,
            )],
            adps: vec![damage_to_dps(
                (player_damage_aggregate.total_raw_damage + given_contributions) as DpsType,
                combat_time_millisecond,
            )],
            pdps: vec![damage_to_dps(
                player_damage_aggregate.total_raw_damage + given_contributions,
                combat_time_millisecond,
            )],
            edps: vec![damage_to_dps(
                player_damage_aggregate.total_raw_damage
                    + player_damage_aggregate.total_contributions_received
                    + given_contributions,
                combat_time_millisecond,
            )],
        }
    }
}

fn create_skill_damage_profile_response(
    skill_id: SkillIdType,
    skill_damage_aggregate: &TotalDamageAggregateForEachSkill,
    combat_time_millisecond: TimeType,
) -> DamageProfileResponse {
    let total_damage = skill_damage_aggregate.total_raw_damage
        + skill_damage_aggregate
            .total_rdps_contribution
            .values()
            .sum::<DpsType>();

    DamageProfileResponse {
        id: skill_id,
        entity: String::from(SKILL_ENTITY_STRING),
        rdps_contribution: damage_to_dps(
            skill_damage_aggregate.total_raw_damage,
            combat_time_millisecond,
        ),
        pdps_contribution: damage_to_dps(total_damage, combat_time_millisecond),
        cast_count: skill_damage_aggregate.cast_count,
    }
}

fn create_status_damage_profile_response(
    skill_id: SkillIdType,
    status_damage_aggregate: &RaidbuffDamageAggregate,
    combat_time_millisecond: TimeType,
) -> DamageProfileResponse {
    let total_damage: MultiplierType =
        status_damage_aggregate.total_raw_damage + status_damage_aggregate.total_raid_damage;
    DamageProfileResponse {
        id: skill_id,
        entity: String::from(STATUS_ENTITY_STRING),
        rdps_contribution: damage_to_dps(total_damage, combat_time_millisecond),
        // only calculate buff for self
        pdps_contribution: damage_to_dps(
            status_damage_aggregate.total_raw_damage,
            combat_time_millisecond,
        ),
        cast_count: 0,
    }
}

fn calculate_damage_profile_response(
    skill_damage_tables: &HashMap<SkillIdType, TotalDamageAggregateForEachSkill>,
    status_damage_aggregate: &HashMap<SkillIdType, RaidbuffDamageAggregate>,
    combat_time_millisecond: TimeType,
) -> Vec<DamageProfileResponse> {
    let mut damage_profile_responses = vec![];

    for (skill_id, skill_damage_table) in skill_damage_tables {
        damage_profile_responses.push(create_skill_damage_profile_response(
            *skill_id,
            skill_damage_table,
            combat_time_millisecond,
        ));
    }

    for (status_id, status_damage_table) in status_damage_aggregate {
        damage_profile_responses.push(create_status_damage_profile_response(
            *status_id,
            status_damage_table,
            combat_time_millisecond,
        ));
    }

    damage_profile_responses
}

fn create_party_contribution_response(
    player_id: PlayerIdType,
    skill_damage_table: &HashMap<SkillIdType, TotalDamageAggregateForEachSkill>,
) -> Vec<PartyContributionResponse> {
    let mut party_contribution_responses = vec![];

    for (skill_id, skill_damage_aggregate) in skill_damage_table {
        for (status_key, contributed_damage) in
            skill_damage_aggregate.total_rdps_contribution.iter()
        {
            let party_member_id = status_key.player_id;

            if party_member_id == player_id {
                continue;
            }

            party_contribution_responses.push(PartyContributionResponse {
                skill_id: *skill_id,
                party_member_id,
                status_id: status_key.status_id,
                contributed_damage: *contributed_damage as i32,
            })
        }
    }

    party_contribution_responses
}

fn create_party_burst_contribution_response(
    player_id: PlayerIdType,
    skill_burst_damage_table: &HashMap<(PlayerIdType, TimeType), MultiplierType>,
) -> Vec<PartyBurstContributionResponse> {
    let mut party_contribution_responses = vec![];

    for ((party_member_id, minute), skill_burst_damage_aggregate) in skill_burst_damage_table {
        if *party_member_id == player_id {
            continue;
        }

        party_contribution_responses.push(PartyBurstContributionResponse {
            party_member_id: *party_member_id,
            contributed_damage: *skill_burst_damage_aggregate as i32,
            minute: *minute,
        });
    }

    party_contribution_responses.sort_by(|a, b| a.minute.cmp(&b.minute));

    party_contribution_responses
}

pub(crate) fn create_response_from_simulation_result(
    simulation_result: SimulationResult,
    main_player_power: PlayerPower,
    main_player_job_abbrev: String,
) -> SimulationApiResponse {
    let combat_time_millisecond = simulation_result.combat_time_millisecond;

    let damage_logs_of_all_players = simulation_result
        .party_simulation_results
        .iter()
        .map(|party_simulation_result| party_simulation_result.damage_log.clone())
        .collect_vec();

    // damage logs -> damage profile for each skill
    let (skill_damage_tables, skill_burst_damage_tables) =
        aggregate_skill_damage(&damage_logs_of_all_players);

    // damage profile for each skill => damage profile for each raidbuff
    let status_damage_aggregates = aggregate_status_damages(&skill_damage_tables);

    // damage profile for each skill => total contribution of each player to each raidbuff
    let party_damage_contribution_table = skill_damage_tables
        .iter()
        .map(|skill_damage_table| aggregate_contribution(skill_damage_table))
        .collect_vec();

    // damage profile for each skill => damage profile(total raw damage, total contribution, total buffs taken) for each player
    let player_damage_profile =
        aggregate_player_damage_statistics(&party_damage_contribution_table, &skill_damage_tables);

    // raw damage, buffs taken, buffs given => summary(pdps, rdps, adps, edps) for each player
    let summaries = player_damage_profile
        .iter()
        .map(|player_damage_aggregate| {
            SimulationSummaryResponse::from_with_time(
                player_damage_aggregate.clone(),
                combat_time_millisecond,
            )
        })
        .collect_vec();

    // convert to response type
    let damage_profile_responses = izip!(&skill_damage_tables, &status_damage_aggregates)
        .map(|(skill_damage_table, status_damage_aggregate)| {
            calculate_damage_profile_response(
                skill_damage_table,
                status_damage_aggregate,
                combat_time_millisecond,
            )
        })
        .collect_vec();

    let party_contribution_responses = skill_damage_tables
        .iter()
        .enumerate()
        .map(|(player_id, skill_damage_table)| {
            create_party_contribution_response(player_id as PlayerIdType, skill_damage_table)
        })
        .collect_vec();

    let party_burst_contribution_responses = skill_burst_damage_tables
        .iter()
        .enumerate()
        .map(|(player_id, skill_burst_damage_tables)| {
            create_party_burst_contribution_response(
                player_id as PlayerIdType,
                skill_burst_damage_tables,
            )
        })
        .collect_vec();

    let rotation_log_responses = simulation_result
        .party_simulation_results
        .iter()
        .map(|party_simulation_result| {
            party_simulation_result
                .skill_log
                .iter()
                .map(|skill_log| convert_to_skill_log_response(skill_log))
                .filter(|skill_log_response| skill_log_response.skill_id != BUNSHIN_CLONE_ID)
                .collect_vec()
        })
        .collect_vec();

    let mut simulation_data_responses = vec![];

    for (
        party_simulation_result,
        summary,
        party_contribution_response,
        party_burst_contribution_response,
        damage_profile_response,
        rotation_log_response,
    ) in izip!(
        &simulation_result.party_simulation_results,
        summaries,
        party_contribution_responses,
        party_burst_contribution_responses,
        damage_profile_responses,
        rotation_log_responses
    ) {
        simulation_data_responses.push(SimulationDataResponse {
            player_id: party_simulation_result.player_id,
            job_abbrev: party_simulation_result.job.clone(),
            role: party_simulation_result.role.clone(),
            simulation_summary: summary,
            party_contribution_table: party_contribution_response,
            damage_profile_table: damage_profile_response,
            party_burst_contribution_table: party_burst_contribution_response,
            rotation_log: rotation_log_response,
        });
    }

    SimulationApiResponse {
        main_player_id: simulation_result.main_player_id,
        combat_time_millisecond,
        simulation_data: simulation_data_responses,
        main_player_power,
        main_player_job_abbrev,
    }
}

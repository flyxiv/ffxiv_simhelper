use crate::response::aggregate_damage_simulation_data::{
    aggregate_contribution, aggregate_player_damage_statistics, aggregate_skill_damage,
    aggregate_status_damages, PlayerDamageAggregate, RaidbuffDamageAggregate, SkillDamageAggregate,
};
use crate::response::from_with_time::FromWithTime;
use crate::response::simulation_api_response::{
    DamageProfileResponse, PartyContributionResponse, RotationLogResponse, SimulationApiResponse,
    SimulationDataResponse, SimulationSummaryResponse, SKILL_ENTITY_STRING,
};
use ffxiv_simbot_combat_components::{DamageType, DpsType, IdType, TimeType};
use itertools::{izip, Itertools};
use juniper::GraphQLObject;
use serde::Serialize;
use std::collections::HashMap;

/// GraphQL object for creating GraphQL Query Responses
/// Saves all the raw data from the simulation
/// and aggregates raw data to needed format depending on the requested query.
#[derive(Debug, Serialize, Clone)]
#[GraphQLObject]
pub struct SimulationResult {
    pub main_player_id: IdType,
    pub combat_time_millisecond: TimeType,
    pub party_simulation_result: Vec<PartySimulationResult>,
}

#[derive(Debug, Serialize, Clone)]
pub struct PartySimulationResult {
    pub player_id: IdType,
    pub job: String,
    pub role: String,
    pub rotation_log: Vec<RotationLog>,
}

#[derive(Debug, Serialize, Clone)]
pub struct RotationLog {
    pub time: TimeType,
    pub skill_id: IdType,
    pub target: Option<IdType>,
    pub raw_damage: DamageType,
    pub rdps_contribution: Vec<RdpsContribution>,
}

#[derive(Debug, Serialize, Clone)]
pub struct RdpsContribution {
    pub player_id: IdType,
    pub raid_buff_status_id: IdType,
    pub contributed_damage: DamageType,
}

impl SimulationResult {
    fn aggregate_simulation_result(&self) -> SimulationApiResponse {
        let main_player_id = self.main_player_id;
        let combat_time_millisecond = self.combat_time_millisecond;

        let rotation_logs_of_all_players = self
            .party_simulation_result
            .iter()
            .map(|party_simulation_result| party_simulation_result.rotation_log)
            .collect_vec();

        let rotation_log_aggregate = aggregate_skill_damage(&rotation_logs_of_all_players);

        let skill_damage_tables = rotation_log_aggregate.skill_damage_tables;

        let status_damage_aggregates = aggregate_status_damages(&skill_damage_tables);
        let party_damage_contribution_table = skill_damage_tables
            .iter()
            .map(|skill_damage_table| aggregate_contribution(skill_damage_table))
            .collect_vec();

        let player_simulation = aggregate_player_damage_statistics(
            &party_damage_contribution_table,
            &skill_damage_tables,
        );

        let summaries = player_simulation
            .iter()
            .map(|player_damage_aggregate| {
                SimulationSummaryResponse::from_with_time(
                    player_damage_aggregate.clone(),
                    combat_time_millisecond,
                )
            })
            .collect_vec();

        let damage_profile_responses = izip!(skill_damage_tables, status_damage_aggregates)
            .map(|(skill_damage_table, status_damage_aggregate)| {
                calculate_damage_profile_response(
                    &skill_damage_table,
                    &status_damage_aggregate,
                    combat_time_millisecond,
                )
            })
            .collect_vec();

        let party_contribution_responses = skill_damage_tables
            .iter()
            .enumerate()
            .map(|(player_id, skill_damage_table)| {
                create_party_contribution_response(
                    player_id,
                    skill_damage_table,
                    combat_time_millisecond,
                )
            })
            .collect_vec();

        let rotation_log_responses = rotation_log_aggregate.rotation_log_responses;

        let mut simulation_data_responses = vec![];

        for (
            party_simulation_result,
            summary,
            party_contribution_response,
            damage_profile_response,
            rotation_log_response,
        ) in izip!(
            &self.party_simulation_result,
            summaries,
            party_contribution_responses,
            damage_profile_responses,
            rotation_log_responses
        ) {
            simulation_data_responses.push(SimulationDataResponse {
                player_id: party_simulation_result.player_id,
                job: party_simulation_result.job.clone(),
                role: party_simulation_result.role.clone(),
                simulation_summary: summary,
                party_contribution_table: party_contribution_response,
                damage_profile_table: damage_profile_response,
                rotation_log: rotation_log_response,
            });
        }

        SimulationApiResponse {
            main_player_id,
            combat_time_millisecond,
            simulation_data: simulation_data_responses,
        }
    }
}

impl From<&RotationLog> for RotationLogResponse {
    fn from(rotation_log: &RotationLog) -> Self {
        RotationLogResponse {
            time: rotation_log.time,
            skill_id: rotation_log.skill_id,
            target: rotation_log.target,
        }
    }
}

#[inline]
fn damage_to_dps(damage: DamageType, time: TimeType) -> DpsType {
    damage as DpsType / time as DpsType
}

impl FromWithTime<PlayerDamageAggregate> for SimulationSummaryResponse {
    fn from_with_time(
        player_damage_aggregate: PlayerDamageAggregate,
        combat_time_millisecond: TimeType,
    ) -> Self {
        let given_contributions = player_damage_aggregate
            .total_rdps_contributions
            .values()
            .sum::<DamageType>();

        SimulationSummaryResponse {
            rdps: damage_to_dps(
                player_damage_aggregate.total_raw_damage,
                combat_time_millisecond,
            ),
            adps: damage_to_dps(
                player_damage_aggregate.total_raw_damage + given_contributions
                    - player_damage_aggregate.total_contributions_received,
                combat_time_millisecond,
            ),
            pdps: damage_to_dps(
                player_damage_aggregate.total_raw_damage + given_contributions,
                combat_time_millisecond,
            ),
            edps: damage_to_dps(
                player_damage_aggregate.total_raw_damage
                    + player_damage_aggregate.total_contributions_received
                    + given_contributions,
                combat_time_millisecond,
            ),
        }
    }
}

fn create_skill_damage_profile_response(
    skill_id: IdType,
    skill_damage_aggregate: &SkillDamageAggregate,
    combat_time_millisecond: TimeType,
) -> DamageProfileResponse {
    let total_damage: DamageType = skill_damage_aggregate.total_raw_damage
        + skill_damage_aggregate
            .total_rdps_contribution
            .values()
            .sum();
    DamageProfileResponse {
        id: skill_id,
        entity: SKILL_ENTITY_STRING,
        rdps_contribution: damage_to_dps(
            skill_damage_aggregate.total_raw_damage,
            combat_time_millisecond,
        ),
        pdps_contribution: damage_to_dps(total_damage, combat_time_millisecond),
    }
}

fn create_status_damage_profile_response(
    skill_id: IdType,
    status_damage_aggregate: &RaidbuffDamageAggregate,
    combat_time_millisecond: TimeType,
) -> DamageProfileResponse {
    let total_damage: DamageType =
        status_damage_aggregate.total_raw_damage + status_damage_aggregate.total_raid_damage;
    DamageProfileResponse {
        id: skill_id,
        entity: SKILL_ENTITY_STRING,
        rdps_contribution: damage_to_dps(total_damage, combat_time_millisecond),
        // only calculate buff for self
        pdps_contribution: damage_to_dps(
            status_damage_aggregate.total_raw_damage,
            combat_time_millisecond,
        ),
    }
}

fn calculate_damage_profile_response(
    skill_damage_tables: &HashMap<IdType, SkillDamageAggregate>,
    status_damage_aggregate: &HashMap<IdType, RaidbuffDamageAggregate>,
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
    player_id: IdType,
    skill_damage_table: &HashMap<IdType, SkillDamageAggregate>,
    combat_time_millisecond: TimeType,
) -> Vec<PartyContributionResponse> {
    let mut party_contribution_responses = vec![];

    for (skill_id, skill_damage_aggregate) in skill_damage_table {
        for (status_key, contributed_damage) in
            skill_damage_aggregate.total_rdps_contribution.iter()
        {
            let party_member_id = status_key.player_id;
            let contributed_rdps = damage_to_dps(*contributed_damage, combat_time_millisecond);

            if party_member_id == player_id {
                continue;
            }

            party_contribution_responses.push(PartyContributionResponse {
                skill_id: *skill_id,
                party_member_id,
                status_id: status_key.status_id,
                contributed_rdps,
            })
        }
    }

    party_contribution_responses
}

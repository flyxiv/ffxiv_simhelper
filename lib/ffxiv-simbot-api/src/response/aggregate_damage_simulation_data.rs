use crate::response::simulation_api_response::RotationLogResponse;
use crate::response::simulation_result::RotationLog;
use ffxiv_simbot_combat_components::live_objects::player::StatusKey;
use ffxiv_simbot_combat_components::{DamageType, IdType};
use itertools::izip;
use std::collections::HashMap;

/// Sum up all damage profile for each skill.
#[derive(Clone)]
pub(crate) struct SkillDamageAggregate {
    pub(crate) total_raw_damage: DamageType,
    pub(crate) total_rdps_contribution: HashMap<StatusKey, DamageType>,
}

/// Sum up all damage profile for each raidbuff.
/// Damage buffs to self are calculated as raw damage, the rest are calculated as rdps.
#[derive(Clone)]
pub(crate) struct RaidbuffDamageAggregate {
    pub(crate) total_raw_damage: DamageType,
    pub(crate) total_raid_damage: DamageType,
}

#[derive(Clone)]
pub(crate) struct PlayerDamageAggregate {
    pub(crate) total_raw_damage: DamageType,
    pub(crate) total_contributions_received: DamageType,
    pub(crate) total_rdps_contributions: HashMap<IdType, DamageType>,
}

impl Default for PlayerDamageAggregate {
    fn default() -> Self {
        PlayerDamageAggregate {
            total_raw_damage: 0,
            total_contributions_received: 0,
            total_rdps_contributions: HashMap::new(),
        }
    }
}

impl Default for SkillDamageAggregate {
    fn default() -> Self {
        SkillDamageAggregate {
            total_raw_damage: 0,
            total_rdps_contribution: Default::default(),
        }
    }
}

pub(crate) struct RotationLogAggregate {
    pub(crate) rotation_log_responses: Vec<Vec<RotationLogResponse>>,
    pub(crate) skill_damage_tables: Vec<HashMap<IdType, SkillDamageAggregate>>,
}

/// Aggregate the total number of raw damage
/// and the total number of buff contribution for each raidbuff.
/// for each skill unit.
pub(crate) fn aggregate_skill_damage(
    rotation_logs: &Vec<Vec<RotationLog>>,
) -> RotationLogAggregate {
    let mut skill_damage_tables = vec![];
    let mut rotation_log_responses = vec![];

    for rotation_log in rotation_logs {
        let mut skill_damage_table = HashMap::new();
        let mut rotation_log_response = vec![];

        for log in rotation_log.iter() {
            let skill_damage_entry = skill_damage_table
                .entry(log.skill_id)
                .or_insert(SkillDamageAggregate::default());

            skill_damage_entry.total_raw_damage += log.raw_damage;

            for rdps_contribution in log.rdps_contribution.iter() {
                let status_key = StatusKey::new(
                    rdps_contribution.raid_buff_status_id,
                    rdps_contribution.player_id,
                );

                let rdps_contribution_entry = skill_damage_entry
                    .total_rdps_contribution
                    .entry(status_key)
                    .or_insert(0);

                *rdps_contribution_entry += rdps_contribution.contributed_damage;
            }

            rotation_log_response.push(RotationLogResponse::from(log));
        }

        skill_damage_tables.push(skill_damage_table);
        rotation_log_responses.push(rotation_log_response);
    }

    RotationLogAggregate {
        skill_damage_tables,
        rotation_log_responses,
    }
}

/// Aggregate the total number of buff contribution for each player in player units.
pub(crate) fn aggregate_contribution(
    skill_damage_table: &HashMap<IdType, SkillDamageAggregate>,
) -> HashMap<StatusKey, DamageType> {
    let mut contribution_table = HashMap::new();
    for skill_damage in skill_damage_table.values() {
        for (status_key, contributed_damage) in skill_damage.total_rdps_contribution.iter() {
            let contribution_entry = contribution_table.entry(*status_key).or_insert(0);
            *contribution_entry += contributed_damage;
        }
    }

    contribution_table
}

/// Calculate the final damage profile statistic for each player:
/// raw damage, given contribution and received contribution.
pub(crate) fn aggregate_player_damage_statistics(
    party_damage_contribution_table: &Vec<HashMap<StatusKey, DamageType>>,
    skill_damage_tables: &Vec<HashMap<IdType, SkillDamageAggregate>>,
) -> Vec<PlayerDamageAggregate> {
    let mut party_damage_aggregate = Vec::new();
    party_damage_aggregate.resize(party_damage_contribution_table.len(), Default::default());

    for (player_id, (contribution_table, skill_damage_table)) in
        izip!(party_damage_contribution_table, skill_damage_tables).enumerate()
    {
        for skill_damage in skill_damage_table.values() {
            party_damage_aggregate[player_id].total_raw_damage += skill_damage.total_raw_damage;
        }

        for (status_key, contributed_damage) in contribution_table.iter() {
            if status_key.player_id == player_id {
                party_damage_aggregate[player_id].total_raw_damage += *contributed_damage;
            } else {
                party_damage_aggregate[status_key.player_id].total_contributions_received +=
                    contributed_damage;
                let entry = party_damage_aggregate[player_id]
                    .total_rdps_contributions
                    .entry(status_key.player_id)
                    .or_insert(0);
                entry += contributed_damage;
            }
        }
    }

    party_damage_aggregate
}

pub(crate) fn aggregate_status_damages(
    skill_damage_tables: &Vec<HashMap<IdType, SkillDamageAggregate>>,
) -> Vec<HashMap<IdType, RaidbuffDamageAggregate>> {
    let mut status_damages = vec![];

    for _ in 0..skill_damage_tables.len() {
        status_damages.push(HashMap::new());
    }

    for (player_id, skill_damage_table) in skill_damage_tables.iter().enumerate() {
        for skill_damage in skill_damage_table.values() {
            for rdps_contribution in skill_damage.total_rdps_contribution.iter() {
                let status_id = rdps_contribution.0.status_id;
                let affected_player_id = rdps_contribution.0.player_id;

                let contributed_damage = rdps_contribution.1;
                let entry =
                    status_damages[player_id]
                        .entry(status_id)
                        .or_insert(RaidbuffDamageAggregate {
                            total_raw_damage: 0,
                            total_raid_damage: 0,
                        });

                if affected_player_id == player_id {
                    entry.total_raw_damage += *contributed_damage;
                } else {
                    entry.total_raid_damage += contributed_damage;
                }
            }
        }
    }

    status_damages
}

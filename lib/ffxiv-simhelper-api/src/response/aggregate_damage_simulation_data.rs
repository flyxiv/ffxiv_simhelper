use crate::response::CountType;
use ffxiv_simhelper_combat_components::live_objects::player::logs::DamageLog;
use ffxiv_simhelper_combat_components::live_objects::player::StatusKey;
use ffxiv_simhelper_combat_components::types::{MultiplierType, TimeType};
use ffxiv_simhelper_combat_components::types::{PlayerIdType, SkillIdType};
use itertools::izip;
use std::collections::HashMap;

const MINUTE_IN_MILLISECOND: TimeType = 60000;

/// standard step, wanderer, mages ballad and armys paeon
const PASSIVE_RAIDBUFFS: [SkillIdType; 4] = [1500, 1302, 1304, 1306];
const PASSIVE_END_MILLISECOND: TimeType = 30000;
const PASSIVE_START_MILLISECOND: TimeType = 4000;

/// Sum up all damage profile for each skill.
#[derive(Clone)]
pub(crate) struct SkillDamageAggregate {
    pub(crate) total_raw_damage: MultiplierType,
    pub(crate) total_rdps_contribution: HashMap<StatusKey, MultiplierType>,
    pub(crate) cast_count: CountType,
}

/// Sum up all damage profile for each raidbuff.
/// Damage buffs to self are calculated as raw damage, the rest are calculated as rdps.
#[derive(Clone)]
pub(crate) struct RaidbuffDamageAggregate {
    pub(crate) total_raw_damage: MultiplierType,
    pub(crate) total_raid_damage: MultiplierType,
}

#[derive(Clone)]
pub(crate) struct PlayerDamageAggregate {
    pub(crate) total_raw_damage: MultiplierType,
    pub(crate) total_contributions_received: MultiplierType,
    pub(crate) total_rdps_contributions: HashMap<PlayerIdType, MultiplierType>,
}

impl Default for PlayerDamageAggregate {
    fn default() -> Self {
        PlayerDamageAggregate {
            total_raw_damage: 0.0,
            total_contributions_received: 0.0,
            total_rdps_contributions: HashMap::new(),
        }
    }
}

impl Default for SkillDamageAggregate {
    fn default() -> Self {
        SkillDamageAggregate {
            total_raw_damage: 0.0,
            total_rdps_contribution: Default::default(),
            cast_count: 0,
        }
    }
}

/// Aggregate the total number of raw damage
/// and the total number of buff contribution for each raidbuff.
/// for each skill unit.
/// first hashmap: detailed dps profile of each skill(how much each skill contributed + raw damage)
/// second hashmap: burst profile: how much each player contributed to each player at each burst periods.
pub(crate) fn aggregate_skill_damage(
    damage_logs_of_party: &[Vec<DamageLog>],
) -> (
    Vec<HashMap<SkillIdType, SkillDamageAggregate>>,
    Vec<HashMap<(PlayerIdType, TimeType), MultiplierType>>,
) {
    let mut skill_damage_tables = vec![];
    let mut skill_burst_damage_tables = vec![];

    for damage_logs_of_single_player in damage_logs_of_party {
        let mut skill_damage_table = HashMap::new();
        let mut skill_burst_damage_table = HashMap::new();

        for damage_log in damage_logs_of_single_player.iter() {
            let damage_time_milliseconds = damage_log.time;
            let damage_minute = damage_time_milliseconds / MINUTE_IN_MILLISECOND;

            let skill_damage_entry = skill_damage_table
                .entry(damage_log.skill_id)
                .or_insert(SkillDamageAggregate::default());

            skill_damage_entry.total_raw_damage += damage_log.raw_damage;
            skill_damage_entry.cast_count += 1;

            for rdps_contribution in damage_log.rdps_contribution.iter() {
                let status_key = StatusKey::new(
                    rdps_contribution.raid_buff_status_id,
                    rdps_contribution.player_id,
                );

                /// only consider passive raidbuffs at even minutes:4 seconds to 30 seconds
                let time_key = if PASSIVE_RAIDBUFFS.contains(&rdps_contribution.raid_buff_status_id)
                {
                    let time_offset = damage_time_milliseconds % MINUTE_IN_MILLISECOND;
                    if time_offset >= PASSIVE_START_MILLISECOND
                        && time_offset <= PASSIVE_END_MILLISECOND
                        && damage_minute % 2 == 0
                    {
                        Some(damage_minute)
                    } else {
                        None
                    }
                } else {
                    Some(damage_minute)
                };

                let rdps_contribution_entry = skill_damage_entry
                    .total_rdps_contribution
                    .entry(status_key)
                    .or_insert(0.0);

                *rdps_contribution_entry += rdps_contribution.contributed_damage;

                if let Some(minute) = time_key {
                    let skill_burst_damage_entry = skill_burst_damage_table
                        .entry((rdps_contribution.player_id, minute))
                        .or_insert(0.0);

                    *skill_burst_damage_entry += rdps_contribution.contributed_damage;
                }
            }
        }

        skill_damage_tables.push(skill_damage_table);
        skill_burst_damage_tables.push(skill_burst_damage_table);
    }

    (skill_damage_tables, skill_burst_damage_tables)
}

/// Aggregate the total number of buff contribution for each player in player units.
pub(crate) fn aggregate_contribution(
    skill_damage_table: &HashMap<SkillIdType, SkillDamageAggregate>,
) -> HashMap<StatusKey, MultiplierType> {
    let mut contribution_table = HashMap::new();
    for skill_damage in skill_damage_table.values() {
        for (status_key, contributed_damage) in skill_damage.total_rdps_contribution.iter() {
            let contribution_entry = contribution_table.entry(*status_key).or_insert(0.0);
            *contribution_entry += *contributed_damage;
        }
    }

    contribution_table
}

/// Calculate the final damage profile statistic for each player:
/// raw damage, given contribution and received contribution.
pub(crate) fn aggregate_player_damage_statistics(
    party_damage_contribution_table: &[HashMap<StatusKey, MultiplierType>],
    skill_damage_tables: &[HashMap<SkillIdType, SkillDamageAggregate>],
) -> Vec<PlayerDamageAggregate> {
    let mut party_damage_aggregate: Vec<PlayerDamageAggregate> = vec![];
    party_damage_aggregate.resize(party_damage_contribution_table.len(), Default::default());

    for (player_id, (contribution_table, skill_damage_table)) in
        izip!(party_damage_contribution_table, skill_damage_tables).enumerate()
    {
        for skill_damage in skill_damage_table.values() {
            party_damage_aggregate[player_id].total_raw_damage += skill_damage.total_raw_damage;
        }

        for (status_key, contributed_damage) in contribution_table.iter() {
            if status_key.player_id == player_id as PlayerIdType {
                party_damage_aggregate[player_id].total_raw_damage += *contributed_damage;
            } else {
                party_damage_aggregate[status_key.player_id as usize]
                    .total_contributions_received += contributed_damage;
                let entry = party_damage_aggregate[player_id]
                    .total_rdps_contributions
                    .entry(status_key.player_id)
                    .or_insert(0.0);
                *entry += contributed_damage;
            }
        }
    }

    party_damage_aggregate
}

/// Aggregates total raw/raid damage a player has given to each raidbuff
/// Vec[player_id] contains raidbuff damage aggregate for each player.
/// HashMap<SkillIdType, RaidbuffDamageAggregate> contains the total raw/raid damage a player has given to each raidbuff.
/// HashMap<(SkillIdType, TimeType), RaidbuffDamageAggregate> contains the total raw/raid damage a player has given at each burst time.
pub(crate) fn aggregate_status_damages(
    skill_damage_tables: &[HashMap<SkillIdType, SkillDamageAggregate>],
) -> Vec<HashMap<SkillIdType, RaidbuffDamageAggregate>> {
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
                            total_raw_damage: 0.0,
                            total_raid_damage: 0.0,
                        });

                if affected_player_id == (player_id as PlayerIdType) {
                    entry.total_raw_damage += *contributed_damage;
                } else {
                    entry.total_raid_damage += contributed_damage;
                }
            }
        }
    }

    status_damages
}

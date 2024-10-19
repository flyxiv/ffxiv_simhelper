use crate::response::CountType;
use ffxiv_simhelper_combat_components::live_objects::player::logs::DamageLog;
use ffxiv_simhelper_combat_components::live_objects::player::StatusKey;
use ffxiv_simhelper_combat_components::types::{MultiplierType, StatusIdType, TimeType};
use ffxiv_simhelper_combat_components::types::{PlayerIdType, SkillIdType};
use itertools::izip;
use std::collections::HashMap;

const MINUTE_IN_MILLISECOND: TimeType = 60000;

// standard step, wanderer, mages ballad and armys paeon
const PASSIVE_RAIDBUFFS: [SkillIdType; 4] = [1500, 1302, 1304, 1306];

// Sum up all damage profile for each skill.
#[derive(Clone)]
pub struct TotalDamageAggregateForEachSkill {
    pub total_raw_damage: MultiplierType,
    pub total_rdps_contribution: HashMap<StatusKey, MultiplierType>,
    pub cast_count: CountType,
}

// Sum up all damage profile for each raidbuff.
// Damage buffs to self are calculated as raw damage, the rest are calculated as rdps.
#[derive(Clone)]
pub struct RaidbuffDamageAggregate {
    pub total_raw_damage: MultiplierType,
    pub total_raid_damage: MultiplierType,
}

#[derive(Clone)]
pub struct TotalDamageAggregateForEachPlayer {
    pub total_raw_damage: MultiplierType,
    pub total_contributions_received: MultiplierType,
    pub total_rdps_contributions: HashMap<PlayerIdType, MultiplierType>,
}

impl Default for TotalDamageAggregateForEachPlayer {
    fn default() -> Self {
        TotalDamageAggregateForEachPlayer {
            total_raw_damage: 0.0,
            total_contributions_received: 0.0,
            total_rdps_contributions: HashMap::new(),
        }
    }
}

impl Default for TotalDamageAggregateForEachSkill {
    fn default() -> Self {
        TotalDamageAggregateForEachSkill {
            total_raw_damage: 0.0,
            total_rdps_contribution: Default::default(),
            cast_count: 0,
        }
    }
}

/// * HashMap<SkillIdType, TotalDamageAggregateForEachSkill>: skill_id -> TotalDamageAggregateForEachSkill
/// * Vec<HashMap<SkillIdType, TotalDamageAggregateForEachSkill>>: vec[player_id] contains skill damage profile of player id.
type TotalDamageSkillAggregateOfParty = Vec<HashMap<SkillIdType, TotalDamageAggregateForEachSkill>>;

/// * HashMap<(PlayerIdType, TimeType), MultiplierType>: (player_id, burst minute(=0, 2, 4, 6, etc)) -> MultiplierType
/// * Vec<HashMap<(PlayerIdType, TimeType), MultiplierType>>: vec[player_id] contains burst damage profile of player id.
/// Only used for BestPartner API
type TotalDamageBurstAggregateOfParty = Vec<HashMap<(PlayerIdType, TimeType), MultiplierType>>;

/// Parse the party's raw damage logs to aggregate various damage profiles:
/// 1) the total number of raw damage
/// 2) the total number of buff contribution of each skill for each raidbuff.
/// 3) the total number of buff contribution of each player for each raidbuff in each burst window(opener=0, 2, 4, ...).
pub(crate) fn aggregate_skill_damage(
    damage_logs_of_party: &[Vec<DamageLog>],
) -> (
    TotalDamageSkillAggregateOfParty,
    TotalDamageBurstAggregateOfParty,
) {
    let mut skill_damage_tables = vec![];
    let mut skill_burst_damage_tables = vec![];

    for damage_logs_of_single_player in damage_logs_of_party {
        let mut skill_damage_table = HashMap::new();
        let mut skill_burst_damage_table = HashMap::new();

        let mut highest_burst_minute = 0;

        for damage_log in damage_logs_of_single_player.iter() {
            let damage_time_milliseconds = damage_log.time;
            let damage_minute = damage_time_milliseconds / MINUTE_IN_MILLISECOND;

            // for long dots like Higanbana, the raidbuff snapshotted at even minute burst can continue to odd minutes.
            // so we need to floor those damage_minutes to even minutes.
            let burst_damage_minute = if damage_minute % 2 == 1 {
                damage_minute - 1
            } else {
                damage_minute
            };

            let skill_damage_entry = skill_damage_table
                .entry(damage_log.skill_id)
                .or_insert(TotalDamageAggregateForEachSkill::default());

            skill_damage_entry.total_raw_damage += damage_log.raw_damage;
            skill_damage_entry.cast_count += 1;

            for rdps_contribution in damage_log.rdps_contribution.iter() {
                let status_key = StatusKey::new(
                    rdps_contribution.raid_buff_status_id,
                    rdps_contribution.player_id,
                );

                // We ignore passive raidbuffs for burst damage calculation.
                let time_key = if PASSIVE_RAIDBUFFS.contains(&rdps_contribution.raid_buff_status_id)
                {
                    None
                } else {
                    Some(burst_damage_minute)
                };

                let rdps_contribution_entry = skill_damage_entry
                    .total_rdps_contribution
                    .entry(status_key)
                    .or_insert(0.0);

                *rdps_contribution_entry += rdps_contribution.contributed_damage;

                if let Some(minute) = time_key {
                    debug_assert!(
                        highest_burst_minute <= minute,
                        "{}, {}",
                        highest_burst_minute,
                        minute
                    );
                    highest_burst_minute = minute;

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

// Aggregate the total number of buff contribution for each player in player units.
/// ex)
/// ```rust
/// use ffxiv_simhelper_api::response::aggregate_damage_simulation_data::{TotalDamageAggregateForEachSkill, aggregate_contribution};
/// use ffxiv_simhelper_combat_components::live_objects::player::StatusKey;
/// use std::collections::HashMap;
///
/// let status_key1 = StatusKey {player_id: 1, status_id: 402};
/// let status_key2 = StatusKey {player_id: 2, status_id: 102};
/// let status_key3 = StatusKey {player_id: 2, status_id: 103};
///
/// let skill1_total_rdps_contribution = HashMap::from([(status_key1.clone(), 10.0), (status_key2.clone(), 20.0)]);
/// let skill2_total_rdps_contribution = HashMap::from([(status_key1.clone(), 30.0), (status_key2.clone(), 40.0), (status_key3.clone(), 50.0)]);
///
/// let skill_damage_table = HashMap::from([(1, TotalDamageAggregateForEachSkill {total_raw_damage: 400.0, total_rdps_contribution: skill1_total_rdps_contribution, cast_count: 1}), (2, TotalDamageAggregateForEachSkill {total_raw_damage: 200.0, total_rdps_contribution: skill2_total_rdps_contribution, cast_count: 2})]);
///
/// let aggregated_contribution = aggregate_contribution(&skill_damage_table);
///
/// assert_eq!(aggregated_contribution.len(), 3);
/// assert_eq!(aggregated_contribution[&status_key1], 40.0);
/// assert_eq!(aggregated_contribution[&status_key2], 60.0);
/// assert_eq!(aggregated_contribution[&status_key3], 50.0);
/// ```
pub fn aggregate_contribution(
    skill_damage_table: &HashMap<SkillIdType, TotalDamageAggregateForEachSkill>,
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
/// 1) Calculate total raw damage the player has done
/// 2) Calculate buffs received for each player by summing up all contributions of each party member to the player's raidbuffs
/// 3) Calculate total contribution the player has given to each player
///
/// player_id's data is stored at party_damage_contribution_table[player_id] and skill_damage_tables[player_id].
///
/// ex)
/// ```rust
/// use ffxiv_simhelper_api::response::aggregate_damage_simulation_data::{TotalDamageAggregateForEachPlayer, TotalDamageAggregateForEachSkill, aggregate_player_damage_statistics};
/// use ffxiv_simhelper_combat_components::live_objects::player::StatusKey;
/// use std::collections::HashMap;
///
/// let status_key1 = StatusKey {player_id: 0, status_id: 402};
/// let status_key2 = StatusKey {player_id: 1, status_id: 102};
/// let status_key3 = StatusKey {player_id: 1, status_id: 103};
///
/// let party_damage_contribution_table = vec![HashMap::from([(status_key1.clone(), 10.0), (status_key2.clone(), 20.0), (status_key3.clone(), 30.0)]), HashMap::from([(status_key1.clone(), 10.0), (status_key2.clone(), 20.0), (status_key3.clone(), 30.0)])];
/// let skill_damage_tables = vec![
///     HashMap::from([
///         (1, TotalDamageAggregateForEachSkill {
///             total_raw_damage: 400.0,
///             total_rdps_contribution: HashMap::new(),
///             cast_count: 1
///         }),
///         (2, TotalDamageAggregateForEachSkill {
///             total_raw_damage: 200.0,
///             total_rdps_contribution: HashMap::new(),
///             cast_count: 2
///         })]),
///     HashMap::from([
///         (3, TotalDamageAggregateForEachSkill {
///             total_raw_damage: 300.0,
///             total_rdps_contribution: HashMap::new(),
///             cast_count: 10
///         }),
///         (4, TotalDamageAggregateForEachSkill {
///             total_raw_damage: 400.0,
///             total_rdps_contribution: HashMap::new(),
///             cast_count: 3
///         })])
/// ];
///
/// let player_damage_statistics = aggregate_player_damage_statistics(&party_damage_contribution_table, &skill_damage_tables);
///
/// assert_eq!(player_damage_statistics.len(), 2);
/// assert_eq!(player_damage_statistics[0].total_raw_damage, 400.0 + 200.0 + 10.0);
/// assert_eq!(player_damage_statistics[0].total_contributions_received, 10.0);
/// assert_eq!(player_damage_statistics[1].total_raw_damage, 300.0 + 400.0 + 20.0 + 30.0);
/// assert_eq!(player_damage_statistics[1].total_contributions_received, 20.0 + 30.0);
/// ```
pub fn aggregate_player_damage_statistics(
    party_damage_contribution_table: &[HashMap<StatusKey, MultiplierType>],
    skill_damage_tables: &[HashMap<SkillIdType, TotalDamageAggregateForEachSkill>],
) -> Vec<TotalDamageAggregateForEachPlayer> {
    let mut party_damage_aggregate: Vec<TotalDamageAggregateForEachPlayer> = vec![];
    party_damage_aggregate.resize(party_damage_contribution_table.len(), Default::default());

    for (player_id, (contribution_table, skill_damage_table)) in
        izip!(party_damage_contribution_table, skill_damage_tables).enumerate()
    {
        // 1) Calculate Total raw damage of player_id by summing up all the total raw damage of each skill of player_id.
        for skill_total_damage_profile in skill_damage_table.values() {
            party_damage_aggregate[player_id].total_raw_damage +=
                skill_total_damage_profile.total_raw_damage;
        }

        for (status_key, contributed_damage) in contribution_table.iter() {
            // If the raidbuff belongs to player_id, the buffed up damage is calculated as raw damage.
            if status_key.player_id == player_id as PlayerIdType {
                party_damage_aggregate[player_id].total_raw_damage += *contributed_damage;
            } else {
                // Add the raidbuff contribution to the buff owner's total contribution received.
                party_damage_aggregate[status_key.player_id as usize]
                    .total_contributions_received += contributed_damage;

                // Add the raidbuff contribution to the skill owner's total contribution given.
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

/// Aggregates total raw/raid damage player has contributed to each raidbuff
/// Vec[player_id] contains raidbuff damage aggregate for each player.
/// HashMap<SkillIdType, RaidbuffDamageAggregate> contains the total raw/raid damage a player has given to each raidbuff.
/// HashMap<(SkillIdType, TimeType), RaidbuffDamageAggregate> contains the total raw/raid damage a player has given at each burst time.
///
/// ex)
/// ```rust
/// use ffxiv_simhelper_api::response::aggregate_damage_simulation_data::{TotalDamageAggregateForEachSkill, RaidbuffDamageAggregate, aggregate_status_damages};
/// use std::collections::HashMap;
/// use ffxiv_simhelper_combat_components::live_objects::player::StatusKey;
///
/// let status_key1 = StatusKey{player_id: 0, status_id: 402};
/// let status_key2 = StatusKey{player_id: 1, status_id: 102};
/// let status_key3 = StatusKey{player_id: 1, status_id: 103};
/// let status_key4 = StatusKey{player_id: 0, status_id: 401};
///
/// let skill_damage_tables = vec![
///     HashMap::from([
///         (1, TotalDamageAggregateForEachSkill {
///             total_raw_damage: 400.0,
///             total_rdps_contribution: HashMap::from([(status_key1.clone(), 10.0), (status_key2.clone(), 20.0), (status_key3.clone(), 30.0), (status_key4.clone(), 40.0)]),
///             cast_count: 5
///         }),
///         (2, TotalDamageAggregateForEachSkill {
///             total_raw_damage: 500.0,
///             total_rdps_contribution: HashMap::from([(status_key1.clone(), 50.0), (status_key2.clone(), 60.0), (status_key3.clone(), 70.0), (status_key4.clone(), 80.0)]),
///             cast_count: 5
///         }),
///     ]),
///     HashMap::from([
///         (3, TotalDamageAggregateForEachSkill {
///             total_raw_damage: 400.0,
///             total_rdps_contribution: HashMap::from([(status_key1.clone(), 10.0), (status_key2.clone(), 20.0), (status_key3.clone(), 30.0), (status_key4.clone(), 40.0)]),
///             cast_count: 5
///         }),
///         (4, TotalDamageAggregateForEachSkill {
///             total_raw_damage: 500.0,
///             total_rdps_contribution: HashMap::from([(status_key1.clone(), 50.0), (status_key2.clone(), 60.0), (status_key3.clone(), 70.0), (status_key4.clone(), 80.0)]),
///             cast_count: 5
///         }),
///     ])
/// ];
///
/// let status_damages = aggregate_status_damages(&skill_damage_tables);
///
/// assert_eq!(status_damages.len(), 2);
/// assert_eq!(status_damages[0].get(&402).unwrap().total_raw_damage, 10.0 + 50.0);
/// assert_eq!(status_damages[0].get(&402).unwrap().total_raid_damage, 0.0);
/// assert_eq!(status_damages[0].get(&401).unwrap().total_raw_damage, 40.0 + 80.0);
/// assert_eq!(status_damages[0].get(&401).unwrap().total_raid_damage, 0.0);
///
/// assert_eq!(status_damages[0].get(&102).unwrap().total_raw_damage, 0.0);
/// assert_eq!(status_damages[0].get(&102).unwrap().total_raid_damage, 20.0 + 60.0);
/// assert_eq!(status_damages[0].get(&103).unwrap().total_raw_damage, 0.0);
/// assert_eq!(status_damages[0].get(&103).unwrap().total_raid_damage, 30.0 + 70.0);
///
/// assert_eq!(status_damages[1].get(&402).unwrap().total_raw_damage, 0.0);
/// assert_eq!(status_damages[1].get(&402).unwrap().total_raid_damage, 10.0 + 50.0);
/// assert_eq!(status_damages[1].get(&401).unwrap().total_raw_damage, 0.0);
/// assert_eq!(status_damages[1].get(&401).unwrap().total_raid_damage, 40.0 + 80.0);
///
/// assert_eq!(status_damages[1].get(&102).unwrap().total_raw_damage, 20.0 + 60.0);
/// assert_eq!(status_damages[1].get(&102).unwrap().total_raid_damage, 0.0);
/// assert_eq!(status_damages[1].get(&103).unwrap().total_raw_damage, 30.0 + 70.0);
/// assert_eq!(status_damages[1].get(&103).unwrap().total_raid_damage, 0.0);
/// ```
pub fn aggregate_status_damages(
    skill_damage_tables: &[HashMap<SkillIdType, TotalDamageAggregateForEachSkill>],
) -> Vec<HashMap<StatusIdType, RaidbuffDamageAggregate>> {
    let mut status_damages = vec![];

    for _ in 0..skill_damage_tables.len() {
        status_damages.push(HashMap::new());
    }

    for (player_id, skill_damage_table) in skill_damage_tables.iter().enumerate() {
        for skill_damage in skill_damage_table.values() {
            for rdps_contribution in skill_damage.total_rdps_contribution.iter() {
                let status_id = rdps_contribution.0.status_id;
                let raidbuff_owner_player_id = rdps_contribution.0.player_id;

                let contributed_damage = rdps_contribution.1;
                let entry =
                    status_damages[player_id]
                        .entry(status_id)
                        .or_insert(RaidbuffDamageAggregate {
                            total_raw_damage: 0.0,
                            total_raid_damage: 0.0,
                        });

                if raidbuff_owner_player_id == (player_id as PlayerIdType) {
                    entry.total_raw_damage += *contributed_damage;
                } else {
                    entry.total_raid_damage += contributed_damage;
                }
            }
        }
    }

    status_damages
}

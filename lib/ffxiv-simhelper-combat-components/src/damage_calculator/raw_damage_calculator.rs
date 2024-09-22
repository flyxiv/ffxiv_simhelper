use crate::damage_calculator::{
    DamageRdpsProfile, DAMAGE_BASE, DIRECT_HIT_DAMAGE_MULTIPLIER, INCREASE_BASE, MULTIPLIER_BASE,
    ONE_HUNDRED_PERCENT,
};
use crate::event_ticker::PercentType;
use crate::live_objects::player::player_power::PlayerPower;
use crate::live_objects::player::StatusKey;
use crate::skill::damage_category::DamageCategory;
use crate::types::{MultiplierType, PlayerIdType, PotencyType, SnapshotTable};
use log::debug;
use rand::{random, thread_rng, Rng};
use std::collections::HashMap;

/// Translates a player's skill potency to expected damage number.
/// Depending on the player's power, the skill's potency, and whether there is a
/// Guaranteed Critical Hit or Direct Hit buff + trait multiplier.
pub trait RawDamageCalculator {
    fn calculate_total_damage(
        &self,
        player_id: PlayerIdType,
        potency: PotencyType,
        damage_category: DamageCategory,
        trait_percent: PercentType,
        is_guaranteed_critical_hit: bool,
        is_guaranteed_direct_hit: bool,
        snapshotted_status: &SnapshotTable,
        player_power: &PlayerPower,
    ) -> (DamageRdpsProfile, bool) {
        let potency = potency as MultiplierType;
        debug_assert!(potency >= DAMAGE_BASE, "{}", potency);
        let mut base_damage = calculate_base_damage(
            potency,
            trait_percent,
            damage_category,
            player_power,
            is_guaranteed_direct_hit,
        );

        for snapshot_info in snapshotted_status.values() {
            if snapshot_info.owner_player_id == player_id {
                let damage_multiplier = snapshot_info.get_damage_multiplier(
                    is_guaranteed_critical_hit,
                    is_guaranteed_direct_hit,
                    player_power.critical_strike_damage,
                );

                if damage_multiplier > MULTIPLIER_BASE {
                    base_damage *= damage_multiplier;
                }
            }
        }

        let (crit_direct_hit_calculation_result, mut contribution_table, is_crit) =
            calculate_crit_direct_hit_damage_direct_damage(
                player_id,
                base_damage,
                snapshotted_status,
                player_power,
                is_guaranteed_critical_hit,
                is_guaranteed_direct_hit,
                damage_category,
            );

        let mut final_damage_multiplier = MULTIPLIER_BASE;
        let mut final_damage = crit_direct_hit_calculation_result;

        for snapshot in snapshotted_status.values() {
            if snapshot.owner_player_id == player_id {
                continue;
            }
            let damage_multiplier = snapshot.get_damage_multiplier(
                is_guaranteed_critical_hit,
                is_guaranteed_direct_hit,
                player_power.critical_strike_damage,
            );

            if damage_multiplier > MULTIPLIER_BASE {
                final_damage_multiplier *= damage_multiplier;
                final_damage *= damage_multiplier;
            }
        }

        let damage_buff_contribution = final_damage - crit_direct_hit_calculation_result;
        let final_damage_multiplier_log = MultiplierType::log10(final_damage_multiplier);

        for value in contribution_table.values_mut() {
            let adjusted_crit_dh_buff_contribution = *value * final_damage_multiplier;
            final_damage += adjusted_crit_dh_buff_contribution - *value;
            *value = adjusted_crit_dh_buff_contribution;
        }

        for (&key, snapshot) in snapshotted_status.iter() {
            if snapshot.owner_player_id == player_id {
                continue;
            }

            let damage_multiplier = snapshot.get_damage_multiplier(
                is_guaranteed_critical_hit,
                is_guaranteed_direct_hit,
                player_power.critical_strike_damage,
            );

            if damage_multiplier > MULTIPLIER_BASE {
                let buff_damage_portion =
                    MultiplierType::log10(damage_multiplier) / final_damage_multiplier_log;
                let buff_damage_contribution = damage_buff_contribution * buff_damage_portion;

                let entry = contribution_table
                    .entry(StatusKey::new(key.status_id, snapshot.owner_player_id))
                    .or_insert(DAMAGE_BASE);
                *entry += buff_damage_contribution;
            }
        }

        let mut raw_damage = final_damage;

        for &value in contribution_table.values() {
            raw_damage -= value;
        }

        let damage_rdps_profile = DamageRdpsProfile {
            raw_damage,
            final_damage,
            rdps_contribution: contribution_table,
        };

        debug!(
            "Player {}'s skill {} has raw damage {} and final damage {}",
            player_id, player_id, damage_rdps_profile.raw_damage, damage_rdps_profile.final_damage,
        );
        (damage_rdps_profile, is_crit)
    }
}

/// https://www.akhmorning.com/allagan-studies/how-to-be-a-math-wizard/shadowbringers/damage-and-healing/#damage-over-time
/// Variables use names from the calculations of the above link.
fn calculate_base_damage(
    potency: MultiplierType,
    trait_percent: PercentType,
    damage_category: DamageCategory,
    player_power: &PlayerPower,
    is_guaranteed_direct_hit: bool,
) -> MultiplierType {
    let determination_multiplier = if is_guaranteed_direct_hit {
        // https://www.akhmorning.com/allagan-studies/stats/dh/#formulae
        // Auto DH is added to determination multiplier
        player_power.determination_multiplier + player_power.auto_direct_hit_increase
    } else {
        player_power.determination_multiplier
    };
    match damage_category {
        DamageCategory::Direct => {
            let d1 = MultiplierType::floor(
                potency * player_power.main_stat_multiplier * determination_multiplier,
            );
            debug_assert!(d1 >= potency, "{}", d1);

            let mut d2 = MultiplierType::floor(d1 * player_power.tenacity_multiplier);
            d2 = MultiplierType::floor(d2 * player_power.weapon_damage_multiplier);
            d2 = MultiplierType::floor(d2 * trait_percent as MultiplierType);

            debug_assert!(d2 >= d1, "{}", d1);
            MultiplierType::floor(d2 / ONE_HUNDRED_PERCENT)
        }
        DamageCategory::PhysicalDot => {
            let d1 = MultiplierType::floor(
                potency * player_power.main_stat_multiplier * determination_multiplier,
            );
            let mut d2 = MultiplierType::floor(d1 * player_power.tenacity_multiplier);
            d2 = MultiplierType::floor(d2 * player_power.speed_multiplier);
            d2 = MultiplierType::floor(d2 * player_power.weapon_damage_multiplier);
            d2 = MultiplierType::floor(d2 * trait_percent as MultiplierType);
            MultiplierType::floor(d2 / ONE_HUNDRED_PERCENT) + 1.0
        }
        DamageCategory::MagicalDot => {
            let mut d1 = MultiplierType::floor(potency * player_power.weapon_damage_multiplier);
            d1 = MultiplierType::floor(d1 * player_power.main_stat_multiplier);
            d1 = MultiplierType::floor(d1 * player_power.speed_multiplier);

            let mut d2 = MultiplierType::floor(d1 * determination_multiplier);
            d2 = MultiplierType::floor(d2 * trait_percent as MultiplierType);
            MultiplierType::floor(d2 / ONE_HUNDRED_PERCENT) + 1.0
        }
        DamageCategory::AutoAttack => {
            let d1 = MultiplierType::floor(
                potency * player_power.main_stat_multiplier * determination_multiplier,
            );

            let mut d2 = MultiplierType::floor(d1 * player_power.tenacity_multiplier);
            d2 = MultiplierType::floor(d2 * player_power.speed_multiplier);
            d2 = MultiplierType::floor(d2 * player_power.auto_attack_delays / 3.0);
            d2 = MultiplierType::floor(d2 * player_power.weapon_damage_multiplier);
            d2 = MultiplierType::floor(d2 * trait_percent as MultiplierType);
            MultiplierType::floor(d2 / ONE_HUNDRED_PERCENT)
        }
    }
}

fn get_damage_variance_multiplier() -> MultiplierType {
    let mut damage_random: MultiplierType = random::<MultiplierType>() - 0.5;
    damage_random = damage_random / 10.0 + MULTIPLIER_BASE;
    damage_random
}

fn calculate_crit_direct_hit_damage_direct_damage(
    player_id: PlayerIdType,
    base_damage: MultiplierType,
    snapshotted_status: &SnapshotTable,
    player_power: &PlayerPower,
    is_guaranteed_critical_hit: bool,
    is_guaranteed_direct_hit: bool,
    damage_category: DamageCategory,
) -> (MultiplierType, HashMap<StatusKey, MultiplierType>, bool) {
    let damage_variance: MultiplierType = get_damage_variance_multiplier();

    debug_assert!(
        damage_variance >= 0.95 && damage_variance <= 1.05,
        "{}",
        damage_variance
    );

    let damage_before_crit_direct_hit = match damage_category {
        DamageCategory::Direct | DamageCategory::AutoAttack => base_damage,
        DamageCategory::PhysicalDot | DamageCategory::MagicalDot => base_damage * damage_variance,
    };

    let critical_hit_rate =
        get_final_critical_hit_rate(snapshotted_status, is_guaranteed_critical_hit, player_power);
    let direct_hit_rate =
        get_final_direct_hit_rate(snapshotted_status, is_guaranteed_direct_hit, player_power);

    let crit_rng = thread_rng().gen_range(0..1000) as MultiplierType / 1000.0;
    let direct_hit_rng = thread_rng().gen_range(0..1000) as MultiplierType / 1000.0;
    let is_crit = crit_rng < critical_hit_rate;
    let is_direct_hit = direct_hit_rng < direct_hit_rate;
    let mut contribution_board: HashMap<StatusKey, MultiplierType> = HashMap::new();

    let crit_multiplier = if is_crit {
        player_power.critical_strike_damage
    } else {
        1.0
    };
    let dh_multiplier = if is_direct_hit {
        DIRECT_HIT_DAMAGE_MULTIPLIER
    } else {
        1.0
    };
    let crit_dh_multiplier = crit_multiplier * dh_multiplier;

    if crit_dh_multiplier == MULTIPLIER_BASE {
        let final_damage = match damage_category {
            DamageCategory::Direct | DamageCategory::AutoAttack => {
                damage_before_crit_direct_hit * damage_variance
            }
            DamageCategory::PhysicalDot | DamageCategory::MagicalDot => {
                damage_before_crit_direct_hit
            }
        };

        return (final_damage, contribution_board, false);
    }

    let damage_after_crit_direct_hit =
        MultiplierType::floor(damage_before_crit_direct_hit * crit_dh_multiplier);

    let crit_probability_multiplier = if is_guaranteed_critical_hit {
        MULTIPLIER_BASE
    } else {
        crit_multiplier
    };
    let dh_probability_multiplier = if is_guaranteed_direct_hit {
        MULTIPLIER_BASE
    } else {
        dh_multiplier
    };
    let crit_dh_probability_multiplier = crit_probability_multiplier * dh_probability_multiplier;

    let crit_dh_contribution = damage_after_crit_direct_hit
        - damage_after_crit_direct_hit / crit_dh_probability_multiplier;

    let crit_portion = MultiplierType::log10(crit_probability_multiplier)
        / MultiplierType::log10(crit_dh_probability_multiplier);
    let dh_portion = MultiplierType::log10(dh_probability_multiplier)
        / MultiplierType::log10(crit_dh_probability_multiplier);

    if crit_probability_multiplier > MULTIPLIER_BASE && !is_guaranteed_critical_hit {
        let crit_contribution = crit_portion * crit_dh_contribution;

        for (&key, snapshot_info) in snapshotted_status.iter() {
            let status_critical_rate =
                snapshot_info.get_critical_strike_rate_increase(is_guaranteed_critical_hit);

            if player_id != snapshot_info.owner_player_id && status_critical_rate > INCREASE_BASE {
                let skill_portion = status_critical_rate / critical_hit_rate;

                let entry = contribution_board
                    .entry(StatusKey::new(key.status_id, snapshot_info.owner_player_id))
                    .or_insert(DAMAGE_BASE);

                *entry += crit_contribution * skill_portion;
            }
        }
    }

    if dh_multiplier > 1.0 && !is_guaranteed_direct_hit {
        let dh_contribution = dh_portion * crit_dh_contribution;

        for (&key, snapshot_info) in snapshotted_status.iter() {
            let buff_direct_hit_rate =
                snapshot_info.get_direct_hit_rate_increase(is_guaranteed_direct_hit);

            if player_id != snapshot_info.owner_player_id && buff_direct_hit_rate > 0.0 {
                let skill_portion = buff_direct_hit_rate / direct_hit_rate;

                let entry = contribution_board
                    .entry(StatusKey::new(key.status_id, snapshot_info.owner_player_id))
                    .or_insert(DAMAGE_BASE);

                *entry += dh_contribution * skill_portion;
            }
        }
    }

    (
        damage_after_crit_direct_hit * damage_variance,
        contribution_board,
        is_crit,
    )
}

fn get_final_critical_hit_rate(
    snapshotted_status: &SnapshotTable,
    is_guaranteed_crit: bool,
    player_power: &PlayerPower,
) -> MultiplierType {
    if is_guaranteed_crit {
        return 1.0;
    }
    let mut final_critical_hit_rate = player_power.critical_strike_rate;

    for status in snapshotted_status.values() {
        final_critical_hit_rate += status.get_critical_strike_rate_increase(is_guaranteed_crit);
    }

    final_critical_hit_rate
}

fn get_final_direct_hit_rate(
    status_infos: &SnapshotTable,
    is_guaranteed_direct_hit: bool,
    player_power: &PlayerPower,
) -> MultiplierType {
    if is_guaranteed_direct_hit {
        return 1.0;
    }
    let mut final_direct_hit_rate = player_power.direct_hit_rate;

    for snapshot_info in status_infos.values() {
        final_direct_hit_rate +=
            snapshot_info.get_direct_hit_rate_increase(is_guaranteed_direct_hit);
    }

    final_direct_hit_rate
}

pub struct FfxivRawDamageCalculator {}

impl RawDamageCalculator for FfxivRawDamageCalculator {}

impl Default for FfxivRawDamageCalculator {
    fn default() -> Self {
        FfxivRawDamageCalculator {}
    }
}

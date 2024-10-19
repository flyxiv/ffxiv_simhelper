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

#[inline]
fn get_crit_direct_hit_rng() -> MultiplierType {
    thread_rng().gen_range(0..1000) as MultiplierType / 1000.0
}

#[inline]
fn decide_crit_direct_hit(
    critical_hit_rate: MultiplierType,
    direct_hit_rate: MultiplierType,
) -> (bool, bool) {
    let crit_rng = get_crit_direct_hit_rng();
    let direct_hit_rng = get_crit_direct_hit_rng();

    (
        crit_rng < critical_hit_rate,
        direct_hit_rng < direct_hit_rate,
    )
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
    let mut damage_variance: MultiplierType = get_damage_variance_multiplier();

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

    let (is_crit, is_direct_hit) = decide_crit_direct_hit(critical_hit_rate, direct_hit_rate);

    calculate_crit_direct_hit_result(
        player_id,
        damage_before_crit_direct_hit,
        damage_variance,
        is_crit,
        is_direct_hit,
        is_guaranteed_critical_hit,
        is_guaranteed_direct_hit,
        damage_category,
        snapshotted_status,
        player_power,
        critical_hit_rate,
        direct_hit_rate,
    )
}

fn calculate_crit_direct_hit_result(
    player_id: PlayerIdType,
    damage_before_crit_direct_hit: MultiplierType,
    damage_variance: MultiplierType,
    is_crit: bool,
    is_direct_hit: bool,
    is_guaranteed_critical_hit: bool,
    is_guaranteed_direct_hit: bool,
    damage_category: DamageCategory,
    snapshotted_status: &SnapshotTable,
    player_power: &PlayerPower,
    critical_hit_rate: MultiplierType,
    direct_hit_rate: MultiplierType,
) -> (MultiplierType, HashMap<StatusKey, MultiplierType>, bool) {
    let mut contribution_board: HashMap<StatusKey, MultiplierType> = HashMap::new();

    let crit_multiplier = if is_crit {
        player_power.critical_strike_damage
    } else {
        MULTIPLIER_BASE
    };
    let dh_multiplier = if is_direct_hit {
        DIRECT_HIT_DAMAGE_MULTIPLIER
    } else {
        MULTIPLIER_BASE
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

    if dh_multiplier > MULTIPLIER_BASE && !is_guaranteed_direct_hit {
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

#[cfg(test)]
mod tests {
    use crate::{
        damage_calculator::raw_damage_calculator::{calculate_base_damage, decide_crit_direct_hit},
        skill::damage_category::DamageCategory,
    };
    use std::collections::HashMap;
    use crate::live_objects::player::player_power::PlayerPower;
    use super::get_damage_variance_multiplier;

    #[test]
    fn damage_variance_multiplier_test() {
        // Check that the damage variance of +- 5% is distributed evenly
        let number_of_samples = 100000;
        let uniform_number_per_key = number_of_samples / 100;
        let number_per_key_lower_bound = uniform_number_per_key - 200;
        let number_per_key_upper_bound = uniform_number_per_key + 200;

        let mut variance_map = HashMap::new();

        for _ in 0..number_of_samples {
            let entry = variance_map
                .entry((get_damage_variance_multiplier() * 1000.0) as i32)
                .or_insert(0);

            *entry = *entry + 1;
        }

        for (_, &value) in variance_map.iter() {
            assert!(
                value > number_per_key_lower_bound && value < number_per_key_upper_bound,
                "get_damage_variance_multiplier() doesn't give a uniform distribution: {:?}",
                variance_map
            );
        }
    }

    #[test]
    fn crit_direct_hit_rng_test() {
        let number_of_samples = 100000;

        let crit_percent = 0.255;
        let direct_hit_percent = 0.455;

        let valid_crit_count = (number_of_samples as f64 * crit_percent) as i32;
        let crit_count_lower_bound = valid_crit_count - 500;
        let crit_count_upper_bound = valid_crit_count + 500;

        let valid_direct_hit_count = (number_of_samples as f64 * direct_hit_percent) as i32;
        let direct_hit_count_lower_bound = valid_direct_hit_count - 500;
        let direct_hit_count_upper_bound = valid_direct_hit_count + 500;

        let valid_crit_direct_hit_count =
            (number_of_samples as f64 * crit_percent * direct_hit_percent) as i32;
        let crit_direct_hit_count_lower_bound = valid_crit_direct_hit_count - 500;
        let crit_direct_hit_count_upper_bound = valid_crit_direct_hit_count + 500;

        let mut critical_hit_count = 0;
        let mut direct_hit_count = 0;
        let mut critical_direct_hit_count = 0;

        for _ in 0..number_of_samples {
            let (is_crit, is_direct_hit) = decide_crit_direct_hit(crit_percent, direct_hit_percent);

            if is_crit {
                critical_hit_count += 1;
            }

            if is_direct_hit {
                direct_hit_count += 1;
            }

            if is_crit && is_direct_hit {
                critical_direct_hit_count += 1;
            }
        }

        assert!(
            critical_hit_count > crit_count_lower_bound
                && critical_hit_count < crit_count_upper_bound,
            "Critical hit rate is not within the expected range: {}",
            critical_hit_count
        );

        assert!(
            direct_hit_count > direct_hit_count_lower_bound
                && direct_hit_count < direct_hit_count_upper_bound,
            "Direct hit rate is not within the expected range: {}",
            direct_hit_count
        );

        assert!(
            critical_direct_hit_count > crit_direct_hit_count_lower_bound
                && critical_direct_hit_count < crit_direct_hit_count_upper_bound,
            "Critical hit and direct hit rate is not within the expected range: {}",
            critical_direct_hit_count
        );
    }

    #[test]
    fn direct_damage_base_damage_test() {
        let potency = 310.0;
        let trait_percent = 130;
        let player_power = crate::live_objects::player::player_power::PlayerPower {
            auto_attack_delays: 3.0,
            critical_strike_rate: 0.15,
            critical_strike_damage: 1.5,
            direct_hit_rate: 0.23,
            auto_direct_hit_increase: 0.4,
            determination_multiplier: 1.124,
            tenacity_multiplier: 1.0,
            speed_multiplier: 1.26,
            weapon_damage_multiplier: 1.96,
            main_stat_multiplier: 24.65,
            weapon_damage: 132,
            main_stat: 3300,
            critical_strike: 2560,
            direct_hit: 2500,
            determination: 2500,
            skill_speed: 2500,
            tenacity: 400,
            spell_speed: 2500,
        };

        let base_damage_answer = 21500.0;
        let base_damage_answer_lower_bound = base_damage_answer * 0.95;
        let base_damage_answer_upper_bound = base_damage_answer * 1.05;

        let base_damage_direct_no_guarantee = calculate_base_damage(
            potency,
            trait_percent,
            DamageCategory::Direct,
            &player_power,
            false,
        );

        assert!(
            base_damage_direct_no_guarantee > base_damage_answer_lower_bound
                && base_damage_direct_no_guarantee < base_damage_answer_upper_bound,
            "base damage no guarantee: {}, lower bound: {}, upper bound: {}",
            base_damage_direct_no_guarantee,
            base_damage_answer_lower_bound,
            base_damage_answer_upper_bound
        );

        let base_damage_direct_guarantee = calculate_base_damage(
            potency,
            trait_percent,
            DamageCategory::Direct,
            &player_power,
            true,
        );

        assert!(
            base_damage_direct_guarantee > base_damage_answer_lower_bound * 1.4
                && base_damage_direct_guarantee < base_damage_answer_upper_bound * 1.4,
            "{}",
            base_damage_direct_guarantee
        );
    }

    #[test]
    fn magical_dot_base_damage_test() {
        let trait_percent = 130;
        let player_power = crate::live_objects::player::player_power::PlayerPower {
            auto_attack_delays: 3.0,
            critical_strike_rate: 0.15,
            critical_strike_damage: 1.5,
            direct_hit_rate: 0.23,
            auto_direct_hit_increase: 0.4,
            determination_multiplier: 1.124,
            tenacity_multiplier: 1.0,
            speed_multiplier: 1.26,
            weapon_damage_multiplier: 1.96,
            main_stat_multiplier: 24.65,
            weapon_damage: 132,
            main_stat: 3300,
            critical_strike: 2560,
            direct_hit: 2500,
            determination: 2500,
            skill_speed: 2500,
            tenacity: 400,
            spell_speed: 2500,
        };

        let potency_magical_dot = 75.0;
        let base_damage_magical_dot = 5170.0 * player_power.speed_multiplier;
        let base_damage_magical_dot_lower_bound = base_damage_magical_dot * 0.95;
        let base_damage_magical_dot_upper_bound = base_damage_magical_dot * 1.05;

        let base_damage_magical_dot = calculate_base_damage(
            potency_magical_dot,
            trait_percent,
            DamageCategory::MagicalDot,
            &player_power,
            false,
        );

        assert!(
            base_damage_magical_dot > base_damage_magical_dot * 0.95
                && base_damage_magical_dot < base_damage_magical_dot * 1.05,
            "base_damage_magical_dot: {}, lower bound: {}, upper bound: {}",
            base_damage_magical_dot,
            base_damage_magical_dot_lower_bound,
            base_damage_magical_dot_upper_bound
        );
    }

    #[test]
    fn physical_dot_damage_test() {
        let player_power_physical = crate::live_objects::player::player_power::PlayerPower {
            auto_attack_delays: 2.64,
            critical_strike_rate: 0.176,
            critical_strike_damage: 1.526,
            direct_hit_rate: 0.234,
            auto_direct_hit_increase: 0.4,
            determination_multiplier: 1.085,
            tenacity_multiplier: 1.0,
            speed_multiplier: 1.025,
            weapon_damage_multiplier: 1.86,
            main_stat_multiplier: 21.44,
            weapon_damage: 132,
            main_stat: 3300,
            critical_strike: 2560,
            direct_hit: 2500,
            determination: 2500,
            skill_speed: 2500,
            tenacity: 400,
            spell_speed: 2500,
        };

        let potency_physical_dot = 50.0;
        let melee_trait = 100;

        let base_damage_physical_dot = calculate_base_damage(
            potency_physical_dot,
            melee_trait,
            DamageCategory::PhysicalDot,
            &player_power_physical,
            false,
        );

        let base_damage_answer_physical_dot = 2170.0;
        let base_damage_answer_physical_dot_lower_bound = base_damage_answer_physical_dot * 0.95;
        let base_damage_answer_physical_dot_upper_bound = base_damage_answer_physical_dot * 1.05;

        assert!(
            base_damage_physical_dot > base_damage_answer_physical_dot_lower_bound
                && base_damage_physical_dot < base_damage_answer_physical_dot_upper_bound,
            "base_damage_physical_dot: {}, lower bound: {}, upper bound: {}",
            base_damage_physical_dot,
            base_damage_answer_physical_dot_lower_bound,
            base_damage_answer_physical_dot_upper_bound
        );
    }

    #[test]
    fn auto_attack_base_damage_test() {
        let player_power_physical = crate::live_objects::player::player_power::PlayerPower {
            auto_attack_delays: 2.64,
            critical_strike_rate: 0.176,
            critical_strike_damage: 1.526,
            direct_hit_rate: 0.234,
            auto_direct_hit_increase: 0.4,
            determination_multiplier: 1.085,
            tenacity_multiplier: 1.0,
            speed_multiplier: 1.025,
            weapon_damage_multiplier: 1.86,
            main_stat_multiplier: 21.44,
            weapon_damage: 132,
            main_stat: 3300,
            critical_strike: 2560,
            direct_hit: 2500,
            determination: 2500,
            skill_speed: 2500,
            tenacity: 400,
            spell_speed: 2500,
        };

        // samurai auto attack
        let potency_auto_attack = 90.0;
        let melee_trait = 100;

        let base_damage_auto_attack = calculate_base_damage(
            potency_auto_attack,
            melee_trait,
            DamageCategory::AutoAttack,
            &player_power_physical,
            false,
        );

        let base_damage_answer_auto_attack = 3450.0;
        let base_damage_answer_auto_attack_lower_bound = base_damage_answer_auto_attack * 0.95;
        let base_damage_answer_auto_attack_upper_bound = base_damage_answer_auto_attack * 1.05;

        assert!(
            base_damage_auto_attack > base_damage_answer_auto_attack_lower_bound
                && base_damage_auto_attack < base_damage_answer_auto_attack_upper_bound,
            "base_damage_auto_attack: {}, lower bound: {}, upper bound: {}",
            base_damage_auto_attack,
            base_damage_answer_auto_attack_lower_bound,
            base_damage_answer_auto_attack_upper_bound
        );
    }

    #[test]
    fn ast_dot_base_damage_test() {
        // ref. https://github.com/flyxiv/ffxiv_simhelper_public/issues/35
        // AST 2.49 GCD and 2.50 GCD differs DOT damage by 5%
        // make sure the dot ticks are similar, since speed multiplier increase of 0.1% won't give that much buff.

        let ast_250_gcd = PlayerPower {
            auto_attack_delays: 2.5,
            critical_strike_rate: 0.23,
            critical_strike_damage: 1.58,
            direct_hit_rate: 0.146,
            auto_direct_hit_increase: 0.4,
            determination_multiplier: 1.131,
            tenacity_multiplier: 1.0,
            speed_multiplier: 1.00,
            weapon_damage_multiplier: 1.96,
            main_stat_multiplier: 24.72,
            weapon_damage: 132,
            main_stat: 3300,
            critical_strike: 2560,
            direct_hit: 2500,
            determination: 2500,
            skill_speed: 2500,
            tenacity: 400,
            spell_speed: 2500,
        };

        let ast_249_gcd = PlayerPower {
            auto_attack_delays: 2.5,
            critical_strike_rate: 0.23,
            critical_strike_damage: 1.58,
            direct_hit_rate: 0.138,
            auto_direct_hit_increase: 0.4,
            determination_multiplier: 1.131,
            tenacity_multiplier: 1.0,
            speed_multiplier: 1.001,
            weapon_damage_multiplier: 1.96,
            main_stat_multiplier: 24.72,
            weapon_damage: 132,
            main_stat: 3300,
            critical_strike: 2560,
            direct_hit: 2500,
            determination: 2500,
            skill_speed: 2500,
            tenacity: 400,
            spell_speed: 2500,
        };

        let combust_iii_base_damage_250 = calculate_base_damage(
            70.0,
            130,
            DamageCategory::MagicalDot,
            &ast_250_gcd,
            false,
        );

        let combust_iii_base_damage_249 = calculate_base_damage(
            70.0,
            130,
            DamageCategory::MagicalDot,
            &ast_249_gcd,
            false,
        );

        assert_eq!(combust_iii_base_damage_249, combust_iii_base_damage_250);
    }
}

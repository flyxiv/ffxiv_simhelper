use crate::damage_calculator::multiplier_calculator::{
    MultiplierCalculator, DIRECT_HIT_DAMAGE_MULTIPLIER,
};
use crate::damage_calculator::DamageRdpsProfile;
use crate::event_ticker::PercentType;
use crate::id_entity::IdEntity;
use crate::live_objects::player::player_power::PlayerPower;
use crate::live_objects::player::StatusKey;
use crate::owner_tracker::OwnerTracker;
use crate::skill::damage_category::DamageCategory;
use crate::status::buff_status::BuffStatus;
use crate::status::debuff_status::DebuffStatus;
use crate::status::Status;
use crate::types::{DamageType, IdType, MultiplierType};
use log::debug;
use rand::{random, thread_rng, Rng};
use std::collections::HashMap;

pub const ONE_HUNDRED_PERCENT: f64 = 100.0;

/// Translates a player's skill potency to expected damage number.
/// Depending on the player's power, the skill's potency, and whether there is a
/// Guaranteed Critical Hit or Direct Hit buff + trait multiplier.
pub trait RawDamageCalculator: MultiplierCalculator {
    fn calculate_total_damage(
        &self,
        player_id: IdType,
        potency: DamageType,
        damage_category: DamageCategory,
        trait_percent: PercentType,
        is_guaranteed_critical_hit: bool,
        is_guaranteed_direct_hit: bool,
        buffs: &HashMap<StatusKey, BuffStatus>,
        debuffs: &HashMap<StatusKey, DebuffStatus>,
        player_power: &PlayerPower,
    ) -> (DamageRdpsProfile, bool) {
        let potency = potency as MultiplierType;
        debug_assert!(potency >= 0.0, "{}", potency);
        let base_damage =
            calculate_base_damage(potency, trait_percent, damage_category, player_power);

        let (crit_direct_hit_calculation_result, mut contribution_table, is_crit) =
            calculate_crit_direct_hit_damage(
                player_id,
                base_damage,
                buffs,
                debuffs,
                player_power,
                damage_category,
                is_guaranteed_critical_hit,
                is_guaranteed_direct_hit,
            );

        let mut final_damage_multiplier = 1.0;
        let mut final_damage = crit_direct_hit_calculation_result;

        for buff in buffs.values() {
            let damage_increase = buff.get_damage_increase(
                is_guaranteed_critical_hit,
                is_guaranteed_direct_hit,
                player_power.critical_strike_damage,
            );

            if damage_increase > 0.0 {
                let damage_multiplier = 1.0 + damage_increase;
                final_damage_multiplier *= damage_multiplier;
                final_damage *= damage_multiplier;
            }
        }

        for debuff in debuffs.values() {
            let damage_increase = debuff.get_damage_increase(
                is_guaranteed_critical_hit,
                is_guaranteed_direct_hit,
                player_power.critical_strike_damage,
            );

            if damage_increase > 0.0 {
                let damage_multiplier = 1.0 + damage_increase;
                final_damage_multiplier *= damage_multiplier;
                final_damage *= damage_multiplier;
            }
        }

        let damage_buff_contribution = final_damage - crit_direct_hit_calculation_result;

        for buff in buffs.values() {
            let damage_increase = buff.get_damage_increase(
                is_guaranteed_critical_hit,
                is_guaranteed_direct_hit,
                player_power.critical_strike_damage,
            );

            if damage_increase > 0.0 && buff.get_owner_id() != player_id {
                let damage_multiplier = 1.0 + damage_increase;
                let buff_damage_portion = MultiplierType::log10(damage_multiplier)
                    / MultiplierType::log10(final_damage_multiplier);
                let buff_damage_contribution = damage_buff_contribution * buff_damage_portion;

                let entry = contribution_table
                    .entry(StatusKey::new(buff.get_id(), buff.get_owner_id()))
                    .or_insert(0.0);
                *entry += buff_damage_contribution;
            }
        }

        for debuff in debuffs.values() {
            let damage_increase = debuff.get_damage_increase(
                is_guaranteed_critical_hit,
                is_guaranteed_direct_hit,
                player_power.critical_strike_damage,
            );

            if damage_increase > 0.0 && debuff.get_owner_id() != player_id {
                let damage_multiplier = 1.0 + damage_increase;
                let buff_damage_portion = MultiplierType::log10(damage_multiplier)
                    / MultiplierType::log10(final_damage_multiplier);
                let buff_damage_contribution = damage_buff_contribution * buff_damage_portion;

                let entry = contribution_table
                    .entry(StatusKey::new(debuff.get_id(), debuff.get_owner_id()))
                    .or_insert(0.0);
                *entry += buff_damage_contribution;
            }
        }

        let mut damage_rdps_profile = DamageRdpsProfile {
            skill_id: player_id,
            raw_damage: final_damage,
            final_damage,
            rdps_contribution: contribution_table,
        };

        for value in damage_rdps_profile.rdps_contribution.values() {
            if *value > 0.0 {
                damage_rdps_profile.raw_damage -= *value;
            }
        }

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
) -> f64 {
    match damage_category {
        DamageCategory::Direct => {
            let d1 = MultiplierType::floor(
                potency
                    * player_power.main_stat_multiplier
                    * player_power.determination_damage_multiplier,
            );
            debug_assert!(d1 >= potency, "{}", d1);

            let mut d2 = MultiplierType::floor(d1 * player_power.tenacity_damage_multiplier);
            d2 = MultiplierType::floor(d2 * player_power.weapon_damage_multiplier);
            d2 = MultiplierType::floor(d2 * trait_percent as MultiplierType);

            debug_assert!(d2 >= d1, "{}", d1);
            MultiplierType::floor(d2 / 100.0)
        }
        DamageCategory::PhysicalDot => {
            let d1 = MultiplierType::floor(
                potency
                    * player_power.main_stat_multiplier
                    * player_power.determination_damage_multiplier,
            );
            let mut d2 = MultiplierType::floor(d1 * player_power.tenacity_damage_multiplier);
            d2 = MultiplierType::floor(d2 * player_power.speed_multiplier);
            d2 = MultiplierType::floor(d2 * player_power.weapon_damage_multiplier);
            d2 = MultiplierType::floor(d2 * trait_percent as MultiplierType);
            MultiplierType::floor(d2 / 100.0) + 1.0
        }
        DamageCategory::MagicalDot => {
            let mut d1 = MultiplierType::floor(potency * player_power.weapon_damage_multiplier);
            d1 = MultiplierType::floor(d1 * player_power.main_stat_multiplier);
            d1 = MultiplierType::floor(d1 * player_power.speed_multiplier);

            let mut d2 = MultiplierType::floor(d1 * player_power.determination_damage_multiplier);
            d2 = MultiplierType::floor(d2 * trait_percent as MultiplierType);
            MultiplierType::floor(d2 / 100.0) + 1.0
        }
        DamageCategory::AutoAttack => {
            let d1 = MultiplierType::floor(
                potency
                    * player_power.main_stat_multiplier
                    * player_power.determination_damage_multiplier,
            );

            let mut d2 = MultiplierType::floor(d1 * player_power.tenacity_damage_multiplier);
            d2 = MultiplierType::floor(d2 * player_power.speed_multiplier);
            d2 = MultiplierType::floor(d2 * player_power.auto_attack_delays / 3.0);
            d2 = MultiplierType::floor(d2 * trait_percent as MultiplierType);
            MultiplierType::floor(d2 / 100.0)
        }
    }
}

fn calculate_crit_direct_hit_damage(
    player_id: IdType,
    base_damage: MultiplierType,
    buffs: &HashMap<StatusKey, BuffStatus>,
    debuffs: &HashMap<StatusKey, DebuffStatus>,
    player_power: &PlayerPower,
    damage_category: DamageCategory,
    is_guaranteed_critical_hit: bool,
    is_guaranteed_direct_hit: bool,
) -> (MultiplierType, HashMap<StatusKey, MultiplierType>, bool) {
    let mut damage_random: MultiplierType = random::<MultiplierType>() - 0.5;
    damage_random = damage_random / 100.0 + 1.0;
    let damage_before_crit_direct_hit = match damage_category {
        DamageCategory::Direct => base_damage,
        DamageCategory::PhysicalDot => base_damage * damage_random,
        DamageCategory::MagicalDot => base_damage * damage_random,
        DamageCategory::AutoAttack => base_damage,
    };

    let critical_hit_rate =
        get_final_critical_hit_rate(buffs, debuffs, is_guaranteed_critical_hit, player_power);
    let direct_hit_rate =
        get_final_direct_hit_rate(buffs, debuffs, is_guaranteed_direct_hit, player_power);

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

    if crit_dh_multiplier == 1.0 {
        return (damage_before_crit_direct_hit, contribution_board, false);
    }

    let damage_after_crit_direct_hit =
        MultiplierType::floor(damage_before_crit_direct_hit * crit_dh_multiplier);
    let crit_dh_contribution = damage_after_crit_direct_hit - damage_before_crit_direct_hit;

    let crit_portion =
        MultiplierType::log10(crit_multiplier) / MultiplierType::log10(crit_dh_multiplier);
    let dh_portion =
        MultiplierType::log10(dh_multiplier) / MultiplierType::log10(crit_dh_multiplier);

    if crit_multiplier > 1.0 && !is_guaranteed_critical_hit {
        let crit_contribution = crit_portion * crit_dh_contribution;

        for buff in buffs.values() {
            let buff_critical_rate =
                buff.get_critical_strike_rate_increase(is_guaranteed_direct_hit);

            if player_id != buff.get_owner_id() && buff_critical_rate > 0.0 {
                let skill_portion = buff_critical_rate / critical_hit_rate;

                let entry = contribution_board
                    .entry(StatusKey::new(buff.get_id(), buff.get_owner_id()))
                    .or_insert(0.0);

                *entry += crit_contribution * skill_portion;
            }
        }

        for debuff in debuffs.values() {
            let debuff_critical_rate =
                debuff.get_critical_strike_rate_increase(is_guaranteed_direct_hit);

            if player_id != debuff.get_owner_id() && debuff_critical_rate > 0.0 {
                let skill_portion = debuff_critical_rate / critical_hit_rate;

                let entry = contribution_board
                    .entry(StatusKey::new(debuff.get_id(), debuff.get_owner_id()))
                    .or_insert(0.0);

                *entry += crit_contribution * skill_portion;
            }
        }
    }

    if dh_multiplier > 1.0 && !is_guaranteed_direct_hit {
        let dh_contribution = dh_portion * crit_dh_contribution;

        for buff in buffs.values() {
            let buff_direct_hit_rate = buff.get_direct_hit_rate_increase(is_guaranteed_direct_hit);

            if player_id != buff.get_owner_id() && buff_direct_hit_rate > 0.0 {
                let skill_portion = buff_direct_hit_rate / direct_hit_rate;

                let entry = contribution_board
                    .entry(StatusKey::new(buff.get_id(), buff.get_owner_id()))
                    .or_insert(0.0);

                *entry += dh_contribution * skill_portion;
            }
        }

        for debuff in debuffs.values() {
            let debuff_direct_hit_rate =
                debuff.get_direct_hit_rate_increase(is_guaranteed_direct_hit);

            if player_id != debuff.get_owner_id() && debuff_direct_hit_rate > 0.0 {
                let skill_portion = debuff_direct_hit_rate / direct_hit_rate;

                let entry = contribution_board
                    .entry(StatusKey::new(debuff.get_id(), debuff.get_owner_id()))
                    .or_insert(0.0);

                *entry += dh_contribution * skill_portion;
            }
        }
    }

    let final_damage = match damage_category {
        DamageCategory::Direct => damage_after_crit_direct_hit * damage_random,
        DamageCategory::PhysicalDot => damage_after_crit_direct_hit,
        DamageCategory::MagicalDot => damage_after_crit_direct_hit,
        DamageCategory::AutoAttack => damage_after_crit_direct_hit * damage_random,
    };

    (final_damage, contribution_board, is_crit)
}

fn get_final_critical_hit_rate(
    buffs: &HashMap<StatusKey, BuffStatus>,
    debuffs: &HashMap<StatusKey, DebuffStatus>,
    is_guaranteed_crit: bool,
    player_power: &PlayerPower,
) -> MultiplierType {
    if is_guaranteed_crit {
        return 1.0;
    }
    let mut final_critical_hit_rate = player_power.critical_strike_rate;

    for buff in buffs.values() {
        final_critical_hit_rate += buff.get_critical_strike_rate_increase(is_guaranteed_crit);
    }

    for debuff in debuffs.values() {
        final_critical_hit_rate += debuff.get_critical_strike_rate_increase(is_guaranteed_crit);
    }

    final_critical_hit_rate
}

fn get_final_direct_hit_rate(
    buffs: &HashMap<StatusKey, BuffStatus>,
    debuffs: &HashMap<StatusKey, DebuffStatus>,
    is_guaranteed_direct_hit: bool,
    player_power: &PlayerPower,
) -> MultiplierType {
    if is_guaranteed_direct_hit {
        return 1.0;
    }
    let mut final_direct_hit_rate = player_power.critical_strike_rate;

    for buff in buffs.values() {
        final_direct_hit_rate += buff.get_critical_strike_rate_increase(is_guaranteed_direct_hit);
    }

    for debuff in debuffs.values() {
        final_direct_hit_rate += debuff.get_critical_strike_rate_increase(is_guaranteed_direct_hit);
    }

    final_direct_hit_rate
}

fn get_final_damage_increase(
    is_guaranteed_critical_hit: bool,
    is_guaranteed_direct_hit: bool,
    buffs: &HashMap<StatusKey, BuffStatus>,
    debuffs: &HashMap<StatusKey, DebuffStatus>,
    player_power: &PlayerPower,
) -> MultiplierType {
    let mut multiplier = 1.0;

    for buff in buffs.values() {
        multiplier *= buff.get_damage_increase(
            is_guaranteed_critical_hit,
            is_guaranteed_direct_hit,
            player_power.critical_strike_damage,
        );
    }

    for debuff in debuffs.values() {
        multiplier *= debuff.get_damage_increase(
            is_guaranteed_critical_hit,
            is_guaranteed_direct_hit,
            player_power.critical_strike_damage,
        );
    }

    multiplier
}

pub struct FfxivRawDamageCalculator {}

impl MultiplierCalculator for FfxivRawDamageCalculator {}
impl RawDamageCalculator for FfxivRawDamageCalculator {}

impl Default for FfxivRawDamageCalculator {
    fn default() -> Self {
        FfxivRawDamageCalculator {}
    }
}

#[cfg(test)]
mod tests {
    use crate::damage_calculator::raw_damage_calculator::{
        FfxivRawDamageCalculator, RawDamageCalculator,
    };
    use crate::live_objects::player::player_power::PlayerPower;
    use crate::types::DamageType;

    fn within_accepted_range(expected: f64, actual: f64) -> bool {
        let lower_bound = 0.90;
        let upper_bound = 1.10;
        expected >= actual * lower_bound && expected <= actual * upper_bound
    }

    #[test]
    fn test_ninja_damage_stat1() {
        // https://github.com/flyxiv/ffxiv_simbot/issues/11#issuecomment-2118842780
        // Ninja stat1 damage test

        let power = PlayerPower {
            critical_strike_rate: 1.169,
            critical_strike_damage: 1.519,
            direct_hit_rate: 1.242,
            determination_damage_multiplier: 1.063,
            tenacity_damage_multiplier: 1.0f64,
            main_stat_multiplier: 12.52,
            weapon_damage_multiplier: 1.62,
            speed_multiplier: 1.032,
        };

        let hyosho_potency = (1300 as f64 * 1.3) as DamageType;
        let raiton_potency = 650;
        let suiton_potency = 500;
        let fuma_potency = 450;
        let raiju_potency = 560;

        let direct_hit_expected = 1.06;
        let crit_expected = 1.08;

        let potencies = vec![
            (hyosho_potency, 37750),
            (raiton_potency, 13945),
            (suiton_potency, 10600),
            (fuma_potency, 9923),
            (raiju_potency, 11483),
        ];

        let damage_calculator = FfxivRawDamageCalculator {};

        for (potency, actual_damage) in potencies {
            let simulated_damage =
                damage_calculator.calculate_total_damage(potency, false, false, &power);

            // With crit/dh expected damage
            let expected_actual_damage =
                (actual_damage as f64 * crit_expected * direct_hit_expected) as DamageType;

            assert!(within_accepted_range(
                expected_actual_damage as f64,
                simulated_damage as f64
            ));
        }
    }
}

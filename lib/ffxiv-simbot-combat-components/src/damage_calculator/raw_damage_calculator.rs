use crate::damage_calculator::multiplier_calculator::{
    MultiplierCalculator, DIRECT_HIT_DAMAGE_MULTIPLIER,
};
use crate::event_ticker::PercentType;
use crate::id_entity::IdEntity;
use crate::live_objects::player::StatusKey;
use crate::skill::damage_category::DamageCategory;
use crate::status::buff_status::BuffStatus;
use crate::status::debuff_status::DebuffStatus;
use crate::status::Status;
use crate::{BuffIncreasePercentType, DamageType, IdType, PotencyType, StatusTable};
use ffxiv_simbot_db::stat_calculator::CharacterPower;
use ffxiv_simbot_db::MultiplierType;
use rand::{thread_rng, Rng};
use std::collections::HashMap;

pub const ONE_HUNDRED_PERCENT: f64 = 100.0;

/// Translates a player's skill potency to expected damage number.
/// Depending on the player's power, the skill's potency, and whether there is a
/// Guaranteed Critical Hit or Direct Hit buff + trait multiplier.
pub trait RawDamageCalculator: MultiplierCalculator {
    fn calculate_total_damage(
        &self,
        potency: DamageType,
        trait_percent: PercentType,
        is_guaranteed_critical_hit: bool,
        is_guaranteed_direct_hit: bool,
        buffs: &HashMap<StatusKey, BuffStatus>,
        debuffs: &HashMap<StatusKey, DebuffStatus>,
        player_power: &CharacterPower,
        damage_category: DamageCategory,
    ) -> (DamageType, HashMap<StatusKey, MultiplierType>, bool) {
        let potency = potency as MultiplierType;
        let base_damage =
            calculate_base_damage(potency, trait_percent, damage_category, player_power);

        let (crit_direct_hit_calculation_result, mut contribution_table, is_crit) =
            calculate_crit_direct_hit_damage(
                base_damage,
                buffs,
                debuffs,
                player_power,
                damage_category,
                is_guaranteed_critical_hit,
                is_guaranteed_direct_hit,
            );

        let damage_increase = get_final_damage_increase(
            is_guaranteed_critical_hit,
            is_guaranteed_direct_hit,
            buffs,
            debuffs,
            player_power,
        );

        let final_damage =
            MultiplierType::floor(crit_direct_hit_calculation_result * damage_increase);

        for buff in buffs.values() {
            let skill_contribution = buff.get_damage_increase();
            let mut entry = contribution_table
                .entry(StatusKey::new(buff.get_id(), buff.owner_id))
                .or_insert(0.0);

            *entry += crit_direct_hit_calculation_result * skill_contribution;
        }

        for debuff in debuffs.values() {
            let skill_contribution = debuff.get_damage_increase();
            let mut entry = contribution_table
                .entry(StatusKey::new(debuff.get_id(), debuff.owner_id))
                .or_insert(0.0);

            *entry += crit_direct_hit_calculation_result * skill_contribution;
        }

        for (value) in contribution_table.values_mut() {
            *value = MultiplierType::floor(*value);
        }

        (final_damage as DamageType, contribution_table, is_crit)
    }
}

/// https://www.akhmorning.com/allagan-studies/how-to-be-a-math-wizard/shadowbringers/damage-and-healing/#damage-over-time
/// Variables use names from the calculations of the above link.
fn calculate_base_damage(
    potency: MultiplierType,
    trait_percent: PercentType,
    damage_category: DamageCategory,
    player_power: &CharacterPower,
) -> f64 {
    match damage_category {
        DamageCategory::Direct => {
            let mut d1 = MultiplierType::floor(
                potency
                    * player_power.main_stat_multiplier
                    * player_power.determination_damage_multiplier,
            );

            let mut d2 = MultiplierType::floor(d1 * player_power.tenacity_damage_multiplier);
            d2 = MultiplierType::floor(d2 * player_power.weapon_damage_multiplier);
            d2 = MultiplierType::floor(d2 * trait_percent as MultiplierType);
            MultiplierType::floor(d2 / 100.0)
        }
        DamageCategory::PhysicalDot => {
            let mut d1 = MultiplierType::floor(
                potency
                    * player_power.main_stat_multiplier
                    * player_power.determination_damage_multiplier,
            );
            let mut d2 = MultiplierType::floor(d1 * player_power.tenacity_damage_multiplier);
            d2 = MultiplierType::floor(d2 * player_power.speed_multiplier * 1000.0);
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
            let mut d1 = MultiplierType::floor(
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
    base_damage: MultiplierType,
    buffs: &HashMap<StatusKey, BuffStatus>,
    debuffs: &HashMap<StatusKey, DebuffStatus>,
    player_power: &CharacterPower,
    damage_category: DamageCategory,
    is_guaranteed_critical_hit: bool,
    is_guaranteed_direct_hit: bool,
) -> (MultiplierType, HashMap<StatusKey, MultiplierType>, bool) {
    let damage_random = thread_rng().gen_range(95..=105) as MultiplierType / 100.0;
    let damage_before_crit_direct_hit = match damage_category {
        DamageCategory::Direct => base_damage,
        DamageCategory::PhysicalDot => base_damage * damage_random,
        DamageCategory::MagicalDot => base_damage * damage_random,
        DamageCategory::AutoAttack => base_damage,
    };

    let mut damage_after_crit_direct_hit = damage_before_crit_direct_hit;

    let critical_hit_rate =
        get_final_critical_hit_rate(buffs, debuffs, is_guaranteed_critical_hit, player_power);
    let direct_hit_rate =
        get_final_direct_hit_rate(buffs, debuffs, is_guaranteed_direct_hit, player_power);

    let crit_rng = thread_rng().gen_range(0..1000) as MultiplierType / 1000.0;
    let direct_hit_rng = thread_rng().gen_range(0..1000) as MultiplierType / 1000.0;
    let is_crit = crit_rng < critical_hit_rate;
    let is_direct_hit = direct_hit_rng < direct_hit_rate;
    let mut contribution_board: HashMap<StatusKey, MultiplierType> = HashMap::new();

    if is_crit {
        let crit_portion = damage_before_crit_direct_hit * player_power.critical_strike_damage
            - damage_before_crit_direct_hit;

        for buff in buffs.values() {
            let critical_hit_multiplier =
                buff.get_direct_hit_rate_increase(is_guaranteed_direct_hit);

            if critical_hit_multiplier > 0.0 {
                let skill_proportion = critical_hit_multiplier / direct_hit_rate;
                let skill_contribution = crit_portion * skill_proportion;
                let mut entry = contribution_board
                    .entry(StatusKey::new(buff.get_id(), buff.owner_id))
                    .or_insert(0.0);

                *entry += skill_contribution;
            }
        }

        for debuff in debuffs.values() {
            let critical_hit_multiplier =
                debuff.get_direct_hit_rate_increase(is_guaranteed_direct_hit);

            if critical_hit_multiplier > 0.0 {
                let skill_proportion = critical_hit_multiplier / direct_hit_rate;
                let skill_contribution = crit_portion * skill_proportion;
                let mut entry = contribution_board
                    .entry(StatusKey::new(debuff.get_id(), debuff.owner_id))
                    .or_insert(0.0);

                *entry += skill_contribution;
            }
        }

        damage_after_crit_direct_hit = MultiplierType::floor(
            damage_after_crit_direct_hit * player_power.critical_strike_damage,
        );
    }

    if is_direct_hit {
        let direct_hit_portion = damage_before_crit_direct_hit * DIRECT_HIT_DAMAGE_MULTIPLIER
            - damage_before_crit_direct_hit;

        for buff in buffs.values() {
            let direct_hit_multiplier = buff.get_direct_hit_rate_increase(is_guaranteed_direct_hit);

            if direct_hit_multiplier > 0.0 {
                let skill_proportion = direct_hit_multiplier / direct_hit_rate;
                let skill_contribution = direct_hit_portion * skill_proportion;
                let mut entry = contribution_board
                    .entry(StatusKey::new(buff.get_id(), buff.owner_id))
                    .or_insert(0.0);

                *entry += skill_contribution;
            }
        }

        for debuff in debuffs.values() {
            let direct_hit_multiplier =
                debuff.get_direct_hit_rate_increase(is_guaranteed_direct_hit);

            if direct_hit_multiplier > 0.0 {
                let skill_proportion = direct_hit_multiplier / direct_hit_rate;
                let skill_contribution = direct_hit_portion * skill_proportion;
                let mut entry = contribution_board
                    .entry(StatusKey::new(debuff.get_id(), debuff.owner_id))
                    .or_insert(0.0);

                *entry += skill_contribution;
            }
        }

        damage_after_crit_direct_hit =
            MultiplierType::floor(damage_after_crit_direct_hit * DIRECT_HIT_DAMAGE_MULTIPLIER);
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
    player_power: &CharacterPower,
) -> f64 {
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
    player_power: &CharacterPower,
) -> MultiplierType {
    if is_guaranteed_direct_hit {
        return 1.0;
    }

    let mut final_direct_hit_rate = player_power.direct_hit_rate;

    for buff in buffs.values() {
        final_direct_hit_rate += buff.get_direct_hit_rate_increase();
    }

    for debuff in debuffs.values() {
        final_direct_hit_rate += debuff.get_direct_hit_rate_increase();
    }

    final_direct_hit_rate
}

fn get_final_damage_increase(
    is_guaranteed_critical_hit: bool,
    is_guaranteed_direct_hit: bool,
    buffs: &HashMap<StatusKey, BuffStatus>,
    debuffs: &HashMap<StatusKey, DebuffStatus>,
    player_power: &CharacterPower,
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
    use crate::DamageType;
    use ffxiv_simbot_db::stat_calculator::CharacterPower;

    fn within_accepted_range(expected: f64, actual: f64) -> bool {
        let lower_bound = 0.90;
        let upper_bound = 1.10;
        expected >= actual * lower_bound && expected <= actual * upper_bound
    }

    #[test]
    fn test_ninja_damage_stat1() {
        // https://github.com/flyxiv/ffxiv_simbot/issues/11#issuecomment-2118842780
        // Ninja stat1 damage test

        let power = CharacterPower {
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

    #[test]
    fn test_ninja_damage_stat2() {
        // https://github.com/flyxiv/ffxiv_simbot/issues/11#issuecomment-2118844294
        // Ninja stat2 damage test

        let power = CharacterPower {
            critical_strike_rate: 1.142,
            critical_strike_damage: 1.492,
            direct_hit_rate: 1.137,
            determination_damage_multiplier: 1.044,
            tenacity_damage_multiplier: 1.0f64,
            main_stat_multiplier: 9.55,
            weapon_damage_multiplier: 1.62,
            speed_multiplier: 1.032,
        };

        let hyosho_potency = (1300 as f64 * 1.3) as DamageType;
        let raiton_potency = 650;
        let suiton_potency = 500;
        let fuma_potency = 450;
        let raiju_potency = 560;
        let spinning_edge_potency = 220;

        let direct_hit_expected = 1.06;
        let crit_expected = 1.08;

        let potencies = vec![
            (hyosho_potency, 28236),
            (raiton_potency, 10900),
            (suiton_potency, 8434),
            (fuma_potency, 7500),
            (raiju_potency, 9200),
            (spinning_edge_potency, 3393),
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

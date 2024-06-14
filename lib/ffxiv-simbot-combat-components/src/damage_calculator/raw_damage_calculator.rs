use crate::damage_calculator::multiplier_calculator::MultiplierCalculator;
use crate::event_ticker::PercentType;
use crate::skill::damage_category::DamageCategory;
use crate::{BuffIncreasePercentType, DamageType};
use ffxiv_simbot_db::stat_calculator::CharacterPower;
use ffxiv_simbot_db::MultiplierType;
use rand::{thread_rng, Rng};

/// Translates a player's skill potency to expected damage number.
/// Depending on the player's power, the skill's potency, and whether there is a
/// Guaranteed Critical Hit or Direct Hit buff + trait multiplier.
pub trait RawDamageCalculator: MultiplierCalculator {
    // TODO: Implement Crit/DH random number
    // TODO: Damage itself has a natural 5% variance
    fn calculate_raw_damage(
        &self,
        potency: DamageType,
        trait_percent: PercentType,
        damage_category: DamageCategory,
        is_guaranteed_critical_hit: bool,
        is_guaranteed_direct_hit: bool,
        player_power: &CharacterPower,
    ) -> (DamageType, bool) {
        let crit_rng = thread_rng().gen_range(0..100) as MultiplierType / 100.0;
        let critical_hit_rate = if is_guaranteed_critical_hit {
            1.0f64
        } else {
            player_power.critical_strike_rate - 1.0f64
        };
        let is_crit = crit_rng < critical_hit_rate;

        let direct_hit_rate = if is_guaranteed_direct_hit {
            1.0f64
        } else {
            player_power.direct_hit_rate - 1.0f64
        };

        let mut raw_damage = potency as MultiplierType;

        raw_damage *= self.calculate_crit_hit_rate_multiplier(
            player_power,
            to_increase_rate(critical_hit_rate),
            None,
        );
        raw_damage *=
            self.calculate_direct_hit_rate_multiplier(to_increase_rate(direct_hit_rate), None);
        raw_damage *= player_power.determination_damage_multiplier;
        raw_damage *= player_power.tenacity_damage_multiplier;
        raw_damage *= player_power.main_stat_multiplier;
        raw_damage *= trait_percent as MultiplierType / 100.0;

        match damage_category {
            DamageCategory::AutoAttack => {
                raw_damage *= player_power.auto_attack_delays / 3.0;
                raw_damage *= player_power.speed_multiplier;
            }
            DamageCategory::MagicalDot | DamageCategory::PhysicalDot => {
                raw_damage *= player_power.weapon_damage_multiplier;
                raw_damage *= player_power.speed_multiplier;
            }
            DamageCategory::Direct => {
                raw_damage *= player_power.weapon_damage_multiplier;
            }
        }

        (raw_damage as DamageType, is_crit)
    }
}

#[inline]
fn to_increase_rate(multiplier: MultiplierType) -> BuffIncreasePercentType {
    (multiplier * 100f64) as BuffIncreasePercentType
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
            auto_attack_delays: 2.56,
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
                damage_calculator.calculate_raw_damage(potency, false, false, &power);

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
            auto_attack_delays: 2.56,
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
                damage_calculator.calculate_raw_damage(potency, false, false, &power);

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

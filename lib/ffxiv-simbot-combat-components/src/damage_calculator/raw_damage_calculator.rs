use crate::damage_calculator::multiplier_calculator::MultiplierCalculator;
use crate::skill::Skill;
use crate::{BuffIncreasePercentType, DamageType};
use ffxiv_simbot_db::stat_calculator::CharacterPower;
use ffxiv_simbot_db::MultiplierType;

/// Translates a player's skill potency to expected damage number.
/// Depending on the player's power, the skill's potency, and whether there is a
/// Guaranteed Critical Hit or Direct Hit buff + trait multiplier.
pub trait RawDamageCalculator: MultiplierCalculator {
    // TODO: Implement Crit/DH random number
    // TODO: Damage itself has a natural 5% variance
    fn calculate_raw_damage(
        &self,
        potency: DamageType,
        is_guaranteed_critical_hit: bool,
        is_guaranteed_direct_hit: bool,
        player_power: &CharacterPower,
    ) -> DamageType {
        let critical_hit_rate = if is_guaranteed_critical_hit {
            1.0f64
        } else {
            player_power.critical_strike_rate - 1.0f64
        };

        let direct_hit_rate = if is_guaranteed_direct_hit {
            1.0f64
        } else {
            player_power.direct_hit_rate - 1.0f64
        };

        let mut raw_damage = potency as MultiplierType;

        raw_damage *= self
            .calculate_crit_hit_rate_multiplier(player_power, to_increase_rate(critical_hit_rate));
        raw_damage *= self.calculate_direct_hit_rate_multiplier(to_increase_rate(direct_hit_rate));
        raw_damage *= player_power.determination_damage_multiplier;
        raw_damage *= player_power.tenacity_damage_multiplier;
        raw_damage *= player_power.main_stat_multiplier;
        raw_damage *= player_power.weapon_damage_multiplier;

        raw_damage as DamageType
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

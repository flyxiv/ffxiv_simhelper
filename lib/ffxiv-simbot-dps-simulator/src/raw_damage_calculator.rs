use crate::multiplier_calculator::MultiplierCalculator;
use ffxiv_simbot_combat_components::skill::{Skill, SkillInfo};
use ffxiv_simbot_combat_components::{BuffIncreaseType, DamageType};
use ffxiv_simbot_db::stat_calculator::CharacterPower;
use ffxiv_simbot_db::DamageMultiplierType;

/// Calculate the expected raw damage of the skill
/// Depending on the player's power, the skill's potency, and whether there is a
/// Guaranteed Critical Hit or Direct Hit buff.
pub(crate) trait RawDamageCalculator: MultiplierCalculator {
    fn calculate_raw_damage<S>(
        &self,
        skill_info: &SkillInfo<S>,
        player_power: &CharacterPower,
    ) -> DamageType
    where
        S: Skill,
    {
        let critical_hit_rate = if skill_info.guaranteed_critical_hit {
            1.0f64
        } else {
            player_power.critical_strike_rate
        };

        let direct_hit_rate = if skill_info.guaranteed_direct_hit {
            1.0f64
        } else {
            player_power.direct_hit_rate
        };

        let mut raw_damage = skill_info.skill.get_potency() as DamageMultiplierType;

        raw_damage *= self
            .calculate_crit_hit_rate_multiplier(player_power, to_increase_rate(critical_hit_rate));
        raw_damage *= self.calculate_direct_hit_rate_multiplier(to_increase_rate(direct_hit_rate));
        raw_damage *= self.calculate_damage_multiplier(to_increase_rate(
            player_power.determination_damage_multiplier,
        ));
        raw_damage *= self
            .calculate_damage_multiplier(to_increase_rate(player_power.tenacity_damage_multiplier));
        raw_damage *=
            self.calculate_damage_multiplier(to_increase_rate(player_power.main_stat_multiplier));
        raw_damage *= self
            .calculate_damage_multiplier(to_increase_rate(player_power.weapon_damage_multiplier));

        raw_damage as DamageType
    }
}

#[inline]
fn to_increase_rate(multiplier: DamageMultiplierType) -> BuffIncreaseType {
    (multiplier * 100f64) as BuffIncreaseType
}

pub(crate) struct FfxivRawDamageCalculator {}

impl MultiplierCalculator for FfxivRawDamageCalculator {}
impl RawDamageCalculator for FfxivRawDamageCalculator {}

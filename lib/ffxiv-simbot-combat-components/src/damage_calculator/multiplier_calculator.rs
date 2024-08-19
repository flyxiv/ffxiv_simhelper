use crate::live_objects::player::player_power::PlayerPower;
use crate::status::status_info::StatusInfo;
use crate::status::Status;
use crate::types::{BuffIncreasePercentType, MultiplierType, SkillStackType};

#[inline]
pub(crate) fn percent_to_actual_value(increase_percent: BuffIncreasePercentType) -> MultiplierType {
    debug_assert!(increase_percent >= 1, "{}", increase_percent);
    increase_percent as MultiplierType / 100f64
}

pub const DIRECT_HIT_DAMAGE_MULTIPLIER: f64 = 1.25f64;

pub trait MultiplierCalculator {
    fn calculate_damage_multiplier(
        &self,
        damage_increase: BuffIncreasePercentType,
        stacks: Option<SkillStackType>,
    ) -> MultiplierType {
        let increase_value = percent_to_actual_value(damage_increase);
        debug_assert!(increase_value >= 0.0f64, "{}", increase_value);

        return if let Some(stack) = stacks {
            1.0f64 + (increase_value * stack as MultiplierType)
        } else {
            1.0f64 + increase_value
        };
    }

    fn calculate_crit_hit_rate_multiplier(
        &self,
        character_power: &PlayerPower,
        crit_rate_increase: BuffIncreasePercentType,
        stacks: Option<SkillStackType>,
    ) -> MultiplierType {
        let critical_percent_damage = character_power.critical_strike_damage - 1.0f64;
        debug_assert!(
            critical_percent_damage >= 0.0f64,
            "{}",
            critical_percent_damage
        );

        let increase_value = percent_to_actual_value(crit_rate_increase);
        debug_assert!(increase_value >= 0.0f64, "{}", increase_value);

        let expected_damage_increase = if let Some(stack) = stacks {
            critical_percent_damage * increase_value * (stack as MultiplierType)
        } else {
            critical_percent_damage * increase_value
        };
        debug_assert!(
            expected_damage_increase >= 0.0f64,
            "{}",
            expected_damage_increase
        );

        1.0f64 + expected_damage_increase
    }
    fn calculate_direct_hit_rate_multiplier(
        &self,
        direct_hit_rate_increase: BuffIncreasePercentType,
        stacks: Option<SkillStackType>,
    ) -> MultiplierType {
        let increase_value = percent_to_actual_value(direct_hit_rate_increase);
        let expected_damage_increase = if let Some(stack) = stacks {
            DIRECT_HIT_DAMAGE_MULTIPLIER * increase_value * (stack as MultiplierType)
        } else {
            DIRECT_HIT_DAMAGE_MULTIPLIER * increase_value
        };
        debug_assert!(
            expected_damage_increase >= 0.0f64,
            "{}",
            expected_damage_increase
        );

        1.0f64 + expected_damage_increase
    }

    fn calculate_multiplier<S>(&self, status: &S, character_power: &PlayerPower) -> MultiplierType
    where
        S: Status,
    {
        let mut total_multiplier = 1.0f64;
        let stacks = status.get_stack();

        for status_info in status.get_status_info() {
            let damage_increase = match status_info {
                StatusInfo::DamagePercent(damage_increase) => {
                    self.calculate_damage_multiplier(*damage_increase, Some(stacks))
                }
                StatusInfo::CritHitRatePercent(crit_rate_increase) => self
                    .calculate_crit_hit_rate_multiplier(
                        character_power,
                        *crit_rate_increase,
                        Some(stacks),
                    ),
                StatusInfo::DirectHitRatePercent(direct_hit_rate_increase) => self
                    .calculate_direct_hit_rate_multiplier(*direct_hit_rate_increase, Some(stacks)),
                _ => 1.0f64,
            };

            debug_assert!(
                damage_increase >= 0.0f64,
                "damage_increase: {}",
                damage_increase
            );

            total_multiplier *= damage_increase;
        }

        total_multiplier
    }
}

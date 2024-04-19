use ffxiv_simbot_combat_components::status::{Status, StatusInfo};
use ffxiv_simbot_combat_components::BuffIncreaseType;
use ffxiv_simbot_db::stat_calculator::CharacterPower;
use ffxiv_simbot_db::DamageMultiplierType;
use lazy_static::lazy_static;

#[inline]
pub(crate) fn percent_to_actual_value(increase_percent: BuffIncreaseType) -> DamageMultiplierType {
    increase_percent as DamageMultiplierType / 100f64
}

lazy_static! {
    static ref DIRECT_HIT_DAMAGE_MULTIPLIER: f64 = 0.25f64;
}

pub trait MultiplierCalculator {
    fn calculate_damage_multiplier(
        &self,
        damage_increase: BuffIncreaseType,
    ) -> DamageMultiplierType {
        let increase_value = percent_to_actual_value(damage_increase);
        1.0f64 + increase_value
    }
    fn calculate_crit_hit_rate_multiplier(
        &self,
        character_power: &CharacterPower,
        crit_rate_increase: BuffIncreaseType,
    ) -> DamageMultiplierType {
        let critical_percent_damage = character_power.critical_strike_damage - 1.0f64;
        let increase_value = percent_to_actual_value(crit_rate_increase);
        let expected_damage_increase = critical_percent_damage * increase_value;

        1.0f64 + expected_damage_increase
    }
    fn calculate_direct_hit_rate_multiplier(
        &self,
        direct_hit_rate_increase: BuffIncreaseType,
    ) -> DamageMultiplierType {
        let increase_value = percent_to_actual_value(direct_hit_rate_increase);
        let expected_damage_increase = *DIRECT_HIT_DAMAGE_MULTIPLIER * increase_value;
        1.0f64 + expected_damage_increase
    }

    fn calculate_multiplier<S>(
        &self,
        status: &S,
        character_power: &CharacterPower,
    ) -> DamageMultiplierType
    where
        S: Status,
    {
        match status.get_status_info() {
            StatusInfo::DamagePercent(damage_increase) => {
                let damage_multiplier = self.calculate_damage_multiplier(damage_increase);
                status.set_damage_multiplier(damage_multiplier);
            }
            StatusInfo::CritHitRatePercent(crit_rate_increase) => {
                let crit_rate_multiplier =
                    self.calculate_crit_hit_rate_multiplier(character_power, crit_rate_increase);
                status.set_crit_rate_multiplier(crit_rate_multiplier);
            }
            StatusInfo::DirectHitRatePercent(direct_hit_rate_increase) => {
                let direct_hit_rate_multiplier =
                    self.calculate_direct_hit_rate_multiplier(direct_hit_rate_increase);
                status.set_direct_hit_rate_multiplier(direct_hit_rate_multiplier);
            }
            _ => {}
        }
    }
}

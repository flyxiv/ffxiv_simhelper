use std::collections::HashMap;
use crate::{BuffIncreaseType, IdType};
use ffxiv_simbot_lib_db::stat_calculator::CharacterPower;
use ffxiv_simbot_lib_db::DamageMultiplierType;
/// Calculates Expected Damage Multiplier for Critical/Direct Hit/Damage buff and debuff.
use lazy_static::lazy_static;

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

    /// Given the raw damage and all the list of buffs/debuffs on the player and the target,
    /// 1) Convert the buffs to a damage multiplier.
    /// 2) Calculate the RDPS contribution of each buff and update it to the RDPS table
    /// 3) Give the final damage with all multipliers - all contributions as the raw DPS of the skill.
    fn convert_buffs_to_damage_multiplier(&self, total_rdps_table: buff_list: HashMap<IdType, DamageMultiplierType>) -> DamageMultiplierType {

    }
}

pub struct FfxivMultiplierCalculator {}

impl MultiplierCalculator for FfxivMultiplierCalculator {}

#[inline]
fn percent_to_actual_value(increase_percent: BuffIncreaseType) -> DamageMultiplierType {
    increase_percent as DamageMultiplierType / 100f64
}

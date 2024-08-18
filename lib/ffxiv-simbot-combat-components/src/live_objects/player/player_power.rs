use crate::live_objects::player::role::{job_abbrev_to_role, Role};
use crate::types::{BuffIncreasePercentType, IncreaseType, MultiplierType, StatType};
use serde::{Deserialize, Serialize};
use std::cmp::min;

const NON_TANK_MULTIPLIER: MultiplierType = 0.5;
const TANK_MULTIPLIER: MultiplierType = 0.4;
const MAIN_STAT_BASE_NON_TANK: StatType = 437;
const MAIN_STAT_BASE_TANK: StatType = 440;

/// The main/sub stats of players that are converted to
/// actual in-game power of characters
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlayerPower {
    pub auto_attack_delays: MultiplierType,
    pub critical_strike_rate: MultiplierType,
    pub critical_strike_damage: MultiplierType,
    pub direct_hit_rate: MultiplierType,
    pub determination_multiplier: MultiplierType,
    pub tenacity_multiplier: MultiplierType,
    pub speed_multiplier: MultiplierType,
    pub weapon_damage_multiplier: MultiplierType,
    pub main_stat_multiplier: MultiplierType,
    pub weapon_damage: StatType,
    pub main_stat: StatType,
    pub critical_strike: StatType,
    pub direct_hit: StatType,
    pub determination: StatType,
    pub skill_speed: StatType,
    pub spell_speed: StatType,
    pub tenacity: StatType,
}

pub fn add_main_stat(
    power: &PlayerPower,
    job_abbrev: &String,
    maximum_increase: IncreaseType,
    increase_percent: BuffIncreasePercentType,
) -> PlayerPower {
    debug_assert!(increase_percent >= 0, "{}", increase_percent);
    let mut power = power.clone();
    let role = job_abbrev_to_role(job_abbrev);

    let increase_percent_amount =
        (power.main_stat_multiplier * (increase_percent as MultiplierType / 100.0)) as IncreaseType;
    let increase_amount = min(maximum_increase, increase_percent_amount);
    power.main_stat += increase_amount as StatType;
    power.main_stat_multiplier = convert_main_stat_to_multiplier(power.main_stat, role);
    power
}

fn convert_main_stat_to_multiplier(main_stat: StatType, role: Role) -> MultiplierType {
    let main_stat_multiplier_increase_percent = if matches!(role, Role::Tank) {
        f64::floor((main_stat - MAIN_STAT_BASE_TANK) as MultiplierType * TANK_MULTIPLIER)
    } else {
        f64::floor((main_stat - MAIN_STAT_BASE_NON_TANK) as MultiplierType * NON_TANK_MULTIPLIER)
    };

    1.0 + main_stat_multiplier_increase_percent / 100.0
}

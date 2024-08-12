use crate::types::{MultiplierType, StatType};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlayerPower {
    pub auto_attack_delays: MultiplierType,
    pub critical_strike_rate: MultiplierType,
    pub critical_strike_damage: MultiplierType,
    pub direct_hit_rate: MultiplierType,
    pub determination_damage_multiplier: MultiplierType,
    pub tenacity_damage_multiplier: MultiplierType,
    pub speed_multiplier: MultiplierType,
    pub weapon_damage_multiplier: MultiplierType,
    pub main_stat_multiplier: MultiplierType,
    pub main_stat: StatType,
    pub job_abbrev: String,
}

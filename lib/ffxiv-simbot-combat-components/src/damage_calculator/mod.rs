use crate::live_objects::player::StatusKey;
use crate::{DamageType, IdType};
use ffxiv_simbot_db::MultiplierType;
use std::collections::HashMap;

pub mod multiplier_calculator;
pub mod raw_damage_calculator;
pub mod rdps_calculator;

/// Implements FFXIV's damage calculation logic based on the
/// player's stat multiplier + skill's potency + guaranteed critical hit/direct hit buff.
pub struct DamageRdpsProfile {
    pub skill_id: IdType,
    pub raw_damage: MultiplierType,
    pub final_damage: MultiplierType,
    pub rdps_contribution: HashMap<StatusKey, MultiplierType>,
}

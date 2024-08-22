use crate::live_objects::player::StatusKey;
use crate::types::{IdType, MultiplierType};
use std::collections::HashMap;
use std::fmt::{Debug, Pointer};

pub mod raw_damage_calculator;

pub const INCREASE_BASE: MultiplierType = 0.0;
pub const MULTIPLIER_BASE: MultiplierType = 1.0;
pub const ONE_HUNDRED_PERCENT: MultiplierType = 100.0;
pub const DIRECT_HIT_DAMAGE_MULTIPLIER: MultiplierType = 1.25f64;
pub const DAMAGE_BASE: MultiplierType = 0.0;

/// Implements FFXIV's damage calculation logic based on the
/// player's stat multiplier + skill's potency + guaranteed critical hit/direct hit buff.
pub struct DamageRdpsProfile {
    pub raw_damage: MultiplierType,
    pub final_damage: MultiplierType,
    pub rdps_contribution: HashMap<StatusKey, MultiplierType>,
}

impl Debug for DamageRdpsProfile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "raw damage: {}", self.raw_damage);

        for (key, value) in &self.rdps_contribution {
            write!(f, "key: {:?}, value: {:?}", key, value);
        }

        write!(f, "final damage: {}", self.final_damage);

        Ok(())
    }
}

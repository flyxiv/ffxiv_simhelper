use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};

type Result<T> = std::result::Result<T, SimulatorError>;
pub(crate) type TimeType = i32;
pub(crate) type StatusIdType = usize;
pub(crate) type DpsType = f64;
pub(crate) type DamageType = usize;
pub(crate) type IdType = usize;
pub(crate) type PotencyType = usize;
pub(crate) type ManaType = i32;
pub(crate) type BuffIncreaseType = usize;

mod damage_rdps_profile;
mod multiplier_calculator;
mod priority_table;
mod simulator;

pub enum SimulatorError {
    DebuffNotFoundError(String),
    BuffNotFoundError(String),
}

impl Debug for SimulatorError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SimulatorError::DebuffNotFoundError(s) => write!(f, "Debuff not found: {}", s),
            SimulatorError::BuffNotFoundError(s) => write!(f, "Buff not found: {}", s),
        }
    }
}

impl Display for SimulatorError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SimulatorError::DebuffNotFoundError(s) => write!(f, "Debuff not found: {}", s),
            SimulatorError::BuffNotFoundError(s) => write!(f, "Buff not found: {}", s),
        }
    }
}

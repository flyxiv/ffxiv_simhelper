use std::fmt::{Debug, Display, Formatter};

type Result<T> = std::result::Result<T, SimulatorError>;

mod damage_rdps_profile;
mod multiplier_calculator;
mod raw_damage_calculator;
pub mod simulator;
mod skill_calculator;
mod skill_simulator;
mod turn_calculator;
mod util;

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

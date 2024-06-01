use std::fmt::{Debug, Display, Formatter};

type Result<T> = std::result::Result<T, SimulatorError>;

pub mod combat_simulator;
pub mod simulation_result;

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

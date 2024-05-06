use std::fmt::{Debug, Display, Formatter};

type Result<T> = std::result::Result<T, SimulatorError>;

pub(crate) mod damage_calculator;
mod damage_rdps_profile;
pub(crate) mod event_ticker;
mod ffxiv_event;
pub(crate) mod party;
pub mod simulation_result;
pub mod simulator;
pub(crate) mod skill_simulator;
mod turn_calculator;

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

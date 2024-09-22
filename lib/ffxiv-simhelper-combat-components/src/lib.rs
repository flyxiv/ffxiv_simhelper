use std::fmt::{Debug, Display, Formatter};

pub(crate) mod combat_resources;
pub mod consts;
pub mod damage_calculator;
pub mod event;
pub mod event_ticker;
pub mod id_entity;
pub mod jobs_skill_data;
pub mod live_objects;
pub mod owner_tracker;
pub mod rotation;
pub mod skill;
pub mod status;
pub mod types;

pub enum CombatComponentsError {
    DebuffNotFoundError(String),
    BuffNotFoundError(String),
}

impl Debug for CombatComponentsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CombatComponentsError::DebuffNotFoundError(s) => write!(f, "Debuff not found: {}", s),
            CombatComponentsError::BuffNotFoundError(s) => write!(f, "Buff not found: {}", s),
        }
    }
}

impl Display for CombatComponentsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CombatComponentsError::DebuffNotFoundError(s) => write!(f, "Debuff not found: {}", s),
            CombatComponentsError::BuffNotFoundError(s) => write!(f, "Buff not found: {}", s),
        }
    }
}

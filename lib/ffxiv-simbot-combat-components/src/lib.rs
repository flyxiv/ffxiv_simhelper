use crate::live_objects::player::StatusKey;
use crate::types::{IdType, TimeType};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::rc::Rc;

pub(crate) mod combat_resources;
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

pub static COMBAT_START_TIME: TimeType = -10000;
pub static TARGET_ID: IdType = 100;
pub static SIMULATION_START_TIME_MILLISECOND: TimeType = -5000;

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

use crate::live_objects::player::StatusKey;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::rc::Rc;

pub(crate) mod combat_resources;
pub mod damage_calculator;
pub mod event;
pub mod event_ticker;
pub mod id_entity;
pub(crate) mod jobs_skill_data;
pub mod live_objects;
pub mod owner_tracker;
pub mod rotation;
pub mod skill;
pub mod status;

type Result<T> = std::result::Result<T, CombatComponentsError>;

/// Shows the damage profile: Damage contribution of each buff/skill.
pub type DamageProfileTable = HashMap<IdType, DamageType>;
pub(crate) type ResourceType = i32;
pub(crate) type StackType = usize;

pub type TimeType = i32;
pub(crate) type StatusIdType = usize;
pub type DpsType = f64;
pub type DamageType = usize;
pub type IdType = usize;
pub(crate) type PotencyType = usize;
pub(crate) type ManaType = i32;
pub type BuffIncreasePercentType = usize;
pub(crate) type TurnCount = usize;

pub(crate) type BuffTable<S> = HashMap<IdType, S>;
pub type StatusTable<S> = Rc<RefCell<HashMap<StatusKey, S>>>;
pub(crate) type ComboType = Option<IdType>;
pub(crate) type PercentType = usize;

pub static COMBAT_START_TIME: TimeType = -10000;
pub static TARGET_ID: IdType = 100;

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

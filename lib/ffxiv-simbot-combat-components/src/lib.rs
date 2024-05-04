use crate::live_objects::player::Player;
use crate::status::Status;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};

pub mod id_entity;
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

pub(crate) type BuffTable<S: Status + Sized> = HashMap<IdType, S>;
pub(crate) type Party<P> = Vec<P>;

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

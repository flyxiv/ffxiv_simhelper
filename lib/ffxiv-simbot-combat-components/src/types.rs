use crate::live_objects::player::StatusKey;
use crate::CombatComponentsError;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub type StatType = i32;
pub type MultiplierType = f64;
pub type IncreaseType = usize;
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

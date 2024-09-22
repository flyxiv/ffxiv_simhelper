use crate::live_objects::player::StatusKey;
use crate::status::snapshot_status::SnapshotInfo;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub type StatType = u16;
pub type MultiplierType = f64;
pub type IncreaseType = u16;
/// Shows the damage profile: Damage contribution of each buff/skill.
pub type DamageProfileTable = HashMap<SkillIdType, PotencyType>;
pub(crate) type ResourceType = i16;
pub(crate) type ResourceIdType = u8;
pub(crate) type StackType = i8;

pub type TimeType = i32;
pub(crate) type StatusIdType = u16;
pub type DpsType = f64;
pub type PotencyType = u16;
pub type SkillIdType = u16;
pub type PlayerIdType = u8;
pub type BuffIncreasePercentType = u8;
pub type SkillStackType = i8;

pub type StatusTable<S> = Rc<RefCell<HashMap<StatusKey, S>>>;
pub(crate) type ComboType = Option<u8>;

pub type SnapshotTable = HashMap<StatusKey, SnapshotInfo>;

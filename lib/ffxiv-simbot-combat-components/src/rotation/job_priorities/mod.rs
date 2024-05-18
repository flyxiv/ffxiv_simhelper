use crate::IdType;
use std::collections::HashMap;

//pub(crate) mod dragoon;
pub(crate) mod ffxiv_priority_table;
pub(crate) mod ninja;
pub(crate) mod priority_table;
pub(crate) mod sage;

pub(crate) type SkillTable<S> = HashMap<IdType, S>;

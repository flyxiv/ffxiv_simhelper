use crate::id_entity::IdEntity;
use crate::rotation::priority_table::SkillPrerequisite;
use crate::types::IdType;
use std::collections::HashMap;

pub mod cooldown_timer;
pub(crate) mod ffxiv_priority_table;
pub(crate) mod priority_table;
mod simulate_status;
mod simulated_combat_resource;

pub(crate) type SkillTable<S> = HashMap<IdType, S>;
#[derive(Clone)]
pub(crate) struct SkillPriorityInfo {
    pub(crate) skill_id: IdType,
    pub(crate) prerequisite: Option<SkillPrerequisite>,
}

impl IdEntity for SkillPriorityInfo {
    fn get_id(&self) -> IdType {
        self.skill_id
    }
}

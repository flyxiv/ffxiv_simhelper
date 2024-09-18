use crate::id_entity::IdEntity;
use crate::rotation::priority_table::SkillPrerequisite;
use crate::types::SkillIdType;
use std::collections::HashMap;

pub mod cooldown_timer;
pub(crate) mod ffxiv_priority_table;
mod information_needed_for_rotation_decision;
pub(crate) mod priority_table;
pub(crate) mod skill_simulation_event;

pub(crate) type SkillTable<S> = HashMap<SkillIdType, S>;
#[derive(Clone)]
pub(crate) struct SkillPriorityInfo {
    pub(crate) skill_id: SkillIdType,
    pub(crate) prerequisite: Option<SkillPrerequisite>,
}

impl IdEntity for SkillPriorityInfo {
    fn get_id(&self) -> SkillIdType {
        self.skill_id
    }
}

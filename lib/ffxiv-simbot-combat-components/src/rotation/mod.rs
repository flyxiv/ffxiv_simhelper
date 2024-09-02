use crate::id_entity::IdEntity;
use crate::rotation::priority_table::SkillPrerequisite;
use crate::types::SkillIdType;
use std::collections::HashMap;

pub mod cooldown_timer;
pub(crate) mod ffxiv_priority_table;
pub(crate) mod priority_simulation_data;
pub(crate) mod priority_table;
mod simulate_status;
mod simulated_combat_resource;

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

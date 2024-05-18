use crate::id_entity::IdEntity;
use crate::rotation::job_priorities::priority_table::SkillPrerequisite;
use crate::IdType;

pub mod cooldown_timer;
pub(crate) mod job_priorities;
mod simulate_status;
mod simulated_combat_resource;

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

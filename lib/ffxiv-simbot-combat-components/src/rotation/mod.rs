use crate::id_entity::IdEntity;
use crate::rotation::job_priorities::ninja::NinjaPriorityTable;
use crate::rotation::job_priorities::sage::SagePriorityTable;
use crate::rotation::priority_table::SkillPrerequisite;
use crate::IdType;

pub(crate) mod cooldown_timer;
pub(crate) mod job_priorities;
pub mod priority_table;
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

use crate::id_entity::IdEntity;
use crate::rotation::job_priorities::ninja::NinjaPriorityTable;
use crate::rotation::job_priorities::sage::SagePriorityTable;
use crate::rotation::priority_table::SkillPrerequisite;
use crate::IdType;

pub mod cooldown_timer;
mod ffxiv_priority_table;
pub(crate) mod job_priorities;
pub mod priority_table;

#[derive(Clone)]
pub(crate) enum FfxivPriorityTable {
    Sage(SagePriorityTable),
    Ninja(NinjaPriorityTable),
}

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

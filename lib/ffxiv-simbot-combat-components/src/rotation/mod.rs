use crate::rotation::job_priorities::ninja::NinjaPriorityTable;
use crate::rotation::job_priorities::sage::SagePriorityTable;
use crate::rotation::priority_table::SkillPrerequisite;

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
pub(crate) struct SkillPriorityInfo<S> {
    pub(crate) skill: S,
    pub(crate) prerequisite: Option<SkillPrerequisite>,
}

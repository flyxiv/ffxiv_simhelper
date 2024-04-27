use crate::rotation::cooldown_timer::CooldownTimer;
use crate::rotation::job_priorities::sage::SagePriorityTable;
use crate::rotation::priority_table::SkillPrerequisite;
use crate::skill::attack_skill::AttackSkill;

pub mod cooldown_timer;
mod ffxiv_priority_table;
pub(crate) mod job_priorities;
pub mod priority_table;

#[derive(Clone)]
pub enum FfxivPriorityTable {
    Sage(SagePriorityTable),
}

pub(crate) struct SkillPriorityInfo {
    pub(crate) skill: AttackSkill,
    pub(crate) prerequisite: Option<SkillPrerequisite>,
}

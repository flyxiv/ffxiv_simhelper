use crate::skill::Skill;

/// Stores the priority list of the job's offensive skills
pub trait PriorityTable<T> {
    fn get_priority_list(&self) -> Vec<T>;
}

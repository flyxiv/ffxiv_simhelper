/// Stores the priority list of the job's offensive skills
pub trait PriorityTable {
    fn get_priority_list(&self) -> Vec<Skill>;
}

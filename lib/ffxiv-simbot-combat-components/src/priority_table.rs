use crate::player::Player;
use crate::skill::Skill;

/// Stores the priority list of the job's offensive skills
/// And gets the next skill to use based on the priority list
pub trait PriorityTable<S: Skill> {
    fn get_next_skill<P: Player>(&self, ffxiv_player: &P) -> S;
}

pub struct FfxivPriorityTable<S: Skill> {
    gcd_priority_list: Vec<S>,
    ogcd_priority_list: Vec<S>,
}

use crate::player::Player;
use crate::skill::{AttackSkill, Skill, SkillInfo};
use crate::status::{BuffStatus, DebuffStatus};
use std::cell::RefCell;
use std::rc::Rc;

/// Stores the priority list of the job's offensive skills
/// And gets the next skill to use based on the priority list
pub trait PriorityTable<S: Skill> {
    fn get_next_skill<P: Player>(
        &self,
        buff_list: Rc<RefCell<Vec<BuffStatus>>>,
        debuff_list: Rc<RefCell<Vec<DebuffStatus>>>,
        player: &P,
    ) -> Option<SkillInfo<S>>;
}

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
    ) -> SkillInfo<S>;
}

pub struct FfxivPriorityTable<S: Skill> {
    gcd_priority_list: Vec<S>,
    ogcd_priority_list: Vec<S>,
}

impl<S> PriorityTable<S> for FfxivPriorityTable<S>
where
    S: Skill,
{
    fn get_next_skill<P: Player>(
        &self,
        _buff_list: Rc<RefCell<Vec<BuffStatus>>>,
        _debuff_list: Rc<RefCell<Vec<DebuffStatus>>>,
        _player: &P,
    ) -> SkillInfo<S> {
        /// TODO
        let first_skill = self.gcd_priority_list.get(0).unwrap();

        SkillInfo {
            skill: first_skill.clone(),
            guaranteed_critical_hit: false,
            guaranteed_direct_hit: false,
        }
    }
}

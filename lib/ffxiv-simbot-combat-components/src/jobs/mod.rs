use crate::player::Player;
use crate::priority_table::PriorityTable;
use crate::skill::{AttackSkill, SkillInfo};
use crate::status::{BuffStatus, DebuffStatus};
use std::cell::RefCell;
use std::rc::Rc;

mod ninja;
pub mod sage;

#[derive(Clone)]
pub enum FfxivPriorityTable {
    Sage(sage::SagePriorityTable),
}

impl PriorityTable<AttackSkill> for FfxivPriorityTable {
    fn get_next_skill<P>(
        &self,
        buff_list: Rc<RefCell<Vec<BuffStatus>>>,
        debuff_list: Rc<RefCell<Vec<DebuffStatus>>>,
        player: &P,
    ) -> Option<SkillInfo<AttackSkill>>
    where
        P: Player,
    {
        match self {
            FfxivPriorityTable::Sage(sage_priority_table) => {
                sage_priority_table.get_next_skill(buff_list, debuff_list, player)
            }
        }
    }
}

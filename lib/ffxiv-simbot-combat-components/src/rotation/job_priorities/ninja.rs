use crate::rotation::job_priorities::priority_table::{Opener, PriorityTable};
use crate::rotation::SkillPriorityInfo;
use crate::skill::job_abilities::ninja_abilities::{
    make_ninja_gcd_table, make_ninja_ogcd_table, make_ninja_opener,
};
use crate::{IdType, TurnCount};
use std::cell::RefCell;

#[derive(Clone)]
pub(crate) struct NinjaPriorityTable {
    turn_count: RefCell<TurnCount>,
    opener: Vec<Opener>,

    gcd_priority_table: Vec<SkillPriorityInfo>,
    ogcd_priority_table: Vec<SkillPriorityInfo>,
}

impl PriorityTable for NinjaPriorityTable {
    fn get_opener_len(&self) -> usize {
        self.opener.len()
    }

    fn get_opener_at(&self, index: usize) -> Opener {
        self.opener[index].clone()
    }

    fn get_gcd_priority_table(&self) -> &Vec<SkillPriorityInfo> {
        &self.gcd_priority_table
    }

    fn get_ogcd_priority_table(&self) -> &Vec<SkillPriorityInfo> {
        &self.ogcd_priority_table
    }

    fn increment_turn(&self) {
        *self.turn_count.borrow_mut() += 1;
    }

    fn get_turn_count(&self) -> IdType {
        *self.turn_count.borrow()
    }
}

impl NinjaPriorityTable {
    pub fn new() -> Self {
        Self {
            turn_count: RefCell::new(0),
            opener: make_ninja_opener(),
            gcd_priority_table: make_ninja_gcd_table(),
            ogcd_priority_table: make_ninja_ogcd_table(),
        }
    }
}

use crate::live_objects::player::ffxiv_player::FfxivPlayer;
use crate::rotation::priority_table::{Opener, PriorityTable, SkillUsageInfo};
use crate::rotation::SkillPriorityInfo;
use crate::skill::job_abilities::sage_abilities::{make_sage_gcd_priority_table, make_sage_opener};
use crate::{IdType, TurnCount};
use std::cell::RefCell;

#[derive(Clone)]
pub struct SagePriorityTable {
    turn_count: RefCell<TurnCount>,
    opener: Vec<Opener>,
    gcd_priority_list: Vec<SkillPriorityInfo>,
    ogcd_priority_list: Vec<SkillPriorityInfo>,
}

impl PriorityTable for SagePriorityTable {
    fn add_additional_skills(&self, _: &mut Vec<SkillUsageInfo>, _: &FfxivPlayer) {}

    fn get_opener_len(&self) -> usize {
        self.opener.len()
    }

    fn get_opener_at(&self, index: usize) -> Opener {
        self.opener[index].clone()
    }

    fn get_gcd_priority_table(&self) -> &Vec<SkillPriorityInfo> {
        &self.gcd_priority_list
    }

    fn get_ogcd_priority_table(&self) -> &Vec<SkillPriorityInfo> {
        &self.ogcd_priority_list
    }

    fn increment_turn(&self) {
        *self.turn_count.borrow_mut() += 1;
    }

    fn get_turn_count(&self) -> IdType {
        *self.turn_count.borrow()
    }
}

impl SagePriorityTable {
    pub fn new() -> Self {
        Self {
            turn_count: RefCell::new(0),
            opener: make_sage_opener(),
            gcd_priority_list: make_sage_gcd_priority_table(),
            ogcd_priority_list: Vec::new(),
        }
    }
}

impl FfxivPlayer {}

use crate::id_entity::IdEntity;
use crate::jobs_skill_data::white_mage::abilities::WhitemageDatabase;
use crate::rotation::priority_table::SkillPrerequisite::Not;
use crate::rotation::priority_table::{Opener, PriorityTable, SkillPrerequisite};
use crate::rotation::SkillPriorityInfo;
use crate::{IdType, TurnCount};
use std::cell::RefCell;

#[derive(Clone)]
pub(crate) struct WhitemagePriorityTable {
    turn_count: RefCell<TurnCount>,
    opener: Vec<Opener>,

    gcd_priority_table: Vec<SkillPriorityInfo>,
    ogcd_priority_table: Vec<SkillPriorityInfo>,
}

impl PriorityTable for WhitemagePriorityTable {
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

impl WhitemagePriorityTable {
    pub fn new(player_id: IdType) -> Self {
        let db = WhitemageDatabase::new(player_id);
        Self {
            turn_count: RefCell::new(0),
            opener: make_whitemage_opener(&db),
            gcd_priority_table: make_whitemage_gcd_priority_table(&db),
            ogcd_priority_table: vec![],
        }
    }
}

pub(crate) fn make_whitemage_opener(db: &WhitemageDatabase) -> Vec<Opener> {
    vec![
        Opener::GcdOpener(db.glare3.get_id()),
        Opener::OgcdOpener((None, None)),
        Opener::GcdOpener(db.dia.get_id()),
        Opener::OgcdOpener((None, None)),
    ]
}

pub(crate) fn make_whitemage_gcd_priority_table(db: &WhitemageDatabase) -> Vec<SkillPriorityInfo> {
    vec![
        SkillPriorityInfo {
            skill_id: db.afflatus_misery.get_id(),
            prerequisite: Some(SkillPrerequisite::MillisecondsBeforeBurst(0)),
        },
        SkillPriorityInfo {
            skill_id: db.dia.get_id(),
            prerequisite: Some(SkillPrerequisite::BufforDebuffLessThan(
                db.dia_dot.get_id(),
                2500,
            )),
        },
        SkillPriorityInfo {
            skill_id: db.afflatus_rapture.get_id(),
            prerequisite: Some(Not(Box::new(SkillPrerequisite::MillisecondsBeforeBurst(0)))),
        },
        SkillPriorityInfo {
            skill_id: db.glare3.get_id(),
            prerequisite: None,
        },
    ]
}

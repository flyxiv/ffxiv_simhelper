use crate::id_entity::IdEntity;
use crate::jobs_skill_data::white_mage::abilities::WhitemageDatabase;
use crate::rotation::priority_table::SkillPrerequisite::{HasBufforDebuff, HasResource, Not, Or};
use crate::rotation::priority_table::{Opener, PriorityTable, SkillPrerequisite};
use crate::rotation::SkillPriorityInfo;
use crate::types::{IdType, PlayerIdType};
use std::cell::RefCell;

#[derive(Clone)]
pub(crate) struct WhitemagePriorityTable {
    turn_count: RefCell<IdType>,
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
    pub fn new(player_id: PlayerIdType) -> Self {
        let db = WhitemageDatabase::new(player_id);
        Self {
            turn_count: RefCell::new(0),
            opener: make_whitemage_opener(&db),
            gcd_priority_table: make_whitemage_gcd_priority_table(&db),
            ogcd_priority_table: make_whitemage_ogcd_priority_table(&db),
        }
    }
}

pub(crate) fn make_whitemage_opener(db: &WhitemageDatabase) -> Vec<Opener> {
    vec![
        Opener::GcdOpener(db.glare3.get_id()),
        Opener::OgcdOpener((None, None)),
        Opener::GcdOpener(db.dia.get_id()),
        Opener::OgcdOpener((Some(db.potion.get_id()), None)),
        Opener::GcdOpener(db.glare3.get_id()),
        Opener::OgcdOpener((Some(db.presence_of_mind.get_id()), None)),
        Opener::GcdOpener(db.glare3.get_id()),
        Opener::OgcdOpener((None, None)),
        Opener::GcdOpener(db.glare3.get_id()),
        Opener::OgcdOpener((Some(db.assize.get_id()), None)),
    ]
}

pub(crate) fn make_whitemage_gcd_priority_table(db: &WhitemageDatabase) -> Vec<SkillPriorityInfo> {
    vec![
        SkillPriorityInfo {
            skill_id: db.afflatus_misery.get_id(),
            prerequisite: Some(Or(
                Box::new(HasResource(1, 1)),
                Box::new(SkillPrerequisite::MillisecondsBeforeBurst(0)),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.dia.get_id(),
            prerequisite: Some(Or(
                Box::new(SkillPrerequisite::BufforDebuffLessThan(
                    db.dia_dot.get_id(),
                    1500,
                )),
                Box::new(Not(Box::new(HasBufforDebuff(db.dia_dot.get_id())))),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.glare4.get_id(),
            prerequisite: None,
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

pub(crate) fn make_whitemage_ogcd_priority_table(db: &WhitemageDatabase) -> Vec<SkillPriorityInfo> {
    vec![
        SkillPriorityInfo {
            skill_id: db.potion.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.presence_of_mind.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.assize.get_id(),
            prerequisite: None,
        },
    ]
}

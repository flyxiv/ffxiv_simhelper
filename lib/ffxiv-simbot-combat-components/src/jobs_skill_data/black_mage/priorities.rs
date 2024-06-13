use crate::id_entity::IdEntity;
use crate::jobs_skill_data::black_mage::abilities::BlackmageDatabase;
use crate::rotation::priority_table::SkillPrerequisite::{And, Combo, Not, Or};
use crate::rotation::priority_table::{Opener, PriorityTable, SkillPrerequisite};
use crate::rotation::SkillPriorityInfo;
use crate::{IdType, TurnCount};
use std::cell::RefCell;

#[derive(Clone)]
pub(crate) struct BlackmagePriorityTable {
    turn_count: RefCell<TurnCount>,
    opener: Vec<Opener>,

    gcd_priority_table: Vec<SkillPriorityInfo>,
    ogcd_priority_table: Vec<SkillPriorityInfo>,
}

impl PriorityTable for BlackmagePriorityTable {
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

impl BlackmagePriorityTable {
    pub fn new(player_id: IdType) -> Self {
        let db = BlackmageDatabase::new(player_id);
        Self {
            turn_count: RefCell::new(0),
            opener: make_blackmage_opener(&db),
            gcd_priority_table: make_blackmage_gcd_priority_table(&db),
            ogcd_priority_table: make_blackmage_ogcd_priority_table(&db),
        }
    }
}

pub(crate) fn make_blackmage_opener(db: &BlackmageDatabase) -> Vec<Opener> {
    vec![
        Opener::OgcdOpener((Some(db.sharpcast.get_id()), None)),
        Opener::GcdOpener(db.blizzard3_opener.get_id()),
        Opener::OgcdOpener((None, None)),
        Opener::GcdOpener(db.thunder3.get_id()),
        Opener::OgcdOpener((None, None)),
        Opener::GcdOpener(db.blizzard4.get_id()),
        Opener::OgcdOpener((None, None)),
        Opener::GcdOpener(db.fire3_ice.get_id()),
        Opener::OgcdOpener((Some(db.triplecast.get_id()), None)),
        Opener::GcdOpener(db.fire4_triplecast.get_id()),
        Opener::OgcdOpener((Some(db.leylines.get_id()), Some(db.amplifier.get_id()))),
    ]
}

pub(crate) fn make_blackmage_gcd_priority_table(db: &BlackmageDatabase) -> Vec<SkillPriorityInfo> {
    vec![
        SkillPriorityInfo {
            skill_id: db.xenoglossy.get_id(),
            prerequisite: Some(Or(Box::new(Combo(Some(2))), Box::new(Combo(Some(3))))),
        },
        SkillPriorityInfo {
            skill_id: db.thunder3_procced.get_id(),
            prerequisite: Some(Or(
                Box::new(SkillPrerequisite::BufforDebuffLessThan(
                    db.thunder3_dot.get_id(),
                    3000,
                )),
                Box::new(Not(Box::new(SkillPrerequisite::HasBufforDebuff(
                    db.thunder3_dot.get_id(),
                )))),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.fire3_ice.get_id(),
            prerequisite: Some(Combo(Some(3))),
        },
        SkillPriorityInfo {
            skill_id: db.fire3_f1.get_id(),
            prerequisite: Some(SkillPrerequisite::HasBufforDebuff(db.astral_fire1.get_id())),
        },
        SkillPriorityInfo {
            skill_id: db.blizzard3.get_id(),
            prerequisite: Some(Combo(Some(1))),
        },
        SkillPriorityInfo {
            skill_id: db.blizzard4.get_id(),
            prerequisite: Some(Combo(Some(2))),
        },
        SkillPriorityInfo {
            skill_id: db.paradox.get_id(),
            prerequisite: Some(SkillPrerequisite::BufforDebuffLessThan(
                db.astral_fire3.get_id(),
                6000,
            )),
        },
        SkillPriorityInfo {
            skill_id: db.despair_triplecast.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.despair_swiftcast.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.despair.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.fire4_triplecast.get_id(),
            prerequisite: Some(SkillPrerequisite::HasBufforDebuff(db.astral_fire3.get_id())),
        },
        SkillPriorityInfo {
            skill_id: db.fire4.get_id(),
            prerequisite: Some(SkillPrerequisite::HasBufforDebuff(db.astral_fire3.get_id())),
        },
    ]
}

pub(crate) fn make_blackmage_ogcd_priority_table(db: &BlackmageDatabase) -> Vec<SkillPriorityInfo> {
    vec![
        SkillPriorityInfo {
            skill_id: db.leylines.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.amplifier.get_id(),
            prerequisite: Some(Not(Box::new(SkillPrerequisite::HasResource(0, 2)))),
        },
        SkillPriorityInfo {
            skill_id: db.sharpcast.get_id(),
            prerequisite: Some(Not(Box::new(SkillPrerequisite::HasBufforDebuff(
                db.sharpcast.get_id(),
            )))),
        },
        SkillPriorityInfo {
            skill_id: db.triplecast.get_id(),
            prerequisite: Some(SkillPrerequisite::HasResource(2, 4)),
        },
        SkillPriorityInfo {
            skill_id: db.swiftcast.get_id(),
            prerequisite: Some(And(
                Box::new(SkillPrerequisite::HasResource(2, 6)),
                Box::new(SkillPrerequisite::BufforDebuffLessThan(
                    db.astral_fire3.get_id(),
                    3000,
                )),
            )),
        },
    ]
}

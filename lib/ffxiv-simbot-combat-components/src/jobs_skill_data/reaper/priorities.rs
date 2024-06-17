use crate::event::FfxivEventQueue;
use crate::rotation::priority_table::{Opener, PriorityTable, SkillPrerequisite};
use crate::rotation::SkillPriorityInfo;
use crate::{IdType, TurnCount};
use std::cell::RefCell;

use crate::id_entity::IdEntity;
use crate::jobs_skill_data::reaper::abilities::ReaperDatabase;
use crate::jobs_skill_data::samurai::abilities::SamuraiDatabase;
use crate::rotation::priority_table::Opener::{GcdOpener, OgcdOpener};
use crate::rotation::priority_table::SkillPrerequisite::{
    And, BufforDebuffLessThan, Combo, HasBufforDebuff, HasResource, MillisecondsBeforeBurst, Not,
    Or, RelatedSkillCooldownLessOrEqualThan,
};

#[derive(Clone)]
pub(crate) struct ReaperPriorityTable {
    turn_count: RefCell<TurnCount>,
    opener: Vec<Opener>,

    gcd_priority_table: Vec<SkillPriorityInfo>,
    ogcd_priority_table: Vec<SkillPriorityInfo>,
}

impl PriorityTable for ReaperPriorityTable {
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

impl ReaperPriorityTable {
    pub fn new(player_id: IdType, player_count: usize) -> Self {
        let db = ReaperDatabase::new(player_id, player_count);
        Self {
            turn_count: RefCell::new(0),
            opener: make_reaper_opener(&db),
            gcd_priority_table: make_reaper_gcd_priority_table(&db),
            ogcd_priority_table: make_reaper_ogcd_priority_table(&db),
        }
    }
}

pub(crate) fn make_reaper_opener(db: &ReaperDatabase) -> Vec<Opener> {
    vec![
        GcdOpener(db.shadow_of_death.get_id()),
        OgcdOpener((Some(db.potion.get_id()), None)),
        GcdOpener(db.soul_slice.get_id()),
        OgcdOpener((Some(db.arcane_circle.get_id()), None)),
        GcdOpener(db.soul_slice.get_id()),
        OgcdOpener((None, None)),
        GcdOpener(db.harvest_moon.get_id()),
        OgcdOpener((None, None)),
        GcdOpener(db.plentiful_harvest.get_id()),
        OgcdOpener((Some(db.enshroud.get_id()), None)),
        GcdOpener(db.void_reaping.get_id()),
        OgcdOpener((None, None)),
        GcdOpener(db.cross_reaping.get_id()),
        OgcdOpener((Some(db.lemures_slice.get_id()), None)),
        GcdOpener(db.void_reaping.get_id()),
        OgcdOpener((None, None)),
        GcdOpener(db.cross_reaping.get_id()),
        OgcdOpener((Some(db.lemures_slice.get_id()), None)),
        GcdOpener(db.communio.get_id()),
        OgcdOpener((Some(db.gluttony.get_id()), None)),
        GcdOpener(db.gallows.get_id()),
        OgcdOpener((None, None)),
        GcdOpener(db.enhanced_gibbet.get_id()),
    ]
}

pub(crate) fn make_reaper_gcd_priority_table(db: &ReaperDatabase) -> Vec<SkillPriorityInfo> {
    vec![
        SkillPriorityInfo {
            skill_id: db.communio.get_id(),
            prerequisite: Some(HasResource(3, 1)),
        },
        SkillPriorityInfo {
            skill_id: db.shadow_of_death.get_id(),
            prerequisite: Some(And(
                Box::new(BufforDebuffLessThan(db.shadow_of_death.get_id(), 37000)),
                Box::new(And(
                    Box::new(HasBufforDebuff(db.enshroud.get_id())),
                    Box::new(MillisecondsBeforeBurst(15000)),
                )),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.cross_reaping.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.void_reaping.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.enhanced_gibbet.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.enhanced_gallows.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.shadow_of_death.get_id(),
            prerequisite: Some(Or(
                Box::new(BufforDebuffLessThan(
                    db.shadow_of_death_debuff.get_id(),
                    5000,
                )),
                Box::new(Not(Box::new(HasBufforDebuff(
                    db.shadow_of_death_debuff.get_id(),
                )))),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.plentiful_harvest.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.soul_slice.get_id(),
            prerequisite: Some(Not(Box::new(HasResource(0, 60)))),
        },
        SkillPriorityInfo {
            skill_id: db.infernal_slice.get_id(),
            prerequisite: Some(Combo(Some(3))),
        },
        SkillPriorityInfo {
            skill_id: db.waxing_slice.get_id(),
            prerequisite: Some(Combo(Some(2))),
        },
        SkillPriorityInfo {
            skill_id: db.slice.get_id(),
            prerequisite: None,
        },
    ]
}

pub(crate) fn make_reaper_ogcd_priority_table(db: &ReaperDatabase) -> Vec<SkillPriorityInfo> {
    vec![
        SkillPriorityInfo {
            skill_id: db.potion.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.arcane_circle.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.lemures_slice.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.gluttony.get_id(),
            prerequisite: Some(And(
                Box::new(Not(Box::new(HasBufforDebuff(db.enshroud_status.get_id())))),
                Box::new(Not(Box::new(HasResource(2, 1)))),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.blood_stalk.get_id(),
            prerequisite: Some(And(
                Box::new(Or(
                    Box::new(RelatedSkillCooldownLessOrEqualThan(
                        db.soul_slice.get_id(),
                        30000,
                    )),
                    Box::new(HasResource(0, 90)),
                )),
                Box::new(Not(Box::new(HasBufforDebuff(db.enshroud_status.get_id())))),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.blood_stalk.get_id(),
            prerequisite: Some(And(
                Box::new(RelatedSkillCooldownLessOrEqualThan(
                    db.soul_slice.get_id(),
                    30000,
                )),
                Box::new(Not(Box::new(HasBufforDebuff(db.enshroud_status.get_id())))),
            )),
        },
    ]
}

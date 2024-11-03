use crate::id_entity::IdEntity;
use crate::jobs_skill_data::reaper::abilities::ReaperDatabase;
use crate::rotation::priority_table::Opener::{GcdOpener, OgcdOpener};
use crate::rotation::priority_table::SkillPrerequisite::{
    And, BufforDebuffLessThan, Combo, ComboTimeLeftLessOrEqualTo, HasBufforDebuff, HasResource,
    HasResourceExactly, MillisecondsBeforeBurst, Not, Or, RelatedSkillCooldownLessOrEqualThan,
};
use crate::rotation::priority_table::{Opener, PriorityTable};
use crate::rotation::SkillPriorityInfo;
use crate::types::{PlayerIdType, SkillIdType};
use std::cell::RefCell;

#[derive(Clone)]
pub struct ReaperPriorityTable {
    turn_count: RefCell<SkillIdType>,
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

    fn get_gcd_priority_table(&self) -> &[SkillPriorityInfo] {
        &self.gcd_priority_table
    }

    fn get_ogcd_priority_table(&self) -> &[SkillPriorityInfo] {
        &self.ogcd_priority_table
    }

    fn increment_turn(&self) {
        *self.turn_count.borrow_mut() += 1;
    }

    fn get_turn_count(&self) -> SkillIdType {
        *self.turn_count.borrow()
    }
}

impl ReaperPriorityTable {
    pub fn new(player_id: PlayerIdType, player_count: usize, use_pots: bool) -> Self {
        let db = ReaperDatabase::new(player_id, player_count);
        Self {
            turn_count: RefCell::new(0),
            opener: make_reaper_opener(&db, use_pots),
            gcd_priority_table: make_reaper_gcd_priority_table(&db),
            ogcd_priority_table: make_reaper_ogcd_priority_table(&db, use_pots),
        }
    }
}

pub(crate) fn make_reaper_opener(db: &ReaperDatabase, use_pots: bool) -> Vec<Opener> {
    let mut openers = if use_pots {
        vec![
            GcdOpener(db.shadow_of_death.get_id()),
            OgcdOpener((Some(db.potion.get_id()), None)),
        ]
    } else {
        vec![
            GcdOpener(db.shadow_of_death.get_id()),
            OgcdOpener((None, None)),
        ]
    };

    openers.extend(vec![
        GcdOpener(db.soul_slice.get_id()),
        OgcdOpener((Some(db.arcane_circle.get_id()), None)),
        GcdOpener(db.soul_slice.get_id()),
        OgcdOpener((Some(db.blood_stalk.get_id()), None)),
        GcdOpener(db.enhanced_gallows.get_id()),
        OgcdOpener((None, None)),
        GcdOpener(db.plentiful_harvest.get_id()),
        OgcdOpener((
            Some(db.enshroud_host.get_id()),
            Some(db.sacrificium.get_id()),
        )),
        GcdOpener(db.void_reaping.get_id()),
        OgcdOpener((None, None)),
        GcdOpener(db.cross_reaping.get_id()),
        OgcdOpener((Some(db.lemures_slice.get_id()), None)),
        GcdOpener(db.void_reaping.get_id()),
        OgcdOpener((None, None)),
        GcdOpener(db.cross_reaping.get_id()),
        OgcdOpener((Some(db.lemures_slice.get_id()), None)),
        GcdOpener(db.communio.get_id()),
        OgcdOpener((None, None)),
        GcdOpener(db.perfectio.get_id()),
        OgcdOpener((Some(db.gluttony.get_id()), None)),
        GcdOpener(db.executioners_gallows.get_id()),
        OgcdOpener((None, None)),
        GcdOpener(db.executioners_gibbet.get_id()),
        OgcdOpener((None, None)),
        GcdOpener(db.harvest_moon.get_id()),
    ]);

    openers
}

pub(crate) fn make_reaper_gcd_priority_table(db: &ReaperDatabase) -> Vec<SkillPriorityInfo> {
    vec![
        SkillPriorityInfo {
            skill_id: db.communio.get_id(),
            prerequisite: Some(HasResourceExactly(3, 1)),
        },
        SkillPriorityInfo {
            skill_id: db.shadow_of_death.get_id(),
            prerequisite: Some(And(
                Box::new(BufforDebuffLessThan(
                    db.shadow_of_death_debuff.get_id(),
                    37000,
                )),
                Box::new(And(
                    Box::new(HasBufforDebuff(db.enshroud_status.get_id())),
                    Box::new(RelatedSkillCooldownLessOrEqualThan(
                        db.arcane_circle.get_id(),
                        7500,
                    )),
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
            skill_id: db.perfectio.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.plentiful_harvest.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.infernal_slice.get_id(),
            prerequisite: Some(And(
                Box::new(ComboTimeLeftLessOrEqualTo(10000)),
                Box::new(Combo(Some(3))),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.waxing_slice.get_id(),
            prerequisite: Some(And(
                Box::new(ComboTimeLeftLessOrEqualTo(10000)),
                Box::new(Combo(Some(2))),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.slice.get_id(),
            prerequisite: Some(ComboTimeLeftLessOrEqualTo(10000)),
        },
        SkillPriorityInfo {
            skill_id: db.executioners_gibbet.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.executioners_gallows.get_id(),
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
            prerequisite: Some(And(
                Box::new(BufforDebuffLessThan(
                    db.shadow_of_death_debuff.get_id(),
                    2600,
                )),
                Box::new(Not(Box::new(MillisecondsBeforeBurst(20000)))),
            )),
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

pub(crate) fn make_reaper_ogcd_priority_table(
    db: &ReaperDatabase,
    use_pots: bool,
) -> Vec<SkillPriorityInfo> {
    let mut ogcd_priorities = if use_pots {
        vec![
            SkillPriorityInfo {
                skill_id: db.enshroud_host.get_id(),
                prerequisite: None,
            },
            SkillPriorityInfo {
                skill_id: db.potion.get_id(),
                prerequisite: Some(MillisecondsBeforeBurst(9000)),
            },
        ]
    } else {
        vec![SkillPriorityInfo {
            skill_id: db.enshroud_host.get_id(),
            prerequisite: None,
        }]
    };

    ogcd_priorities.extend(vec![
        SkillPriorityInfo {
            skill_id: db.sacrificium.get_id(),
            prerequisite: Some(HasBufforDebuff(db.arcane_circle_buff.get_id())),
        },
        SkillPriorityInfo {
            skill_id: db.arcane_circle.get_id(),
            prerequisite: Some(HasResourceExactly(3, 4)),
        },
        SkillPriorityInfo {
            skill_id: db.enshroud.get_id(),
            prerequisite: Some(And(
                Box::new(Or(
                    Box::new(And(
                        Box::new(And(
                            Box::new(RelatedSkillCooldownLessOrEqualThan(
                                db.gluttony.get_id(),
                                15000,
                            )),
                            Box::new(Not(Box::new(RelatedSkillCooldownLessOrEqualThan(
                                db.gluttony.get_id(),
                                11000,
                            )))),
                        )),
                        Box::new(HasResource(0, 70)),
                    )),
                    Box::new(RelatedSkillCooldownLessOrEqualThan(
                        db.arcane_circle.get_id(),
                        6500,
                    )),
                )),
                Box::new(And(
                    Box::new(Not(Box::new(HasBufforDebuff(db.enshroud_status.get_id())))),
                    Box::new(Not(Box::new(HasResource(6, 2)))),
                )),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.lemures_slice.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.sacrificium.get_id(),
            prerequisite: Some(Not(Box::new(HasResource(3, 5)))),
        },
        SkillPriorityInfo {
            skill_id: db.gluttony.get_id(),
            prerequisite: Some(And(
                Box::new(Not(Box::new(HasBufforDebuff(db.enshroud_status.get_id())))),
                Box::new(Not(Box::new(HasResource(6, 2)))),
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
                    Box::new(HasResource(0, 100)),
                )),
                Box::new(And(
                    Box::new(And(
                        Box::new(Not(Box::new(HasBufforDebuff(db.enshroud_status.get_id())))),
                        Box::new(And(
                            Box::new(Not(Box::new(MillisecondsBeforeBurst(0)))),
                            Box::new(And(
                                Box::new(Not(Box::new(HasResource(6, 2)))),
                                Box::new(Not(Box::new(HasResource(5, 1)))),
                            )),
                        )),
                    )),
                    Box::new(Not(Box::new(RelatedSkillCooldownLessOrEqualThan(
                        db.gluttony.get_id(),
                        2500,
                    )))),
                )),
            )),
        },
    ]);

    ogcd_priorities
}

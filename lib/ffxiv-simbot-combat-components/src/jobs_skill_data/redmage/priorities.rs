use crate::id_entity::IdEntity;
use crate::jobs_skill_data::redmage::abilities::RedmageDatabase;
use crate::rotation::priority_table::SkillPrerequisite::{
    And, Combo, HasBufforDebuff, HasResource, MillisecondsBeforeBurst, Not, Or,
};
use crate::rotation::priority_table::{Opener, PriorityTable, SkillPrerequisite};
use crate::rotation::SkillPriorityInfo;
use crate::types::{PlayerIdType, ResourceType, SkillIdType};
use std::cell::RefCell;

const MANA_DIFFERENCE_THRESHOLD: ResourceType = 30;
const VERSTONE_PROC_ID: SkillIdType = 1804;
const VERFIRE_PROC_ID: SkillIdType = 1805;

#[derive(Clone)]
pub struct RedmagePriorityTable {
    turn_count: RefCell<SkillIdType>,
    opener: Vec<Opener>,

    gcd_priority_table: Vec<SkillPriorityInfo>,
    ogcd_priority_table: Vec<SkillPriorityInfo>,
}

impl PriorityTable for RedmagePriorityTable {
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

impl RedmagePriorityTable {
    pub fn new(player_id: PlayerIdType, use_pots: bool) -> Self {
        let db = RedmageDatabase::new(player_id);
        Self {
            turn_count: RefCell::new(0),
            opener: make_redmage_opener(&db, use_pots),
            gcd_priority_table: make_redmage_gcd_priority_table(&db),
            ogcd_priority_table: make_redmage_ogcd_priority_table(&db, use_pots),
        }
    }
}

pub(crate) fn make_redmage_opener(db: &RedmageDatabase, use_pots: bool) -> Vec<Opener> {
    let mut openers = if use_pots {
        vec![
            Opener::GcdOpener(db.veraero_iii.get_id()),
            Opener::OgcdOpener((None, None)),
            Opener::GcdOpener(db.verthunder_iii_dual.get_id()),
            Opener::OgcdOpener((Some(db.swiftcast.get_id()), Some(db.potion.get_id()))),
        ]
    } else {
        vec![
            Opener::GcdOpener(db.veraero_iii.get_id()),
            Opener::OgcdOpener((None, None)),
            Opener::GcdOpener(db.verthunder_iii_dual.get_id()),
            Opener::OgcdOpener((Some(db.swiftcast.get_id()), None)),
        ]
    };

    openers.extend(vec![
        Opener::GcdOpener(db.grand_impact.get_id()),
        Opener::OgcdOpener((Some(db.fleche.get_id()), Some(db.acceleration.get_id()))),
        Opener::GcdOpener(db.verthunder_iii_swift.get_id()),
        Opener::OgcdOpener((Some(db.embolden.get_id()), Some(db.manafication.get_id()))),
        Opener::GcdOpener(db.manafication_riposte.get_id()),
        Opener::OgcdOpener((Some(db.contre_sixte.get_id()), None)),
        Opener::GcdOpener(db.manafication_zwerchhau.get_id()),
        Opener::OgcdOpener((Some(db.engagement.get_id()), None)),
        Opener::GcdOpener(db.manafication_redoublement.get_id()),
        Opener::OgcdOpener((Some(db.corps_a_corps.get_id()), None)),
        Opener::GcdOpener(db.manafication_verholy.get_id()),
        Opener::OgcdOpener((Some(db.vice_of_thorns.get_id()), None)),
        Opener::GcdOpener(db.manafication_scorch.get_id()),
        Opener::OgcdOpener((
            Some(db.corps_a_corps.get_id()),
            Some(db.engagement.get_id()),
        )),
        Opener::GcdOpener(db.manafication_resolution.get_id()),
        Opener::OgcdOpener((Some(db.prefulgence.get_id()), None)),
    ]);

    openers
}

pub(crate) fn make_redmage_gcd_priority_table(db: &RedmageDatabase) -> Vec<SkillPriorityInfo> {
    let mut priority = make_magic_pair_priority(
        11,
        db.manafication_verholy.get_id(),
        db.manafication_verflare.get_id(),
    );

    priority.extend(make_magic_pair_priority(
        6,
        db.veraero_iii_accel.get_id(),
        db.verthunder_iii_accel.get_id(),
    ));
    priority.extend(make_magic_pair_priority(
        6,
        db.veraero_iii_swift.get_id(),
        db.verthunder_iii_swift.get_id(),
    ));

    priority.extend(vec![
        SkillPriorityInfo {
            skill_id: db.manafication_resolution.get_id(),
            prerequisite: Some(Combo(Some(5))),
        },
        SkillPriorityInfo {
            skill_id: db.manafication_scorch.get_id(),
            prerequisite: Some(Combo(Some(4))),
        },
        SkillPriorityInfo {
            skill_id: db.manafication_redoublement.get_id(),
            prerequisite: Some(Combo(Some(3))),
        },
        SkillPriorityInfo {
            skill_id: db.manafication_zwerchhau.get_id(),
            prerequisite: Some(Combo(Some(2))),
        },
        SkillPriorityInfo {
            skill_id: db.manafication_riposte.get_id(),
            prerequisite: Some(Not(Box::new(HasBufforDebuff(db.dualcast_buff.get_id())))),
        },
        SkillPriorityInfo {
            skill_id: db.enchanted_riposte.get_id(),
            prerequisite: Some(And(
                Box::new(And(
                    Box::new(Combo(Some(0))),
                    Box::new(Not(Box::new(HasBufforDebuff(db.dualcast_buff.get_id())))),
                )),
                Box::new(Or(
                    Box::new(And(
                        Box::new(MillisecondsBeforeBurst(3000)),
                        Box::new(And(
                            Box::new(HasResource(0, 50)),
                            Box::new(And(Box::new(HasResource(1, 50)), Box::new(Combo(Some(0))))),
                        )),
                    )),
                    Box::new(Or(
                        Box::new(HasResource(0, 90)),
                        Box::new(HasResource(1, 90)),
                    )),
                )),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.enchanted_riposte.get_id(),
            prerequisite: Some(And(
                Box::new(And(
                    Box::new(And(
                        Box::new(Combo(Some(0))),
                        Box::new(Not(Box::new(HasBufforDebuff(db.dualcast_buff.get_id())))),
                    )),
                    Box::new(Not(Box::new(MillisecondsBeforeBurst(40000)))),
                )),
                Box::new(And(
                    Box::new(HasResource(0, 50)),
                    Box::new(HasResource(1, 50)),
                )),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.enchanted_zwerchhau.get_id(),
            prerequisite: Some(Combo(Some(2))),
        },
        SkillPriorityInfo {
            skill_id: db.enchanted_redoublement.get_id(),
            prerequisite: Some(Combo(Some(3))),
        },
        SkillPriorityInfo {
            skill_id: db.scorch.get_id(),
            prerequisite: Some(Combo(Some(4))),
        },
        SkillPriorityInfo {
            skill_id: db.resolution.get_id(),
            prerequisite: Some(Combo(Some(5))),
        },
    ]);

    priority.extend(make_magic_pair_priority(
        11,
        db.verholy.get_id(),
        db.verflare.get_id(),
    ));

    priority.push(SkillPriorityInfo {
        skill_id: db.grand_impact.get_id(),
        prerequisite: None,
    });

    priority.extend(make_magic_pair_priority(
        6,
        db.veraero_iii_dual.get_id(),
        db.verthunder_iii_dual.get_id(),
    ));

    priority.extend(make_magic_pair_priority(
        5,
        db.verstone.get_id(),
        db.verfire.get_id(),
    ));

    priority.push(SkillPriorityInfo {
        skill_id: db.jolt_iii.get_id(),
        prerequisite: None,
    });

    priority
}

pub(crate) fn make_redmage_ogcd_priority_table(
    db: &RedmageDatabase,
    use_pots: bool,
) -> Vec<SkillPriorityInfo> {
    let mut ogcd_priorities = if use_pots {
        vec![SkillPriorityInfo {
            skill_id: db.potion.get_id(),
            prerequisite: Some(MillisecondsBeforeBurst(4000)),
        }]
    } else {
        vec![]
    };

    ogcd_priorities.extend(vec![
        SkillPriorityInfo {
            skill_id: db.manafication.get_id(),
            prerequisite: Some(HasBufforDebuff(db.finish_ready.get_id())),
        },
        SkillPriorityInfo {
            skill_id: db.embolden.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.fleche.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.contre_sixte.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.engagement.get_id(),
            prerequisite: Some(Or(
                Box::new(MillisecondsBeforeBurst(0)),
                Box::new(Not(Box::new(MillisecondsBeforeBurst(35000)))),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.corps_a_corps.get_id(),
            prerequisite: Some(Or(
                Box::new(MillisecondsBeforeBurst(0)),
                Box::new(Not(Box::new(MillisecondsBeforeBurst(35000)))),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.prefulgence.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.vice_of_thorns.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.acceleration.get_id(),
            prerequisite: Some(And(
                Box::new(Combo(Some(0))),
                Box::new(Or(
                    Box::new(Not(Box::new(HasBufforDebuff(db.verstone_ready.get_id())))),
                    Box::new(Not(Box::new(HasBufforDebuff(db.verfire_ready.get_id())))),
                )),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.swiftcast.get_id(),
            prerequisite: Some(And(
                Box::new(And(
                    Box::new(Combo(Some(0))),
                    Box::new(Not(Box::new(MillisecondsBeforeBurst(0)))),
                )),
                Box::new(Or(
                    Box::new(Not(Box::new(HasBufforDebuff(
                        db.acceleration_buff.get_id(),
                    )))),
                    Box::new(Not(Box::new(HasBufforDebuff(db.dualcast_buff.get_id())))),
                )),
            )),
        },
    ]);

    ogcd_priorities
}

fn make_magic_pair_priority(
    mana_generation: ResourceType,
    white_mana_skill_id: SkillIdType,
    black_mana_skill_id: SkillIdType,
) -> Vec<SkillPriorityInfo> {
    let threshold = MANA_DIFFERENCE_THRESHOLD - mana_generation + 1;
    vec![
        SkillPriorityInfo {
            skill_id: white_mana_skill_id,
            prerequisite: Some(
                SkillPrerequisite::ResourceGreaterOrEqualThanAnotherResourceBy(1, 0, threshold),
            ),
        },
        SkillPriorityInfo {
            skill_id: black_mana_skill_id,
            prerequisite: Some(
                SkillPrerequisite::ResourceGreaterOrEqualThanAnotherResourceBy(0, 1, threshold),
            ),
        },
        SkillPriorityInfo {
            skill_id: white_mana_skill_id,
            prerequisite: Some(And(
                Box::new(SkillPrerequisite::ResourceGreaterOrEqualThanAnotherResourceBy(1, 0, 0)),
                Box::new(Not(Box::new(HasBufforDebuff(VERSTONE_PROC_ID)))),
            )),
        },
        SkillPriorityInfo {
            skill_id: black_mana_skill_id,
            prerequisite: Some(And(
                Box::new(SkillPrerequisite::ResourceGreaterOrEqualThanAnotherResourceBy(0, 1, 0)),
                Box::new(Not(Box::new(HasBufforDebuff(VERFIRE_PROC_ID)))),
            )),
        },
        SkillPriorityInfo {
            skill_id: white_mana_skill_id,
            prerequisite: Some(Not(Box::new(HasBufforDebuff(VERSTONE_PROC_ID)))),
        },
        SkillPriorityInfo {
            skill_id: black_mana_skill_id,
            prerequisite: Some(Not(Box::new(HasBufforDebuff(VERFIRE_PROC_ID)))),
        },
        SkillPriorityInfo {
            skill_id: white_mana_skill_id,
            prerequisite: Some(
                SkillPrerequisite::ResourceGreaterOrEqualThanAnotherResourceBy(1, 0, 0),
            ),
        },
        SkillPriorityInfo {
            skill_id: black_mana_skill_id,
            prerequisite: Some(
                SkillPrerequisite::ResourceGreaterOrEqualThanAnotherResourceBy(0, 1, 0),
            ),
        },
    ]
}

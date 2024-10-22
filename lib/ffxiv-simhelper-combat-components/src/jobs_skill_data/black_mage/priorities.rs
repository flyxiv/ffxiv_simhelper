use crate::id_entity::IdEntity;
use crate::jobs_skill_data::black_mage::abilities::BlackmageDatabase;
use crate::rotation::priority_table::SkillPrerequisite::{
    And, BufforDebuffLessThan, Combo, HasBufforDebuff, HasResource, HasSkillStacks, Not, Or,
    RelatedSkillCooldownLessOrEqualThan,
};
use crate::rotation::priority_table::{Opener, PriorityTable};
use crate::rotation::SkillPriorityInfo;
use crate::types::{PlayerIdType, SkillIdType};
use std::cell::RefCell;

#[derive(Clone)]
pub struct BlackmagePriorityTable {
    turn_count: RefCell<SkillIdType>,
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

impl BlackmagePriorityTable {
    pub fn new(player_id: PlayerIdType, use_pot: bool) -> Self {
        let db = BlackmageDatabase::new(player_id);
        Self {
            turn_count: RefCell::new(0),
            opener: make_blackmage_opener(&db, use_pot),
            gcd_priority_table: make_blackmage_gcd_priority_table(&db),
            ogcd_priority_table: make_blackmage_ogcd_priority_table(&db, use_pot),
        }
    }
}

pub(crate) fn make_blackmage_opener(db: &BlackmageDatabase, use_pot: bool) -> Vec<Opener> {
    let mut opener = vec![
        Opener::GcdOpener(db.fire3_opener.get_id()),
        Opener::OgcdOpener((None, None)),
        Opener::GcdOpener(db.high_thunder.get_id()),
        Opener::OgcdOpener((Some(db.swiftcast.get_id()), Some(db.amplifier.get_id()))),
        Opener::GcdOpener(db.fire4_swiftcast.get_id()),
    ];

    if use_pot {
        opener.push(Opener::OgcdOpener((Some(db.potion.get_id()), None)));
    } else {
        opener.push(Opener::OgcdOpener((None, None)));
    }

    opener.extend(vec![
        Opener::GcdOpener(db.fire4.get_id()),
        Opener::OgcdOpener((None, None)),
        Opener::GcdOpener(db.xenoglossy.get_id()),
        Opener::OgcdOpener((Some(db.triplecast.get_id()), Some(db.leylines.get_id()))),
        Opener::GcdOpener(db.fire4_triplecast.get_id()),
        Opener::OgcdOpener((None, None)),
        Opener::GcdOpener(db.fire4_triplecast.get_id()),
        Opener::OgcdOpener((None, None)),
        Opener::GcdOpener(db.despair_triplecast.get_id()),
        Opener::OgcdOpener((Some(db.manafont.get_id()), Some(db.triplecast.get_id()))),
        Opener::GcdOpener(db.fire4_triplecast.get_id()),
        Opener::OgcdOpener((None, None)),
        Opener::GcdOpener(db.fire4_triplecast.get_id()),
        Opener::OgcdOpener((None, None)),
        Opener::GcdOpener(db.flare_star_triplecast.get_id()),
        Opener::OgcdOpener((None, None)),
        Opener::GcdOpener(db.fire4.get_id()),
        Opener::OgcdOpener((None, None)),
        Opener::GcdOpener(db.high_thunder.get_id()),
        Opener::OgcdOpener((None, None)),
        Opener::GcdOpener(db.paradox.get_id()),
        Opener::OgcdOpener((None, None)),
        Opener::GcdOpener(db.fire4.get_id()),
        Opener::OgcdOpener((None, None)),
        Opener::GcdOpener(db.fire4.get_id()),
        Opener::OgcdOpener((None, None)),
        Opener::GcdOpener(db.fire4.get_id()),
        Opener::OgcdOpener((None, None)),
        Opener::GcdOpener(db.despair.get_id()),
        Opener::OgcdOpener((None, None)),
        Opener::GcdOpener(db.blizzard3.get_id()),
    ]);

    opener
}

pub(crate) fn make_blackmage_gcd_priority_table(db: &BlackmageDatabase) -> Vec<SkillPriorityInfo> {
    let timing_for_triplecast = And(
        Box::new(HasResource(2, 6)),
        Box::new(And(
            Box::new(HasSkillStacks(db.triplecast.get_id(), 1)),
            Box::new(Not(Box::new(RelatedSkillCooldownLessOrEqualThan(
                db.manafont.get_id(),
                1000,
            )))),
        )),
    );

    vec![
        SkillPriorityInfo {
            skill_id: db.xenoglossy.get_id(),
            prerequisite: Some(And(
                Box::new(HasBufforDebuff(db.umbral_ice1.get_id())),
                Box::new(And(
                    Box::new(Not(Box::new(HasBufforDebuff(db.swiftcast.get_id())))),
                    Box::new(Not(Box::new(HasBufforDebuff(db.triplecast_buff.get_id())))),
                )),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.xenoglossy.get_id(),
            prerequisite: Some(Or(
                Box::new(HasResource(4, 1)),
                Box::new(timing_for_triplecast),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.despair_triplecast.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.despair.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.fire_iii_proc.get_id(),
            prerequisite: Some(And(
                Box::new(HasResource(3, 1)),
                Box::new(RelatedSkillCooldownLessOrEqualThan(
                    db.manafont.get_id(),
                    2000,
                )),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.paradox.get_id(),
            prerequisite: Some(Or(
                Box::new(Combo(Some(3))),
                Box::new(And(
                    Box::new(HasBufforDebuff(db.astral_fire3.get_id())),
                    Box::new(BufforDebuffLessThan(db.astral_fire3.get_id(), 3500)),
                )),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.xenoglossy.get_id(),
            prerequisite: Some(RelatedSkillCooldownLessOrEqualThan(
                db.leylines.get_id(),
                500,
            )),
        },
        SkillPriorityInfo {
            skill_id: db.flare_star_triplecast.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.flare_star.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.high_thunder.get_id(),
            prerequisite: Some(Or(
                Box::new(BufforDebuffLessThan(db.high_thunder_dot.get_id(), 3000)),
                Box::new(Not(Box::new(HasBufforDebuff(db.high_thunder_dot.get_id())))),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.fire3_f1.get_id(),
            prerequisite: Some(HasBufforDebuff(db.astral_fire1.get_id())),
        },
        SkillPriorityInfo {
            skill_id: db.blizzard3_transpose_swift.get_id(),
            prerequisite: Some(Combo(Some(1))),
        },
        SkillPriorityInfo {
            skill_id: db.blizzard3_transpose_triplecast.get_id(),
            prerequisite: Some(Combo(Some(1))),
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
            skill_id: db.xenoglossy.get_id(),
            prerequisite: Some(And(
                Box::new(HasResource(0, 2)),
                Box::new(Or(Box::new(Combo(Some(2))), Box::new(Combo(Some(3))))),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.fire4.get_id(),
            prerequisite: Some(HasBufforDebuff(db.astral_fire3.get_id())),
        },
    ]
}

pub(crate) fn make_blackmage_ogcd_priority_table(
    db: &BlackmageDatabase,
    use_pot: bool,
) -> Vec<SkillPriorityInfo> {
    let mut ogcd_priorities = if use_pot {
        vec![SkillPriorityInfo {
            skill_id: db.potion.get_id(),
            prerequisite: None,
        }]
    } else {
        vec![]
    };
    ogcd_priorities.extend(vec![
        SkillPriorityInfo {
            skill_id: db.swiftcast.get_id(),
            prerequisite: Some(And(
                Box::new(HasBufforDebuff(db.umbral_ice1.get_id())),
                Box::new(Not(Box::new(HasBufforDebuff(db.triplecast_buff.get_id())))),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.manafont.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.triplecast.get_id(),
            prerequisite: Some(And(
                Box::new(HasResource(2, 6)),
                Box::new(Not(Box::new(RelatedSkillCooldownLessOrEqualThan(
                    db.manafont.get_id(),
                    1000,
                )))),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.transpose_fire_to_ice.get_id(),
            prerequisite: Some(Or(
                Box::new(HasBufforDebuff(db.triplecast.get_id())),
                Box::new(HasSkillStacks(db.swiftcast.get_id(), 1)),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.transpose_ice_to_fire.get_id(),
            prerequisite: Some(Combo(Some(3))),
        },
        SkillPriorityInfo {
            skill_id: db.leylines.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.amplifier.get_id(),
            prerequisite: Some(Not(Box::new(HasResource(0, 2)))),
        },
    ]);

    ogcd_priorities
}

use crate::id_entity::IdEntity;
use crate::jobs_skill_data::machinist::abilities::MachinistDatabase;
use crate::rotation::priority_table::SkillPrerequisite::{
    And, Combo, HasBufforDebuff, HasResource, MillisecondsBeforeBurst, Not, Or,
    RelatedSkillCooldownLessOrEqualThan, SkillCooldownWillComeBackMillisecondsBeforeBurst,
};
use crate::rotation::priority_table::{Opener, PriorityTable};
use crate::rotation::SkillPriorityInfo;
use crate::types::{PlayerIdType, SkillIdType};
use std::cell::RefCell;

#[derive(Clone)]
pub struct MachinistPriorityTable {
    turn_count: RefCell<SkillIdType>,
    opener: Vec<Opener>,

    gcd_priority_table: Vec<SkillPriorityInfo>,
    ogcd_priority_table: Vec<SkillPriorityInfo>,
}

impl PriorityTable for MachinistPriorityTable {
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

impl MachinistPriorityTable {
    pub fn new(player_id: PlayerIdType, use_pots: bool) -> Self {
        let db = MachinistDatabase::new(player_id);
        Self {
            turn_count: RefCell::new(0),
            opener: make_machinist_opener(&db, use_pots),
            gcd_priority_table: make_machinist_gcd_priority_table(&db),
            ogcd_priority_table: make_machinist_ogcd_priority_table(&db, use_pots),
        }
    }
}

pub(crate) fn make_machinist_opener(db: &MachinistDatabase, use_pots: bool) -> Vec<Opener> {
    let mut openers = if use_pots {
        vec![
            Opener::GcdOpener(db.reassemble_opener.get_id()),
            Opener::OgcdOpener((Some(db.potion.get_id()), None)),
        ]
    } else {
        vec![
            Opener::GcdOpener(db.reassemble_opener.get_id()),
            Opener::OgcdOpener((None, None)),
        ]
    };

    openers.extend(vec![
        Opener::GcdOpener(db.air_anchor.get_id()),
        Opener::OgcdOpener((Some(db.double_check.get_id()), Some(db.checkmate.get_id()))),
        Opener::GcdOpener(db.drill.get_id()),
        Opener::OgcdOpener((Some(db.barrel_stabilizer.get_id()), None)),
        Opener::GcdOpener(db.chainsaw.get_id()),
        Opener::OgcdOpener((None, None)),
        Opener::GcdOpener(db.excavator.get_id()),
        Opener::OgcdOpener((
            Some(db.automaton_queen.get_id()),
            Some(db.reassemble.get_id()),
        )),
        Opener::GcdOpener(db.drill_reassemble.get_id()),
        Opener::OgcdOpener((Some(db.checkmate.get_id()), Some(db.wildfire.get_id()))),
        Opener::GcdOpener(db.full_metal_field.get_id()),
        Opener::OgcdOpener((
            Some(db.double_check.get_id()),
            Some(db.hypercharge_hypercharged.get_id()),
        )),
        Opener::GcdOpener(db.blazing_shot.get_id()),
        Opener::OgcdOpener((Some(db.checkmate.get_id()), None)),
        Opener::GcdOpener(db.blazing_shot.get_id()),
        Opener::OgcdOpener((Some(db.double_check.get_id()), None)),
        Opener::GcdOpener(db.blazing_shot.get_id()),
        Opener::OgcdOpener((Some(db.checkmate.get_id()), None)),
        Opener::GcdOpener(db.blazing_shot.get_id()),
        Opener::OgcdOpener((Some(db.double_check.get_id()), None)),
        Opener::GcdOpener(db.blazing_shot.get_id()),
        Opener::OgcdOpener((Some(db.checkmate.get_id()), None)),
        Opener::GcdOpener(db.drill.get_id()),
        Opener::OgcdOpener((Some(db.double_check.get_id()), Some(db.checkmate.get_id()))),
        Opener::GcdOpener(db.heated_split_shot.get_id()),
        Opener::OgcdOpener((Some(db.double_check.get_id()), None)),
        Opener::GcdOpener(db.heated_slug_shot.get_id()),
        Opener::OgcdOpener((None, None)),
        Opener::GcdOpener(db.heated_clean_shot.get_id()),
        Opener::OgcdOpener((None, None)),
    ]);

    openers
}

pub(crate) fn make_machinist_gcd_priority_table(db: &MachinistDatabase) -> Vec<SkillPriorityInfo> {
    vec![
        SkillPriorityInfo {
            skill_id: db.excavator_reassemble.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.drill_reassemble.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.air_anchor_reassemble.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.chainsaw_reassemble.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.blazing_shot.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.air_anchor.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.chainsaw.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.excavator.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.drill.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.full_metal_field.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.heated_clean_shot.get_id(),
            prerequisite: Some(Combo(Some(3))),
        },
        SkillPriorityInfo {
            skill_id: db.heated_slug_shot.get_id(),
            prerequisite: Some(Combo(Some(2))),
        },
        SkillPriorityInfo {
            skill_id: db.heated_split_shot.get_id(),
            prerequisite: None,
        },
    ]
}

pub(crate) fn make_machinist_ogcd_priority_table(
    db: &MachinistDatabase,
    use_pots: bool,
) -> Vec<SkillPriorityInfo> {
    let can_use_hypercharge = And(
        Box::new(And(
            Box::new(Not(Box::new(RelatedSkillCooldownLessOrEqualThan(
                db.drill.get_id(),
                8000,
            )))),
            Box::new(And(
                Box::new(Not(Box::new(RelatedSkillCooldownLessOrEqualThan(
                    db.air_anchor.get_id(),
                    8000,
                )))),
                Box::new(Not(Box::new(RelatedSkillCooldownLessOrEqualThan(
                    db.chainsaw.get_id(),
                    8000,
                )))),
            )),
        )),
        Box::new(Not(Box::new(HasBufforDebuff(db.hypercharge_buff.get_id())))),
    );

    let mut ogcd_priorites = if use_pots {
        vec![SkillPriorityInfo {
            skill_id: db.potion.get_id(),
            prerequisite: Some(MillisecondsBeforeBurst(9000)),
        }]
    } else {
        vec![]
    };

    ogcd_priorites.extend(vec![
        SkillPriorityInfo {
            skill_id: db.wildfire.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.hypercharge_hypercharged.get_id(),
            prerequisite: Some(Not(Box::new(RelatedSkillCooldownLessOrEqualThan(
                db.wildfire.get_id(),
                105000,
            )))),
        },
        SkillPriorityInfo {
            skill_id: db.reassemble.get_id(),
            prerequisite: Some(And(
                Box::new(Or(
                    Box::new(RelatedSkillCooldownLessOrEqualThan(
                        db.drill.get_id(),
                        21500,
                    )),
                    Box::new(Or(
                        Box::new(RelatedSkillCooldownLessOrEqualThan(
                            db.air_anchor.get_id(),
                            1500,
                        )),
                        Box::new(RelatedSkillCooldownLessOrEqualThan(
                            db.chainsaw.get_id(),
                            1500,
                        )),
                    )),
                )),
                Box::new(And(
                    Box::new(Not(Box::new(HasBufforDebuff(db.hypercharge_buff.get_id())))),
                    Box::new(Or(
                        Box::new(MillisecondsBeforeBurst(0)),
                        Box::new(SkillCooldownWillComeBackMillisecondsBeforeBurst(
                            db.reassemble.get_id(),
                            0,
                        )),
                    )),
                )),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.hypercharge.get_id(),
            prerequisite: Some(And(
                Box::new(can_use_hypercharge),
                Box::new(Or(
                    Box::new(HasResource(0, 90)),
                    Box::new(HasResource(2, 1)),
                )),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.checkmate.get_id(),
            prerequisite: Some(Or(
                Box::new(MillisecondsBeforeBurst(0)),
                Box::new(RelatedSkillCooldownLessOrEqualThan(
                    db.checkmate.get_id(),
                    60000,
                )),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.double_check.get_id(),
            prerequisite: Some(Or(
                Box::new(MillisecondsBeforeBurst(0)),
                Box::new(RelatedSkillCooldownLessOrEqualThan(
                    db.double_check.get_id(),
                    60000,
                )),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.barrel_stabilizer.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.automaton_queen.get_id(),
            prerequisite: Some(Or(
                Box::new(And(
                    Box::new(MillisecondsBeforeBurst(3000)),
                    Box::new(HasResource(1, 5)),
                )),
                Box::new(HasResource(1, 8)),
            )),
        },
    ]);

    ogcd_priorites
}

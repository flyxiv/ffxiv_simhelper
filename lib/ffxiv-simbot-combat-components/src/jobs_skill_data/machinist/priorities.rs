use crate::id_entity::IdEntity;
use crate::jobs_skill_data::machinist::abilities::MachinistDatabase;
use crate::jobs_skill_data::ninja::abilities::NinjaDatabase;
use crate::rotation::priority_table::SkillPrerequisite::{
    And, Combo, HasResource, HasSkillStacks, MillisecondsBeforeBurst, Not, Or,
    RelatedSkillCooldownLessOrEqualThan,
};
use crate::rotation::priority_table::{Opener, PriorityTable, SkillPrerequisite};
use crate::rotation::SkillPriorityInfo;
use crate::{IdType, PotencyType, ResourceType, TimeType, TurnCount};
use std::cell::RefCell;

#[derive(Clone)]
pub(crate) struct MachinistPriorityTable {
    turn_count: RefCell<TurnCount>,
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

impl MachinistPriorityTable {
    pub fn new(player_id: IdType) -> Self {
        let db = MachinistDatabase::new(player_id);
        Self {
            turn_count: RefCell::new(0),
            opener: make_machinist_opener(&db),
            gcd_priority_table: make_machinist_gcd_priority_table(&db),
            ogcd_priority_table: make_machinist_ogcd_priority_table(&db),
        }
    }
}

pub(crate) fn make_machinist_opener(db: &MachinistDatabase) -> Vec<Opener> {
    vec![
        Opener::OgcdOpener((Some(db.potion.get_id()), None)),
        Opener::GcdOpener(db.heated_split_shot.get_id()),
        Opener::OgcdOpener((Some(db.gauss_round.get_id()), Some(db.ricochet.get_id()))),
        Opener::GcdOpener(db.drill.get_id()),
        Opener::OgcdOpener((Some(db.barrel_stabilizer.get_id()), None)),
        Opener::GcdOpener(db.heated_slug_shot.get_id()),
        Opener::OgcdOpener((Some(db.ricochet.get_id()), None)),
        Opener::GcdOpener(db.heated_clean_shot.get_id()),
        Opener::OgcdOpener((Some(db.reassemble.get_id()), Some(db.gauss_round.get_id()))),
        Opener::GcdOpener(db.air_anchor.get_id()),
        Opener::OgcdOpener((Some(db.reassemble.get_id()), Some(db.wildfire.get_id()))),
        Opener::GcdOpener(db.chainsaw.get_id()),
        Opener::OgcdOpener((
            Some(db.automaton_queen.get_id()),
            Some(db.hypercharge.get_id()),
        )),
        Opener::GcdOpener(db.heat_blast.get_id()),
        Opener::OgcdOpener((Some(db.ricochet.get_id()), None)),
        Opener::GcdOpener(db.heat_blast.get_id()),
        Opener::OgcdOpener((Some(db.gauss_round.get_id()), None)),
        Opener::GcdOpener(db.heat_blast.get_id()),
        Opener::OgcdOpener((Some(db.ricochet.get_id()), None)),
        Opener::GcdOpener(db.heat_blast.get_id()),
        Opener::OgcdOpener((Some(db.gauss_round.get_id()), None)),
        Opener::GcdOpener(db.heat_blast.get_id()),
        Opener::OgcdOpener((Some(db.ricochet.get_id()), None)),
        Opener::GcdOpener(db.drill.get_id()),
    ]
}

pub(crate) fn make_machinist_gcd_priority_table(db: &MachinistDatabase) -> Vec<SkillPriorityInfo> {
    vec![
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
            skill_id: db.heat_blast.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.drill.get_id(),
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

pub(crate) fn make_machinist_ogcd_priority_table(db: &MachinistDatabase) -> Vec<SkillPriorityInfo> {
    vec![
        SkillPriorityInfo {
            skill_id: db.potion.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.wildfire.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.hypercharge.get_id(),
            prerequisite: Some(Or(
                Box::new(And(
                    Box::new(RelatedSkillCooldownLessOrEqualThan(db.drill.get_id(), 8000)),
                    Box::new(And(
                        Box::new(RelatedSkillCooldownLessOrEqualThan(
                            db.air_anchor.get_id(),
                            8000,
                        )),
                        Box::new(RelatedSkillCooldownLessOrEqualThan(
                            db.chainsaw.get_id(),
                            8000,
                        )),
                    )),
                )),
                Box::new(Or(
                    Box::new(SkillPrerequisite::HasResource(0, 70)),
                    Box::new(MillisecondsBeforeBurst(0)),
                )),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.reassemble.get_id(),
            prerequisite: Some(And(
                Box::new(Or(
                    Box::new(RelatedSkillCooldownLessOrEqualThan(db.drill.get_id(), 1000)),
                    Box::new(Or(
                        Box::new(RelatedSkillCooldownLessOrEqualThan(
                            db.air_anchor.get_id(),
                            1000,
                        )),
                        Box::new(RelatedSkillCooldownLessOrEqualThan(
                            db.chainsaw.get_id(),
                            1000,
                        )),
                    )),
                )),
                Box::new(Or(
                    Box::new(MillisecondsBeforeBurst(0)),
                    Box::new(HasSkillStacks(db.reassemble.get_id(), 2)),
                )),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.ricochet.get_id(),
            prerequisite: Some(Or(
                Box::new(MillisecondsBeforeBurst(0)),
                Box::new(RelatedSkillCooldownLessOrEqualThan(
                    db.ricochet.get_id(),
                    60000,
                )),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.gauss_round.get_id(),
            prerequisite: Some(Or(
                Box::new(MillisecondsBeforeBurst(0)),
                Box::new(RelatedSkillCooldownLessOrEqualThan(
                    db.gauss_round.get_id(),
                    60000,
                )),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.barrel_stabilizer.get_id(),
            prerequisite: Some(Not(Box::new(SkillPrerequisite::HasResource(0, 50)))),
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
    ]
}

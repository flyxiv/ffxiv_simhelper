use crate::id_entity::IdEntity;
use crate::jobs_skill_data::scholar::abilities::ScholarDatabase;
use crate::rotation::priority_table::SkillPrerequisite::{
    HasBufforDebuff, MillisecondsBeforeBurst, Not, Or, RelatedSkillCooldownLessOrEqualThan,
};
use crate::rotation::priority_table::{Opener, PriorityTable, SkillPrerequisite};
use crate::rotation::SkillPriorityInfo;
use crate::types::{PlayerIdType, SkillIdType};
use std::cell::RefCell;

#[derive(Clone)]
pub struct ScholarPriorityTable {
    turn_count: RefCell<SkillIdType>,
    opener: Vec<Opener>,

    gcd_priority_table: Vec<SkillPriorityInfo>,
    ogcd_priority_table: Vec<SkillPriorityInfo>,
}

impl PriorityTable for ScholarPriorityTable {
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

impl ScholarPriorityTable {
    pub fn new(player_id: PlayerIdType, use_pots: bool) -> Self {
        let db = ScholarDatabase::new(player_id);
        Self {
            turn_count: RefCell::new(0),
            opener: make_scholar_opener(&db, use_pots),
            gcd_priority_table: make_scholar_gcd_priority_table(&db),
            ogcd_priority_table: make_scholar_ogcd_priority_table(&db, use_pots),
        }
    }
}

pub(crate) fn make_scholar_opener(db: &ScholarDatabase, use_pots: bool) -> Vec<Opener> {
    let mut openers = if use_pots {
        vec![
            Opener::GcdOpener(db.broil_iv.get_id()),
            Opener::OgcdOpener((Some(db.potion.get_id()), None)),
            Opener::GcdOpener(db.biolysis.get_id()),
            Opener::OgcdOpener((Some(db.dissipation.get_id()), None)),
        ]
    } else {
        vec![
            Opener::GcdOpener(db.broil_iv.get_id()),
            Opener::OgcdOpener((None, None)),
            Opener::GcdOpener(db.biolysis.get_id()),
            Opener::OgcdOpener((Some(db.dissipation.get_id()), None)),
        ]
    };
    openers.extend(vec![
        Opener::GcdOpener(db.broil_iv.get_id()),
        Opener::OgcdOpener((Some(db.chain_stratagem.get_id()), None)),
        Opener::GcdOpener(db.broil_iv.get_id()),
        Opener::OgcdOpener((Some(db.energy_drain.get_id()), None)),
        Opener::GcdOpener(db.broil_iv.get_id()),
        Opener::OgcdOpener((Some(db.energy_drain.get_id()), None)),
        Opener::GcdOpener(db.broil_iv.get_id()),
        Opener::OgcdOpener((Some(db.energy_drain.get_id()), None)),
        Opener::GcdOpener(db.broil_iv.get_id()),
        Opener::OgcdOpener((Some(db.aetherflow.get_id()), None)),
        Opener::GcdOpener(db.broil_iv.get_id()),
        Opener::OgcdOpener((Some(db.baneful_impaction.get_id()), None)),
        Opener::GcdOpener(db.broil_iv.get_id()),
        Opener::OgcdOpener((Some(db.energy_drain.get_id()), None)),
        Opener::GcdOpener(db.broil_iv.get_id()),
        Opener::OgcdOpener((Some(db.energy_drain.get_id()), None)),
        Opener::GcdOpener(db.broil_iv.get_id()),
        Opener::OgcdOpener((Some(db.energy_drain.get_id()), None)),
        Opener::GcdOpener(db.biolysis.get_id()),
    ]);

    openers
}

pub(crate) fn make_scholar_gcd_priority_table(db: &ScholarDatabase) -> Vec<SkillPriorityInfo> {
    vec![
        SkillPriorityInfo {
            skill_id: db.biolysis.get_id(),
            prerequisite: Some(Or(
                Box::new(SkillPrerequisite::BufforDebuffLessThan(
                    db.biolysis_dot.get_id(),
                    1500,
                )),
                Box::new(Not(Box::new(HasBufforDebuff(db.biolysis_dot.get_id())))),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.broil_iv.get_id(),
            prerequisite: None,
        },
    ]
}

pub(crate) fn make_scholar_ogcd_priority_table(
    db: &ScholarDatabase,
    use_pots: bool,
) -> Vec<SkillPriorityInfo> {
    let mut ogcd_priorities = if use_pots {
        vec![SkillPriorityInfo {
            skill_id: db.potion.get_id(),
            prerequisite: Some(MillisecondsBeforeBurst(12000)),
        }]
    } else {
        vec![]
    };
    ogcd_priorities.extend(vec![
        SkillPriorityInfo {
            skill_id: db.dissipation.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.chain_stratagem.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.baneful_impaction.get_id(),
            prerequisite: Some(SkillPrerequisite::And(
                Box::new(Not(Box::new(RelatedSkillCooldownLessOrEqualThan(
                    db.dissipation.get_id(),
                    10000,
                )))),
                Box::new(Not(Box::new(RelatedSkillCooldownLessOrEqualThan(
                    db.aetherflow.get_id(),
                    10000,
                )))),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.energy_drain.get_id(),
            prerequisite: Some(Or(
                Box::new(MillisecondsBeforeBurst(0)),
                Box::new(Or(
                    Box::new(RelatedSkillCooldownLessOrEqualThan(
                        db.dissipation.get_id(),
                        8000,
                    )),
                    Box::new(RelatedSkillCooldownLessOrEqualThan(
                        db.aetherflow.get_id(),
                        8000,
                    )),
                )),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.aetherflow.get_id(),
            prerequisite: None,
        },
    ]);

    ogcd_priorities
}

use crate::id_entity::IdEntity;
use crate::jobs_skill_data::sage::abilities::SageDatabase;
use crate::rotation::priority_table::SkillPrerequisite::{
    And, BufforDebuffLessThan, Not, RelatedSkillCooldownLessOrEqualThan,
};
use crate::rotation::priority_table::{Opener, PriorityTable, SkillPrerequisite};
use crate::rotation::SkillPriorityInfo;
use crate::types::{PlayerIdType, SkillIdType};
use std::cell::RefCell;

#[derive(Clone)]
pub struct SagePriorityTable {
    turn_count: RefCell<SkillIdType>,
    opener: Vec<Opener>,
    gcd_priority_list: Vec<SkillPriorityInfo>,
    ogcd_priority_list: Vec<SkillPriorityInfo>,
}

impl PriorityTable for SagePriorityTable {
    fn get_opener_len(&self) -> usize {
        self.opener.len()
    }

    fn get_opener_at(&self, index: usize) -> Opener {
        self.opener[index].clone()
    }

    fn get_gcd_priority_table(&self) -> &[SkillPriorityInfo] {
        &self.gcd_priority_list
    }

    fn get_ogcd_priority_table(&self) -> &[SkillPriorityInfo] {
        &self.ogcd_priority_list
    }

    fn increment_turn(&self) {
        *self.turn_count.borrow_mut() += 1;
    }

    fn get_turn_count(&self) -> SkillIdType {
        *self.turn_count.borrow()
    }
}

impl SagePriorityTable {
    pub fn new(player_id: PlayerIdType, use_pots: bool) -> Self {
        let db = SageDatabase::new(player_id);

        Self {
            turn_count: RefCell::new(0),
            opener: make_sage_opener(&db, use_pots),
            gcd_priority_list: make_sage_gcd_priority_db(&db),
            ogcd_priority_list: make_sage_ogcd_priority_db(&db, use_pots),
        }
    }
}

pub(crate) fn make_sage_opener(db: &SageDatabase, use_pots: bool) -> Vec<Opener> {
    let mut openers = if use_pots {
        vec![
            Opener::GcdOpener(db.eukrasia.get_id()),
            Opener::OgcdOpener((Some(db.potion.get_id()), None)),
        ]
    } else {
        vec![
            Opener::GcdOpener(db.eukrasia.get_id()),
            Opener::OgcdOpener((None, None)),
        ]
    };
    openers.extend(vec![
        Opener::GcdOpener(db.toxikon.get_id()),
        Opener::OgcdOpener((None, None)),
        Opener::GcdOpener(db.eukrasia_dot_opener.get_id()),
        Opener::OgcdOpener((None, None)),
        Opener::GcdOpener(db.gcd.get_id()),
        Opener::OgcdOpener((None, None)),
        Opener::GcdOpener(db.gcd.get_id()),
        Opener::OgcdOpener((None, None)),
        Opener::GcdOpener(db.gcd.get_id()),
        Opener::OgcdOpener((None, None)),
        Opener::GcdOpener(db.phlegma.get_id()),
    ]);

    openers
}

pub(crate) fn make_sage_gcd_priority_db(db: &SageDatabase) -> Vec<SkillPriorityInfo> {
    let sage_priority_list: Vec<SkillPriorityInfo> = vec![
        SkillPriorityInfo {
            skill_id: db.phlegma.get_id(),
            prerequisite: Some(SkillPrerequisite::Or(
                Box::new(SkillPrerequisite::HasSkillStacks(702, 2)),
                Box::new(And(
                    Box::new(SkillPrerequisite::MillisecondsBeforeBurst(0)),
                    Box::new(Not(Box::new(RelatedSkillCooldownLessOrEqualThan(
                        db.psyche.get_id(),
                        50000,
                    )))),
                )),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.dot.get_id(),
            prerequisite: Some(BufforDebuffLessThan(700, 3000)),
        },
        SkillPriorityInfo {
            skill_id: db.gcd.get_id(),
            prerequisite: None,
        },
    ];

    sage_priority_list
}

pub(crate) fn make_sage_ogcd_priority_db(
    db: &SageDatabase,
    use_pots: bool,
) -> Vec<SkillPriorityInfo> {
    if use_pots {
        vec![
            SkillPriorityInfo {
                skill_id: db.potion.get_id(),
                prerequisite: None,
            },
            SkillPriorityInfo {
                skill_id: db.psyche.get_id(),
                prerequisite: None,
            },
        ]
    } else {
        vec![SkillPriorityInfo {
            skill_id: db.psyche.get_id(),
            prerequisite: None,
        }]
    }
}

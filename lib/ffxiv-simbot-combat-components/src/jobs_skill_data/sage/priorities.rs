use crate::id_entity::IdEntity;
use crate::jobs_skill_data::sage::abilities::SageDatabase;
use crate::rotation::priority_table::{Opener, PriorityTable, SkillPrerequisite};
use crate::rotation::SkillPriorityInfo;
use crate::types::{IdType, PlayerIdType};
use std::cell::RefCell;

#[derive(Clone)]
pub struct SagePriorityTable {
    turn_count: RefCell<IdType>,
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

    fn get_turn_count(&self) -> IdType {
        *self.turn_count.borrow()
    }
}

impl SagePriorityTable {
    pub fn new(player_id: PlayerIdType) -> Self {
        let db = SageDatabase::new(player_id);

        Self {
            turn_count: RefCell::new(0),
            opener: make_sage_opener(&db),
            gcd_priority_list: make_sage_gcd_priority_db(&db),
            ogcd_priority_list: make_sage_ogcd_priority_db(&db),
        }
    }
}

pub(crate) fn make_sage_opener(db: &SageDatabase) -> Vec<Opener> {
    let sage_opener: Vec<Opener> = vec![
        Opener::GcdOpener(db.gcd.get_id()),
        Opener::OgcdOpener((Some(db.potion.get_id()), None)),
        Opener::GcdOpener(db.dot.get_id()),
        Opener::OgcdOpener((None, None)),
        Opener::GcdOpener(db.gcd.get_id()),
        Opener::OgcdOpener((None, None)),
        Opener::GcdOpener(db.gcd.get_id()),
    ];

    sage_opener
}

pub(crate) fn make_sage_gcd_priority_db(db: &SageDatabase) -> Vec<SkillPriorityInfo> {
    let sage_priority_list: Vec<SkillPriorityInfo> = vec![
        SkillPriorityInfo {
            skill_id: db.phlegma.get_id(),
            prerequisite: Some(SkillPrerequisite::Or(
                Box::new(SkillPrerequisite::HasSkillStacks(702, 2)),
                Box::new(SkillPrerequisite::MillisecondsBeforeBurst(0)),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.dot.get_id(),
            prerequisite: Some(SkillPrerequisite::BufforDebuffLessThan(700, 3000)),
        },
        SkillPriorityInfo {
            skill_id: db.gcd.get_id(),
            prerequisite: None,
        },
    ];

    sage_priority_list
}

pub(crate) fn make_sage_ogcd_priority_db(db: &SageDatabase) -> Vec<SkillPriorityInfo> {
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
}

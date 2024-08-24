use crate::id_entity::IdEntity;
use crate::jobs_skill_data::dancer::abilities::DancerDatabase;
use crate::rotation::priority_table::SkillPrerequisite::{
    HasBufforDebuff, MillisecondsBeforeBurst,
};
use crate::rotation::priority_table::{Opener, PriorityTable, SkillPrerequisite};
use crate::rotation::SkillPriorityInfo;
use crate::types::{IdType, PlayerIdType};
use std::cell::RefCell;

#[derive(Clone)]
pub(crate) struct DancerPriorityTable {
    turn_count: RefCell<IdType>,
    opener: Vec<Opener>,

    gcd_priority_table: Vec<SkillPriorityInfo>,
    ogcd_priority_table: Vec<SkillPriorityInfo>,
}

impl PriorityTable for DancerPriorityTable {
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

impl DancerPriorityTable {
    pub fn new(player_id: PlayerIdType, partner_player_id: PlayerIdType) -> Self {
        let db = DancerDatabase::new(player_id, partner_player_id);
        Self {
            turn_count: RefCell::new(0),
            opener: make_dancer_opener(&db),
            gcd_priority_table: make_dancer_gcd_priority_table(&db),
            ogcd_priority_table: make_dancer_ogcd_priority_table(&db),
        }
    }
}

pub(crate) fn make_dancer_opener(db: &DancerDatabase) -> Vec<Opener> {
    vec![
        Opener::GcdOpener(db.standard_opener.get_id()),
        Opener::OgcdOpener((Some(db.potion.get_id()), None)),
        Opener::GcdOpener(db.technical_step.get_id()),
        Opener::OgcdOpener((Some(db.devilment.get_id()), None)),
        Opener::GcdOpener(db.last_dance.get_id()),
        Opener::OgcdOpener((Some(db.flourish.get_id()), None)),
        Opener::GcdOpener(db.starfall_dance.get_id()),
    ]
}

pub(crate) fn make_dancer_gcd_priority_table(db: &DancerDatabase) -> Vec<SkillPriorityInfo> {
    vec![
        SkillPriorityInfo {
            skill_id: db.technical_step.get_id(),
            prerequisite: Some(SkillPrerequisite::MillisecondsBeforeBurst(5500)),
        },
        SkillPriorityInfo {
            skill_id: db.dance_of_the_dawn.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.finishing_move.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.standard_step.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.saber_dance.get_id(),
            prerequisite: Some(HasBufforDebuff(db.technical_step_buff.get_id())),
        },
        SkillPriorityInfo {
            skill_id: db.starfall_dance.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.tillana.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.last_dance.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.saber_dance.get_id(),
            prerequisite: Some(SkillPrerequisite::HasResource(0, 80)),
        },
        SkillPriorityInfo {
            skill_id: db.fountainfall_flourish.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.reverse_cascade_flourish.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.fountainfall.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.reverse_cascade.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.fountain.get_id(),
            prerequisite: Some(SkillPrerequisite::Combo(Some(1))),
        },
        SkillPriorityInfo {
            skill_id: db.cascade.get_id(),
            prerequisite: None,
        },
    ]
}

pub(crate) fn make_dancer_ogcd_priority_table(db: &DancerDatabase) -> Vec<SkillPriorityInfo> {
    vec![
        SkillPriorityInfo {
            skill_id: db.potion.get_id(),
            prerequisite: Some(MillisecondsBeforeBurst(9000)),
        },
        SkillPriorityInfo {
            skill_id: db.devilment.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.fan_dance3.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.flourish.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.fan_dance1.get_id(),
            prerequisite: Some(SkillPrerequisite::Or(
                Box::new(SkillPrerequisite::HasResource(1, 4)),
                Box::new(SkillPrerequisite::MillisecondsBeforeBurst(0)),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.fan_dance4.get_id(),
            prerequisite: None,
        },
    ]
}

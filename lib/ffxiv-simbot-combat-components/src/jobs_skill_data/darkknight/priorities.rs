use crate::rotation::priority_table::{Opener, PriorityTable};
use crate::rotation::SkillPriorityInfo;
use crate::types::IdType;
use std::cell::RefCell;

use crate::id_entity::IdEntity;
use crate::jobs_skill_data::darkknight::abilities::DarkknightDatabase;
use crate::rotation::priority_table::Opener::{GcdOpener, OgcdOpener};
use crate::rotation::priority_table::SkillPrerequisite::{
    BufforDebuffLessThan, Combo, HasResource, MillisecondsBeforeBurst, Or,
};
use crate::types::TurnCount;

#[derive(Clone)]
pub(crate) struct DarkknightPriorityTable {
    turn_count: RefCell<TurnCount>,
    opener: Vec<Opener>,

    gcd_priority_table: Vec<SkillPriorityInfo>,
    ogcd_priority_table: Vec<SkillPriorityInfo>,
}

impl PriorityTable for DarkknightPriorityTable {
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

impl DarkknightPriorityTable {
    pub fn new(player_id: IdType) -> Self {
        let db = DarkknightDatabase::new(player_id);
        Self {
            turn_count: RefCell::new(0),
            opener: make_darkknight_opener(&db),
            gcd_priority_table: make_darkknight_gcd_priority_table(&db),
            ogcd_priority_table: make_darkknight_ogcd_priority_table(&db),
        }
    }
}

pub(crate) fn make_darkknight_opener(db: &DarkknightDatabase) -> Vec<Opener> {
    vec![
        GcdOpener(db.hard_slash.get_id()),
        OgcdOpener((Some(db.edge_of_shadow.get_id()), Some(db.potion.get_id()))),
        GcdOpener(db.syphon_strike.get_id()),
        OgcdOpener((Some(db.delirium.get_id()), Some(db.living_shadow.get_id()))),
        GcdOpener(db.scarlet_delirium.get_id()),
        OgcdOpener((
            Some(db.edge_of_shadow.get_id()),
            Some(db.carve_and_spit.get_id()),
        )),
        GcdOpener(db.comeuppance.get_id()),
        OgcdOpener((
            Some(db.shadowbringer.get_id()),
            Some(db.edge_of_shadow.get_id()),
        )),
        GcdOpener(db.torcleaver.get_id()),
        OgcdOpener((
            Some(db.shadowbringer.get_id()),
            Some(db.salt_and_darkness.get_id()),
        )),
        GcdOpener(db.souleater.get_id()),
        OgcdOpener((Some(db.edge_of_shadow.get_id()), None)),
        GcdOpener(db.disesteem.get_id()),
    ]
}

pub(crate) fn make_darkknight_gcd_priority_table(
    db: &DarkknightDatabase,
) -> Vec<SkillPriorityInfo> {
    vec![
        SkillPriorityInfo {
            skill_id: db.torcleaver.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.comeuppance.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.scarlet_delirium.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.bloodspiller.get_id(),
            prerequisite: Some(Or(
                Box::new(MillisecondsBeforeBurst(0)),
                Box::new(HasResource(1, 80)),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.souleater.get_id(),
            prerequisite: Some(Combo(Some(3))),
        },
        SkillPriorityInfo {
            skill_id: db.syphon_strike.get_id(),
            prerequisite: Some(Combo(Some(2))),
        },
        SkillPriorityInfo {
            skill_id: db.hard_slash.get_id(),
            prerequisite: None,
        },
    ]
}

pub(crate) fn make_darkknight_ogcd_priority_table(
    db: &DarkknightDatabase,
) -> Vec<SkillPriorityInfo> {
    vec![
        SkillPriorityInfo {
            skill_id: db.potion.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.living_shadow.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.edge_of_shadow.get_id(),
            prerequisite: Some(HasResource(0, 9400)),
        },
        SkillPriorityInfo {
            skill_id: db.delirium.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.shadowbringer.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.salted_earth.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.disesteem.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.salt_and_darkness.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.edge_of_shadow.get_id(),
            prerequisite: Some(Or(
                Box::new(MillisecondsBeforeBurst(0)),
                Box::new(BufforDebuffLessThan(db.darkside.get_id(), 3000)),
            )),
        },
    ]
}

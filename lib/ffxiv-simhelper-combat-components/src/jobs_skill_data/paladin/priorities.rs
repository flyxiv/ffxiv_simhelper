use crate::rotation::priority_table::{Opener, PriorityTable};
use crate::rotation::SkillPriorityInfo;
use crate::types::{PlayerIdType, SkillIdType};
use std::cell::RefCell;

use crate::id_entity::IdEntity;
use crate::jobs_skill_data::paladin::abilities::PaladinDatabase;
use crate::rotation::priority_table::Opener::{GcdOpener, OgcdOpener};
use crate::rotation::priority_table::SkillPrerequisite::{
    Combo, HasBufforDebuff, MillisecondsBeforeBurst,
};

#[derive(Clone)]
pub struct PaladinPriorityTable {
    turn_count: RefCell<SkillIdType>,
    opener: Vec<Opener>,

    gcd_priority_table: Vec<SkillPriorityInfo>,
    ogcd_priority_table: Vec<SkillPriorityInfo>,
}

impl PriorityTable for PaladinPriorityTable {
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

impl PaladinPriorityTable {
    pub fn new(player_id: PlayerIdType, use_pots: bool) -> Self {
        let db = PaladinDatabase::new(player_id);
        Self {
            turn_count: RefCell::new(0),
            opener: make_paladin_opener(&db, use_pots),
            gcd_priority_table: make_paladin_gcd_priority_table(&db),
            ogcd_priority_table: make_paladin_ogcd_priority_table(&db, use_pots),
        }
    }
}

pub(crate) fn make_paladin_opener(db: &PaladinDatabase, use_pots: bool) -> Vec<Opener> {
    let mut openers = if use_pots {
        vec![
            GcdOpener(db.weak_holy_spirit.get_id()),
            OgcdOpener((None, None)),
            GcdOpener(db.fast_blade.get_id()),
            OgcdOpener((None, None)),
            GcdOpener(db.riot_blade.get_id()),
            OgcdOpener((Some(db.potion.get_id()), None)),
        ]
    } else {
        vec![
            GcdOpener(db.weak_holy_spirit.get_id()),
            OgcdOpener((None, None)),
            GcdOpener(db.fast_blade.get_id()),
            OgcdOpener((None, None)),
            GcdOpener(db.riot_blade.get_id()),
            OgcdOpener((None, None)),
        ]
    };

    openers.extend(vec![
        GcdOpener(db.royal_authority.get_id()),
        OgcdOpener((
            Some(db.fight_or_flight.get_id()),
            Some(db.imperator.get_id()),
        )),
        GcdOpener(db.confiteor.get_id()),
        OgcdOpener((
            Some(db.circle_of_scorn.get_id()),
            Some(db.explacion.get_id()),
        )),
        GcdOpener(db.blade_of_faith.get_id()),
        OgcdOpener((Some(db.intervene.get_id()), None)),
        GcdOpener(db.blade_of_truth.get_id()),
        OgcdOpener((Some(db.intervene.get_id()), None)),
        GcdOpener(db.blade_of_valor.get_id()),
        OgcdOpener((Some(db.blade_of_honor.get_id()), None)),
        GcdOpener(db.goring_blade.get_id()),
        OgcdOpener((None, None)),
        GcdOpener(db.atonement.get_id()),
        OgcdOpener((None, None)),
    ]);

    openers
}

pub(crate) fn make_paladin_gcd_priority_table(db: &PaladinDatabase) -> Vec<SkillPriorityInfo> {
    vec![
        SkillPriorityInfo {
            skill_id: db.confiteor.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.blade_of_faith.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.blade_of_truth.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.blade_of_valor.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.goring_blade.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.holy_spirit.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.sepulchre.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.supplication.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.atonement.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.royal_authority.get_id(),
            prerequisite: Some(Combo(Some(3))),
        },
        SkillPriorityInfo {
            skill_id: db.riot_blade.get_id(),
            prerequisite: Some(Combo(Some(2))),
        },
        SkillPriorityInfo {
            skill_id: db.fast_blade.get_id(),
            prerequisite: None,
        },
    ]
}

pub(crate) fn make_paladin_ogcd_priority_table(
    db: &PaladinDatabase,
    use_pots: bool,
) -> Vec<SkillPriorityInfo> {
    let mut skill_ogcd_priorities = if use_pots {
        vec![SkillPriorityInfo {
            skill_id: db.potion.get_id(),
            prerequisite: Some(MillisecondsBeforeBurst(9000)),
        }]
    } else {
        vec![]
    };

    skill_ogcd_priorities.extend(vec![
        SkillPriorityInfo {
            skill_id: db.imperator.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.fight_or_flight.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.circle_of_scorn.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.explacion.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.intervene.get_id(),
            prerequisite: Some(HasBufforDebuff(db.fight_or_flight_buff.get_id())),
        },
        SkillPriorityInfo {
            skill_id: db.blade_of_honor.get_id(),
            prerequisite: None,
        },
    ]);

    skill_ogcd_priorities
}

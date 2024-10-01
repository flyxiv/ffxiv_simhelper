use crate::rotation::priority_table::{Opener, PriorityTable, SkillPrerequisite};
use crate::rotation::SkillPriorityInfo;
use std::cell::RefCell;

use crate::id_entity::IdEntity;
use crate::jobs_skill_data::warrior::abilities::WarriorDatabase;
use crate::rotation::priority_table::Opener::{GcdOpener, OgcdOpener};
use crate::rotation::priority_table::SkillPrerequisite::{
    And, BufforDebuffLessThan, Combo, HasBufforDebuff, HasResource, MillisecondsBeforeBurst, Not,
    Or,
};
use crate::types::{PlayerIdType, SkillIdType};

#[derive(Clone)]
pub struct WarriorPriorityTable {
    turn_count: RefCell<SkillIdType>,
    opener: Vec<Opener>,

    gcd_priority_table: Vec<SkillPriorityInfo>,
    ogcd_priority_table: Vec<SkillPriorityInfo>,
}

impl PriorityTable for WarriorPriorityTable {
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

impl WarriorPriorityTable {
    pub fn new(player_id: PlayerIdType, use_pots: bool) -> Self {
        let db = WarriorDatabase::new(player_id);
        Self {
            turn_count: RefCell::new(0),
            opener: make_warrior_opener(&db, use_pots),
            gcd_priority_table: make_warrior_gcd_priority_table(&db),
            ogcd_priority_table: make_warrior_ogcd_priority_table(&db, use_pots),
        }
    }
}

pub(crate) fn make_warrior_opener(db: &WarriorDatabase, use_pots: bool) -> Vec<Opener> {
    let mut openers = if use_pots {
        vec![
            GcdOpener(db.heavy_swing.get_id()),
            OgcdOpener((Some(db.infuriate.get_id()), None)),
            GcdOpener(db.maim.get_id()),
            OgcdOpener((None, None)),
            GcdOpener(db.storms_eye.get_id()),
            OgcdOpener((Some(db.inner_release.get_id()), Some(db.potion.get_id()))),
        ]
    } else {
        vec![
            GcdOpener(db.heavy_swing.get_id()),
            OgcdOpener((Some(db.infuriate.get_id()), None)),
            GcdOpener(db.maim.get_id()),
            OgcdOpener((None, None)),
            GcdOpener(db.storms_eye.get_id()),
            OgcdOpener((Some(db.inner_release.get_id()), None)),
        ]
    };

    openers.extend(vec![
        GcdOpener(db.inner_chaos.get_id()),
        OgcdOpener((Some(db.upheaval.get_id()), Some(db.onslaught.get_id()))),
        GcdOpener(db.primal_rend.get_id()),
        OgcdOpener((Some(db.onslaught.get_id()), None)),
        GcdOpener(db.primal_ruination.get_id()),
        OgcdOpener((Some(db.onslaught.get_id()), None)),
        GcdOpener(db.fell_cleave_inner.get_id()),
        OgcdOpener((None, None)),
        GcdOpener(db.fell_cleave_inner.get_id()),
        OgcdOpener((None, None)),
        GcdOpener(db.fell_cleave_inner.get_id()),
        OgcdOpener((Some(db.primal_wrath.get_id()), Some(db.infuriate.get_id()))),
        GcdOpener(db.inner_chaos.get_id()),
        OgcdOpener((None, None)),
        GcdOpener(db.heavy_swing.get_id()),
        OgcdOpener((None, None)),
        GcdOpener(db.maim.get_id()),
        OgcdOpener((None, None)),
        GcdOpener(db.storms_path.get_id()),
        OgcdOpener((None, None)),
        GcdOpener(db.fell_cleave.get_id()),
        OgcdOpener((Some(db.infuriate.get_id()), None)),
        GcdOpener(db.inner_chaos.get_id()),
    ]);

    openers
}

pub(crate) fn make_warrior_gcd_priority_table(db: &WarriorDatabase) -> Vec<SkillPriorityInfo> {
    vec![
        SkillPriorityInfo {
            skill_id: db.inner_chaos.get_id(),
            prerequisite: Some(HasBufforDebuff(db.nascent_chaos.get_id())),
        },
        SkillPriorityInfo {
            skill_id: db.primal_rend.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.primal_ruination.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.fell_cleave_inner.get_id(),
            prerequisite: Some(HasBufforDebuff(db.inner_release_stack.get_id())),
        },
        SkillPriorityInfo {
            skill_id: db.fell_cleave.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.storms_eye.get_id(),
            prerequisite: Some(And(
                Box::new(Combo(Some(3))),
                Box::new(BufforDebuffLessThan(db.surging_tempest.get_id(), 30000)),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.storms_path.get_id(),
            prerequisite: Some(Combo(Some(3))),
        },
        SkillPriorityInfo {
            skill_id: db.maim.get_id(),
            prerequisite: Some(Combo(Some(2))),
        },
        SkillPriorityInfo {
            skill_id: db.heavy_swing.get_id(),
            prerequisite: None,
        },
    ]
}

pub(crate) fn make_warrior_ogcd_priority_table(
    db: &WarriorDatabase,
    use_pots: bool,
) -> Vec<SkillPriorityInfo> {
    let mut ogcd_priorities = if use_pots {
        vec![SkillPriorityInfo {
            skill_id: db.potion.get_id(),
            prerequisite: Some(MillisecondsBeforeBurst(9000)),
        }]
    } else {
        vec![]
    };

    ogcd_priorities.extend(vec![
        SkillPriorityInfo {
            skill_id: db.inner_release.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.infuriate.get_id(),
            prerequisite: Some(And(
                Box::new(Not(Box::new(HasResource(0, 50)))),
                Box::new(Or(
                    Box::new(MillisecondsBeforeBurst(0)),
                    Box::new(SkillPrerequisite::HasSkillStacks(db.infuriate.get_id(), 2)),
                )),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.upheaval.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.primal_wrath.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.onslaught.get_id(),
            prerequisite: Some(Or(
                Box::new(MillisecondsBeforeBurst(0)),
                Box::new(Not(Box::new(MillisecondsBeforeBurst(85000)))),
            )),
        },
    ]);

    ogcd_priorities
}

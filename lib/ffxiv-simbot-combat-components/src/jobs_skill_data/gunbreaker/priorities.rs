use crate::rotation::priority_table::{Opener, PriorityTable, SkillPrerequisite};
use crate::rotation::SkillPriorityInfo;
use crate::{IdType, TurnCount};
use std::cell::RefCell;

use crate::id_entity::IdEntity;
use crate::jobs_skill_data::gunbreaker::abilities::GunbreakerDatabase;
use crate::jobs_skill_data::paladin::abilities::PaladinDatabase;
use crate::rotation::priority_table::Opener::{GcdOpener, OgcdOpener};
use crate::rotation::priority_table::SkillPrerequisite::{
    And, Combo, HasBufforDebuff, HasResource, Not, Or, RelatedSkillCooldownLessOrEqualThan,
};

#[derive(Clone)]
pub(crate) struct GunbreakerPriorityTable {
    turn_count: RefCell<TurnCount>,
    opener: Vec<Opener>,

    gcd_priority_table: Vec<SkillPriorityInfo>,
    ogcd_priority_table: Vec<SkillPriorityInfo>,
}

impl PriorityTable for GunbreakerPriorityTable {
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

impl GunbreakerPriorityTable {
    pub fn new(player_id: IdType) -> Self {
        let db = GunbreakerDatabase::new(player_id);
        Self {
            turn_count: RefCell::new(0),
            opener: make_gunbreaker_opener(&db),
            gcd_priority_table: make_gunbreaker_gcd_priority_table(&db),
            ogcd_priority_table: make_gunbreaker_ogcd_priority_table(&db),
        }
    }
}

pub(crate) fn make_gunbreaker_opener(db: &GunbreakerDatabase) -> Vec<Opener> {
    vec![
        GcdOpener(db.keen_edge.get_id()),
        OgcdOpener((Some(db.potion.get_id()), None)),
        GcdOpener(db.brutal_shell.get_id()),
        OgcdOpener((Some(db.no_mercy.get_id()), None)),
        GcdOpener(db.solid_barrel.get_id()),
        OgcdOpener((None, None)),
        GcdOpener(db.gnashing_fang.get_id()),
        OgcdOpener((Some(db.bloodfest.get_id()), Some(db.jugular_rip.get_id()))),
        GcdOpener(db.sonic_break.get_id()),
        OgcdOpener((
            Some(db.blasting_zone.get_id()),
            Some(db.rough_divide.get_id()),
        )),
        GcdOpener(db.double_down.get_id()),
        OgcdOpener((Some(db.bow_shock.get_id()), Some(db.rough_divide.get_id()))),
        GcdOpener(db.savage_claw.get_id()),
        OgcdOpener((Some(db.abdomen_tear.get_id()), None)),
        GcdOpener(db.wicked_talon.get_id()),
        OgcdOpener((Some(db.eye_gouge.get_id()), None)),
    ]
}

pub(crate) fn make_gunbreaker_gcd_priority_table(
    db: &GunbreakerDatabase,
) -> Vec<SkillPriorityInfo> {
    vec![
        SkillPriorityInfo {
            skill_id: db.gnashing_fang.get_id(),
            prerequisite: Some(Or(
                Box::new(HasBufforDebuff(db.no_mercy.get_id())),
                Box::new(Not(Box::new(RelatedSkillCooldownLessOrEqualThan(
                    db.no_mercy.get_id(),
                    29500,
                )))),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.burst_strike.get_id(),
            prerequisite: Some(Or(
                Box::new(RelatedSkillCooldownLessOrEqualThan(
                    db.bloodfest.get_id(),
                    8000,
                )),
                Box::new(Or(Box::new(HasResource(0, 3)), Box::new(Combo(Some(3))))),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.double_down.get_id(),
            prerequisite: Some(HasBufforDebuff(db.no_mercy.get_id())),
        },
        SkillPriorityInfo {
            skill_id: db.sonic_break.get_id(),
            prerequisite: Some(HasBufforDebuff(db.no_mercy.get_id())),
        },
        SkillPriorityInfo {
            skill_id: db.savage_claw.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.wicked_talon.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.burst_strike.get_id(),
            prerequisite: Some(HasBufforDebuff(db.no_mercy.get_id())),
        },
        SkillPriorityInfo {
            skill_id: db.solid_barrel.get_id(),
            prerequisite: Some(Combo(Some(3))),
        },
        SkillPriorityInfo {
            skill_id: db.brutal_shell.get_id(),
            prerequisite: Some(Combo(Some(2))),
        },
        SkillPriorityInfo {
            skill_id: db.keen_edge.get_id(),
            prerequisite: None,
        },
    ]
}

pub(crate) fn make_gunbreaker_ogcd_priority_table(
    db: &GunbreakerDatabase,
) -> Vec<SkillPriorityInfo> {
    vec![
        SkillPriorityInfo {
            skill_id: db.potion.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.no_mercy.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.jugular_rip.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.abdomen_tear.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.eye_gouge.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.hypervelocity.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.bloodfest.get_id(),
            prerequisite: Some(Not(Box::new(HasResource(0, 1)))),
        },
        SkillPriorityInfo {
            skill_id: db.blasting_zone.get_id(),
            prerequisite: Some(Or(
                Box::new(HasBufforDebuff(db.no_mercy.get_id())),
                Box::new(Not(Box::new(RelatedSkillCooldownLessOrEqualThan(
                    db.no_mercy.get_id(),
                    29500,
                )))),
            )),
        },
    ]
}

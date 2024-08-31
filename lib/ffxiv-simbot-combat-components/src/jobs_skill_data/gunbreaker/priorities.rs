use crate::rotation::priority_table::{Opener, PriorityTable};
use crate::rotation::SkillPriorityInfo;
use crate::types::{IdType, PlayerIdType};
use std::cell::RefCell;

use crate::id_entity::IdEntity;
use crate::jobs_skill_data::gunbreaker::abilities::GunbreakerDatabase;
use crate::rotation::priority_table::Opener::{GcdOpener, OgcdOpener};
use crate::rotation::priority_table::SkillPrerequisite::{
    And, Combo, HasBufforDebuff, HasResource, MillisecondsBeforeBurst, Not, Or,
    RelatedSkillCooldownLessOrEqualThan,
};

#[derive(Clone)]
pub struct GunbreakerPriorityTable {
    turn_count: RefCell<IdType>,
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

    fn get_gcd_priority_table(&self) -> &[SkillPriorityInfo] {
        &self.gcd_priority_table
    }

    fn get_ogcd_priority_table(&self) -> &[SkillPriorityInfo] {
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
    pub fn new(player_id: PlayerIdType) -> Self {
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
        OgcdOpener((Some(db.no_mercy.get_id()), Some(db.bloodfest.get_id()))),
        GcdOpener(db.gnashing_fang.get_id()),
        OgcdOpener((Some(db.jugular_rip.get_id()), None)),
        GcdOpener(db.sonic_break.get_id()),
        OgcdOpener((Some(db.blasting_zone.get_id()), Some(db.bow_shock.get_id()))),
        GcdOpener(db.double_down.get_id()),
        OgcdOpener((None, None)),
        GcdOpener(db.savage_claw.get_id()),
        OgcdOpener((Some(db.abdomen_tear.get_id()), None)),
        GcdOpener(db.wicked_talon.get_id()),
        OgcdOpener((Some(db.eye_gouge.get_id()), None)),
        GcdOpener(db.reign_of_beasts.get_id()),
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
                    22500,
                )))),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.double_down.get_id(),
            prerequisite: Some(HasBufforDebuff(db.no_mercy_buff.get_id())),
        },
        SkillPriorityInfo {
            skill_id: db.burst_strike.get_id(),
            prerequisite: Some(Or(
                Box::new(RelatedSkillCooldownLessOrEqualThan(
                    db.bloodfest.get_id(),
                    8000,
                )),
                Box::new(And(Box::new(HasResource(0, 3)), Box::new(Combo(Some(3))))),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.sonic_break.get_id(),
            prerequisite: Some(HasBufforDebuff(db.no_mercy_buff.get_id())),
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
            skill_id: db.lion_heart.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.noble_blood.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.reign_of_beasts.get_id(),
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
            prerequisite: Some(MillisecondsBeforeBurst(9000)),
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
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.bow_shock.get_id(),
            prerequisite: None,
        },
    ]
}

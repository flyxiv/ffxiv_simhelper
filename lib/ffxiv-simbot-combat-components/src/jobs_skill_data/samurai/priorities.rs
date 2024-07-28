use crate::rotation::priority_table::{Opener, PriorityTable};
use crate::rotation::SkillPriorityInfo;
use crate::{IdType, TurnCount};
use std::cell::RefCell;

use crate::id_entity::IdEntity;
use crate::jobs_skill_data::samurai::abilities::SamuraiDatabase;
use crate::rotation::priority_table::Opener::{GcdOpener, OgcdOpener};
use crate::rotation::priority_table::SkillPrerequisite::{
    And, BufforDebuffLessThan, Combo, HasBufforDebuff, HasResource, MillisecondsBeforeBurst, Not,
    Or, RelatedSkillCooldownLessOrEqualThan,
};

#[derive(Clone)]
pub(crate) struct SamuraiPriorityTable {
    turn_count: RefCell<TurnCount>,
    opener: Vec<Opener>,

    gcd_priority_table: Vec<SkillPriorityInfo>,
    ogcd_priority_table: Vec<SkillPriorityInfo>,
}

impl PriorityTable for SamuraiPriorityTable {
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

impl SamuraiPriorityTable {
    pub fn new(player_id: IdType) -> Self {
        let db = SamuraiDatabase::new(player_id);
        Self {
            turn_count: RefCell::new(0),
            opener: make_samurai_opener(&db),
            gcd_priority_table: make_samurai_gcd_priority_table(&db),
            ogcd_priority_table: make_samurai_ogcd_priority_table(&db),
        }
    }
}

pub(crate) fn make_samurai_opener(db: &SamuraiDatabase) -> Vec<Opener> {
    vec![
        OgcdOpener((Some(db.meikyo_shisui.get_id()), None)),
        GcdOpener(db.gekko_meiko.get_id()),
        OgcdOpener((Some(db.potion.get_id()), None)),
        GcdOpener(db.kasha_meiko.get_id()),
        OgcdOpener((None, None)),
        GcdOpener(db.yukikaze.get_id()),
        OgcdOpener((None, None)),
        GcdOpener(db.tendo_setsugekka.get_id()),
        OgcdOpener((Some(db.ikishoten.get_id()), None)),
        GcdOpener(db.kaeshi_tendo_setsugekka.get_id()),
        OgcdOpener((
            Some(db.hissatsu_senei.get_id()),
            Some(db.meikyo_shisui.get_id()),
        )),
        GcdOpener(db.gekko_meiko.get_id()),
        OgcdOpener((Some(db.zanshin.get_id()), None)),
        GcdOpener(db.higanbana.get_id()),
        OgcdOpener((None, None)),
        GcdOpener(db.ogi_namikiri.get_id()),
        OgcdOpener((Some(db.shoha.get_id()), None)),
        GcdOpener(db.kasha_meiko.get_id()),
        OgcdOpener((None, None)),
        GcdOpener(db.gekko_meiko.get_id()),
        OgcdOpener((Some(db.hissatsu_shinten.get_id()), None)),
        GcdOpener(db.gyofu.get_id()),
        OgcdOpener((None, None)),
        GcdOpener(db.yukikaze.get_id()),
        OgcdOpener((None, None)),
        GcdOpener(db.tendo_setsugekka.get_id()),
        OgcdOpener((None, None)),
        GcdOpener(db.kaeshi_tendo_setsugekka.get_id()),
    ]
}

pub(crate) fn make_samurai_gcd_priority_table(db: &SamuraiDatabase) -> Vec<SkillPriorityInfo> {
    vec![
        SkillPriorityInfo {
            skill_id: db.kaeshi_tendo_setsugekka.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.kaeshi_namikiri.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.midare_setsugekka.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.higanbana.get_id(),
            prerequisite: Some(Or(
                Box::new(RelatedSkillCooldownLessOrEqualThan(
                    db.higanbana_dot.get_id(),
                    3000,
                )),
                Box::new(Not(Box::new(HasBufforDebuff(db.higanbana_dot.get_id())))),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.higanbana_two.get_id(),
            prerequisite: Some(Or(
                Box::new(RelatedSkillCooldownLessOrEqualThan(
                    db.higanbana_dot.get_id(),
                    3000,
                )),
                Box::new(Not(Box::new(HasBufforDebuff(db.higanbana_dot.get_id())))),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.higanbana_three.get_id(),
            prerequisite: Some(Or(
                Box::new(RelatedSkillCooldownLessOrEqualThan(
                    db.higanbana_dot.get_id(),
                    3000,
                )),
                Box::new(Not(Box::new(HasBufforDebuff(db.higanbana_dot.get_id())))),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.ogi_namikiri.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.gekko_meiko.get_id(),
            prerequisite: Some(Not(Box::new(HasResource(2, 1)))),
        },
        SkillPriorityInfo {
            skill_id: db.kasha_meiko.get_id(),
            prerequisite: Some(Not(Box::new(HasResource(3, 1)))),
        },
        SkillPriorityInfo {
            skill_id: db.yukikaze.get_id(),
            prerequisite: Some(And(
                Box::new(Not(Box::new(HasResource(4, 1)))),
                Box::new(Combo(Some(2))),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.gekko.get_id(),
            prerequisite: Some(Combo(Some(3))),
        },
        SkillPriorityInfo {
            skill_id: db.kasha.get_id(),
            prerequisite: Some(Combo(Some(4))),
        },
        SkillPriorityInfo {
            skill_id: db.jinpu.get_id(),
            prerequisite: Some(Not(Box::new(HasResource(2, 1)))),
        },
        SkillPriorityInfo {
            skill_id: db.shifu.get_id(),
            prerequisite: Some(Not(Box::new(HasResource(3, 1)))),
        },
        SkillPriorityInfo {
            skill_id: db.gyofu.get_id(),
            prerequisite: None,
        },
    ]
}

pub(crate) fn make_samurai_ogcd_priority_table(db: &SamuraiDatabase) -> Vec<SkillPriorityInfo> {
    vec![
        SkillPriorityInfo {
            skill_id: db.potion.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.ikishoten.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.hissatsu_senei.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.meikyo_shisui.get_id(),
            prerequisite: Some(And(
                Box::new(BufforDebuffLessThan(db.higanbana.get_id(), 7000)),
                Box::new(Combo(Some(0))),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.shoha.get_id(),
            prerequisite: Some(Or(
                Box::new(MillisecondsBeforeBurst(0)),
                Box::new(HasResource(1, 3)),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.hissatsu_shinten.get_id(),
            prerequisite: Some(Or(
                Box::new(MillisecondsBeforeBurst(0)),
                Box::new(HasResource(0, 50)),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.hagakure.get_id(),
            prerequisite: Some(Not(Box::new(HasResource(5, 1)))),
        },
    ]
}

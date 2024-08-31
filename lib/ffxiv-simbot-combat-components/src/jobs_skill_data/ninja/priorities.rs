use crate::id_entity::IdEntity;
use crate::jobs_skill_data::ninja::abilities::NinjaDatabase;
use crate::rotation::priority_table::SkillPrerequisite::{
    And, HasBufforDebuff, HasResource, Not, Or, RelatedSkillCooldownLessOrEqualThan,
};
use crate::rotation::priority_table::{Opener, PriorityTable, SkillPrerequisite};
use crate::rotation::SkillPriorityInfo;
use crate::types::{IdType, PlayerIdType};
use std::cell::RefCell;

#[derive(Clone)]
pub struct NinjaPriorityTable {
    turn_count: RefCell<IdType>,
    opener: Vec<Opener>,

    gcd_priority_table: Vec<SkillPriorityInfo>,
    ogcd_priority_table: Vec<SkillPriorityInfo>,
}

impl PriorityTable for NinjaPriorityTable {
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

impl NinjaPriorityTable {
    pub fn new(player_id: PlayerIdType) -> Self {
        let db = NinjaDatabase::new(player_id);
        Self {
            turn_count: RefCell::new(0),
            opener: make_ninja_opener(&db),
            gcd_priority_table: make_ninja_gcd_priority_table(&db),
            ogcd_priority_table: make_ninja_ogcd_priority_table(&db),
        }
    }
}

pub(crate) fn make_ninja_opener(db: &NinjaDatabase) -> Vec<Opener> {
    vec![
        Opener::GcdOpener(db.suiton.get_id()),
        Opener::OgcdOpener((Some(db.kassatsu.get_id()), None)),
        Opener::GcdOpener(db.spinning_edge.get_id()),
        Opener::OgcdOpener((Some(db.potion.get_id()), None)),
        Opener::GcdOpener(db.gust_slash.get_id()),
        Opener::OgcdOpener((Some(db.dokumori.get_id()), Some(db.bunshin.get_id()))),
        Opener::GcdOpener(db.phantom_kamaitachi.get_id()),
        Opener::OgcdOpener((Some(db.kunais_bane.get_id()), Some(db.dream.get_id()))),
        Opener::GcdOpener(db.hyosho.get_id()),
        Opener::OgcdOpener((Some(db.tenchijin.get_id()), None)),
        Opener::GcdOpener(db.fuma_tenchijin.get_id()),
        Opener::OgcdOpener((None, None)),
        Opener::GcdOpener(db.raiton_tenchijin.get_id()),
        Opener::OgcdOpener((None, None)),
        Opener::GcdOpener(db.suiton_tenchijin.get_id()),
        Opener::OgcdOpener((Some(db.meisui.get_id()), None)),
        Opener::GcdOpener(db.raiton.get_id()),
        Opener::OgcdOpener((Some(db.zesho_meppo_meisui.get_id()), None)),
        Opener::GcdOpener(db.raiju.get_id()),
        Opener::OgcdOpener((Some(db.tenri_jindo.get_id()), None)),
        Opener::GcdOpener(db.raiju.get_id()),
        Opener::OgcdOpener((None, None)),
        Opener::GcdOpener(db.armor_crush.get_id()),
        Opener::OgcdOpener((Some(db.bhavacakra.get_id()), None)),
        Opener::GcdOpener(db.raiton.get_id()),
        Opener::OgcdOpener((None, None)),
    ]
}

pub(crate) fn make_ninja_gcd_priority_table(db: &NinjaDatabase) -> Vec<SkillPriorityInfo> {
    vec![
        SkillPriorityInfo {
            skill_id: db.fuma_tenchijin.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.raiton_tenchijin.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.suiton_tenchijin.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.hyosho.get_id(),
            prerequisite: Some(HasBufforDebuff(db.kunais_bane_status.get_id())),
        },
        SkillPriorityInfo {
            skill_id: db.suiton.get_id(),
            prerequisite: Some(And(
                Box::new(RelatedSkillCooldownLessOrEqualThan(1010, 17000)),
                Box::new(Not(Box::new(HasBufforDebuff(1002)))),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.aeolian_edge.get_id(),
            prerequisite: Some(And(
                Box::new(SkillPrerequisite::Combo(Some(2))),
                Box::new(HasBufforDebuff(db.kunais_bane_status.get_id())),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.armor_crush.get_id(),
            prerequisite: Some(And(
                Box::new(Not(Box::new(HasResource(1, 4)))),
                Box::new(And(
                    Box::new(Not(Box::new(HasBufforDebuff(
                        db.kunais_bane_status.get_id(),
                    )))),
                    Box::new(SkillPrerequisite::Combo(Some(2))),
                )),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.phantom_kamaitachi.get_id(),
            prerequisite: Some(SkillPrerequisite::BufforDebuffLessThan(1007, 3000)),
        },
        SkillPriorityInfo {
            skill_id: db.raiju.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.raiton.get_id(),
            prerequisite: Some(And(
                Box::new(And(
                    Box::new(SkillPrerequisite::HasSkillStacks(1023, 2)),
                    Box::new(Not(Box::new(RelatedSkillCooldownLessOrEqualThan(
                        db.kunais_bane.get_id(),
                        30000,
                    )))),
                )),
                Box::new(Not(Box::new(HasBufforDebuff(db.kassatsu_status.get_id())))),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.phantom_kamaitachi.get_id(),
            prerequisite: Some(And(
                Box::new(SkillPrerequisite::MillisecondsBeforeBurst(0)),
                Box::new(HasBufforDebuff(1007)),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.raiton.get_id(),
            prerequisite: Some(And(
                Box::new(Not(Box::new(HasBufforDebuff(db.kassatsu_status.get_id())))),
                Box::new(Or(
                    Box::new(HasBufforDebuff(1003)),
                    Box::new(HasBufforDebuff(1004)),
                )),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.aeolian_edge.get_id(),
            prerequisite: Some(SkillPrerequisite::Combo(Some(2))),
        },
        SkillPriorityInfo {
            skill_id: db.gust_slash.get_id(),
            prerequisite: Some(SkillPrerequisite::Combo(Some(1))),
        },
        SkillPriorityInfo {
            skill_id: db.spinning_edge.get_id(),
            prerequisite: None,
        },
    ]
}

pub(crate) fn make_ninja_ogcd_priority_table(db: &NinjaDatabase) -> Vec<SkillPriorityInfo> {
    // TODO: calculate future ninki
    vec![
        SkillPriorityInfo {
            skill_id: db.potion.get_id(),
            prerequisite: Some(SkillPrerequisite::MillisecondsBeforeBurst(9000)),
        },
        SkillPriorityInfo {
            skill_id: db.bunshin.get_id(),
            prerequisite: Some(HasResource(0, 50)),
        },
        SkillPriorityInfo {
            skill_id: db.zesho_meppo_meisui.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.bhavacakra_meisui.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.zesho_meppo.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.bhavacakra.get_id(),
            prerequisite: Some(Or(
                Box::new(HasResource(0, 80)),
                Box::new(Not(Box::new(SkillPrerequisite::MillisecondsBeforeBurst(0)))),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.dokumori.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.kunais_bane.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.meisui.get_id(),
            prerequisite: Some(HasBufforDebuff(db.kunais_bane_status.get_id())),
        },
        SkillPriorityInfo {
            skill_id: db.kassatsu.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.dream.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.tenri_jindo.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.tenchijin.get_id(),
            prerequisite: None,
        },
    ]
}

use crate::rotation::priority_table::{Opener, PriorityTable};
use crate::rotation::SkillPriorityInfo;
use std::cell::RefCell;

use crate::id_entity::IdEntity;
use crate::jobs_skill_data::astrologian::abilities::AstrologianDatabase;
use crate::rotation::priority_table::Opener::{GcdOpener, OgcdOpener};
use crate::rotation::priority_table::SkillPrerequisite::{
    And, BufforDebuffLessThan, HasBufforDebuff, HasResource, MillisecondsBeforeBurst, Not, Or,
};
use crate::types::{PlayerIdType, SkillIdType};

#[derive(Clone)]
pub struct AstrologianPriorityTable {
    turn_count: RefCell<SkillIdType>,
    opener: Vec<Opener>,

    gcd_priority_table: Vec<SkillPriorityInfo>,
    ogcd_priority_table: Vec<SkillPriorityInfo>,
}

impl PriorityTable for AstrologianPriorityTable {
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

impl AstrologianPriorityTable {
    pub fn new(player_id: PlayerIdType, use_pots: bool) -> Self {
        let db = AstrologianDatabase::new(player_id);
        Self {
            turn_count: RefCell::new(0),
            opener: make_astrologian_opener(&db, use_pots),
            gcd_priority_table: make_astrologian_gcd_priority_table(&db),
            ogcd_priority_table: make_astrologian_ogcd_priority_table(&db, use_pots),
        }
    }
}

pub(crate) fn make_astrologian_opener(db: &AstrologianDatabase, use_pots: bool) -> Vec<Opener> {
    let mut openers = vec![
        OgcdOpener((Some(db.earthly_star.get_id()), None)),
        GcdOpener(db.fall_malefic.get_id()),
    ];

    if use_pots {
        openers.push(OgcdOpener((Some(db.potion.get_id()), None)));
    } else {
        openers.push(OgcdOpener((None, None)));
    }
    openers.extend(vec![
        GcdOpener(db.combust_iii.get_id()),
        OgcdOpener((Some(db.lightspeed.get_id()), None)),
        GcdOpener(db.fall_malefic_lightspeed.get_id()),
        OgcdOpener((None, None)),
        GcdOpener(db.fall_malefic_lightspeed.get_id()),
        OgcdOpener((Some(db.divination.get_id()), Some(db.the_balance.get_id()))),
        GcdOpener(db.fall_malefic_lightspeed.get_id()),
        OgcdOpener((
            Some(db.lord_of_crowns.get_id()),
            Some(db.umbral_draw.get_id()),
        )),
        GcdOpener(db.fall_malefic_lightspeed.get_id()),
        OgcdOpener((Some(db.the_spear.get_id()), Some(db.oracle.get_id()))),
        GcdOpener(db.fall_malefic_lightspeed.get_id()),
        OgcdOpener((Some(db.lightspeed.get_id()), None)),
        GcdOpener(db.fall_malefic_lightspeed.get_id()),
        OgcdOpener((None, None)),
        GcdOpener(db.fall_malefic_lightspeed.get_id()),
        OgcdOpener((None, None)),
        GcdOpener(db.fall_malefic_lightspeed.get_id()),
        OgcdOpener((None, None)),
        GcdOpener(db.fall_malefic_lightspeed.get_id()),
        OgcdOpener((None, None)),
        GcdOpener(db.combust_iii.get_id()),
    ]);

    openers
}

pub(crate) fn make_astrologian_gcd_priority_table(
    db: &AstrologianDatabase,
) -> Vec<SkillPriorityInfo> {
    vec![
        SkillPriorityInfo {
            skill_id: db.combust_iii.get_id(),
            prerequisite: Some(Or(
                Box::new(BufforDebuffLessThan(db.combust_iii_dot.get_id(), 1000)),
                Box::new(Not(Box::new(HasBufforDebuff(db.combust_iii_dot.get_id())))),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.fall_malefic_lightspeed.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.fall_malefic.get_id(),
            prerequisite: None,
        },
    ]
}

pub(crate) fn make_astrologian_ogcd_priority_table(
    db: &AstrologianDatabase,
    use_pots: bool,
) -> Vec<SkillPriorityInfo> {
    let mut skill_priorities = if use_pots {
        vec![SkillPriorityInfo {
            skill_id: db.potion.get_id(),
            prerequisite: Some(MillisecondsBeforeBurst(9000)),
        }]
    } else {
        vec![]
    };

    skill_priorities.extend(vec![
        SkillPriorityInfo {
            skill_id: db.divination.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.the_balance.get_id(),
            prerequisite: Some(MillisecondsBeforeBurst(0)),
        },
        SkillPriorityInfo {
            skill_id: db.lord_of_crowns.get_id(),
            prerequisite: Some(MillisecondsBeforeBurst(0)),
        },
        SkillPriorityInfo {
            skill_id: db.the_spear.get_id(),
            prerequisite: Some(MillisecondsBeforeBurst(0)),
        },
        SkillPriorityInfo {
            skill_id: db.lightspeed.get_id(),
            prerequisite: Some(MillisecondsBeforeBurst(3000)),
        },
        SkillPriorityInfo {
            skill_id: db.earthly_star.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.umbral_draw.get_id(),
            prerequisite: Some(And(
                Box::new(Not(Box::new(HasBufforDebuff(
                    db.the_balance_ready.get_id(),
                )))),
                Box::new(And(
                    Box::new(Not(Box::new(HasBufforDebuff(db.the_spear_ready.get_id())))),
                    Box::new(Not(Box::new(HasBufforDebuff(
                        db.lord_of_crowns_buff.get_id(),
                    )))),
                )),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.astral_draw.get_id(),
            prerequisite: Some(And(
                Box::new(Not(Box::new(HasBufforDebuff(
                    db.the_balance_ready.get_id(),
                )))),
                Box::new(And(
                    Box::new(Not(Box::new(HasBufforDebuff(db.the_spear_ready.get_id())))),
                    Box::new(And(
                        Box::new(Not(Box::new(HasBufforDebuff(
                            db.lord_of_crowns_buff.get_id(),
                        )))),
                        Box::new(Not(Box::new(HasResource(0, 1)))),
                    )),
                )),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.oracle.get_id(),
            prerequisite: Some(BufforDebuffLessThan(db.divination_buff.get_id(), 15000)),
        },
    ]);

    skill_priorities
}

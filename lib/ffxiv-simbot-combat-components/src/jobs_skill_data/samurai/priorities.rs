use crate::rotation::priority_table::{Opener, PriorityTable};
use crate::rotation::SkillPriorityInfo;
use crate::types::{PlayerIdType, SkillIdType};
use std::cell::RefCell;

use crate::id_entity::IdEntity;
use crate::jobs_skill_data::samurai::abilities::SamuraiDatabase;
use crate::rotation::priority_table::Opener::{GcdOpener, OgcdOpener};
use crate::rotation::priority_table::SkillPrerequisite::{
    And, BufforDebuffLessThan, Combo, HasBufforDebuff, HasResource, MillisecondsBeforeBurst, Not,
    Or,
};

#[derive(Clone)]
pub struct SamuraiPriorityTable {
    turn_count: RefCell<SkillIdType>,
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

impl SamuraiPriorityTable {
    pub fn new(player_id: PlayerIdType, use_pots: bool) -> Self {
        let db = SamuraiDatabase::new(player_id);
        Self {
            turn_count: RefCell::new(0),
            opener: make_samurai_opener(&db, use_pots),
            gcd_priority_table: make_samurai_gcd_priority_table(&db),
            ogcd_priority_table: make_samurai_ogcd_priority_table(&db, use_pots),
        }
    }
}

pub(crate) fn make_samurai_opener(db: &SamuraiDatabase, use_pots: bool) -> Vec<Opener> {
    let mut openers = if use_pots {
        vec![
            OgcdOpener((Some(db.meikyo_shisui.get_id()), None)),
            GcdOpener(db.gekko_meikyo.get_id()),
            OgcdOpener((Some(db.potion.get_id()), None)),
        ]
    } else {
        vec![
            OgcdOpener((Some(db.meikyo_shisui.get_id()), None)),
            GcdOpener(db.gekko_meikyo.get_id()),
            OgcdOpener((None, None)),
        ]
    };

    openers.extend(vec![
        GcdOpener(db.kasha_meikyo.get_id()),
        OgcdOpener((None, None)),
        GcdOpener(db.yukikaze.get_id()),
        OgcdOpener((None, None)),
        GcdOpener(db.tendo_setsugekka.get_id()),
        OgcdOpener((Some(db.meikyo_shisui.get_id()), None)),
        GcdOpener(db.gekko_meikyo.get_id()),
        OgcdOpener((
            Some(db.ikishoten.get_id()),
            Some(db.hissatsu_senei.get_id()),
        )),
        GcdOpener(db.higanbana.get_id()),
        OgcdOpener((Some(db.zanshin.get_id()), None)),
        GcdOpener(db.kaeshi_tendo_setsugekka.get_id()),
        OgcdOpener((None, None)),
        GcdOpener(db.ogi_namikiri.get_id()),
        OgcdOpener((Some(db.shoha.get_id()), None)),
        GcdOpener(db.kaeshi_namikiri.get_id()),
        OgcdOpener((None, None)),
        GcdOpener(db.kasha_meikyo.get_id()),
        OgcdOpener((None, None)),
        GcdOpener(db.gekko_meikyo.get_id()),
        OgcdOpener((Some(db.hissatsu_shinten.get_id()), None)),
        GcdOpener(db.gyofu.get_id()),
        OgcdOpener((None, None)),
        GcdOpener(db.yukikaze.get_id()),
        OgcdOpener((None, None)),
        GcdOpener(db.tendo_setsugekka.get_id()),
        OgcdOpener((None, None)),
        GcdOpener(db.kaeshi_tendo_setsugekka.get_id()),
    ]);

    openers
}

pub(crate) fn make_samurai_gcd_priority_table(db: &SamuraiDatabase) -> Vec<SkillPriorityInfo> {
    vec![
        SkillPriorityInfo {
            skill_id: db.higanbana.get_id(),
            prerequisite: Some(And(
                Box::new(And(
                    Box::new(Not(Box::new(HasResource(4, 1)))),
                    Box::new(Not(Box::new(HasResource(3, 1)))),
                )),
                Box::new(Or(
                    Box::new(BufforDebuffLessThan(db.higanbana_dot.get_id(), 3000)),
                    Box::new(And(
                        Box::new(MillisecondsBeforeBurst(0)),
                        Box::new(BufforDebuffLessThan(db.higanbana_dot.get_id(), 10000)),
                    )),
                )),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.higanbana_two.get_id(),
            prerequisite: Some(And(
                Box::new(And(
                    Box::new(Not(Box::new(HasResource(2, 1)))),
                    Box::new(Not(Box::new(HasResource(4, 1)))),
                )),
                Box::new(Or(
                    Box::new(BufforDebuffLessThan(db.higanbana_dot.get_id(), 3000)),
                    Box::new(And(
                        Box::new(MillisecondsBeforeBurst(0)),
                        Box::new(BufforDebuffLessThan(db.higanbana_dot.get_id(), 10000)),
                    )),
                )),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.higanbana_three.get_id(),
            prerequisite: Some(And(
                Box::new(And(
                    Box::new(Not(Box::new(HasResource(2, 1)))),
                    Box::new(Not(Box::new(HasResource(3, 1)))),
                )),
                Box::new(Or(
                    Box::new(BufforDebuffLessThan(db.higanbana_dot.get_id(), 3000)),
                    Box::new(And(
                        Box::new(MillisecondsBeforeBurst(0)),
                        Box::new(BufforDebuffLessThan(db.higanbana_dot.get_id(), 10000)),
                    )),
                )),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.kaeshi_setsugekka.get_id(),
            prerequisite: Some(Or(
                Box::new(MillisecondsBeforeBurst(0)),
                Box::new(And(
                    Box::new(HasResource(2, 1)),
                    Box::new(And(
                        Box::new(HasResource(3, 1)),
                        Box::new(HasResource(4, 1)),
                    )),
                )),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.tendo_setsugekka.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.gekko_meikyo.get_id(),
            prerequisite: Some(And(
                Box::new(Not(Box::new(HasResource(2, 1)))),
                Box::new(And(
                    Box::new(MillisecondsBeforeBurst(0)),
                    Box::new(BufforDebuffLessThan(db.higanbana_dot.get_id(), 7000)),
                )),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.kasha_meikyo.get_id(),
            prerequisite: Some(And(
                Box::new(Not(Box::new(HasResource(3, 1)))),
                Box::new(And(
                    Box::new(MillisecondsBeforeBurst(0)),
                    Box::new(BufforDebuffLessThan(db.higanbana_dot.get_id(), 7000)),
                )),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.yuki_meikyo.get_id(),
            prerequisite: Some(And(
                Box::new(Not(Box::new(HasResource(4, 1)))),
                Box::new(And(
                    Box::new(MillisecondsBeforeBurst(0)),
                    Box::new(BufforDebuffLessThan(db.higanbana_dot.get_id(), 7000)),
                )),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.kaeshi_tendo_setsugekka.get_id(),
            prerequisite: Some(Or(
                Box::new(MillisecondsBeforeBurst(0)),
                Box::new(And(
                    Box::new(HasResource(2, 1)),
                    Box::new(And(
                        Box::new(HasResource(3, 1)),
                        Box::new(HasResource(4, 1)),
                    )),
                )),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.kaeshi_namikiri.get_id(),
            prerequisite: Some(Or(
                Box::new(MillisecondsBeforeBurst(0)),
                Box::new(And(
                    Box::new(HasResource(2, 1)),
                    Box::new(And(
                        Box::new(HasResource(3, 1)),
                        Box::new(HasResource(4, 1)),
                    )),
                )),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.midare_setsugekka.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.ogi_namikiri.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.gekko_meikyo.get_id(),
            prerequisite: Some(Not(Box::new(HasResource(2, 1)))),
        },
        SkillPriorityInfo {
            skill_id: db.kasha_meikyo.get_id(),
            prerequisite: Some(Not(Box::new(HasResource(3, 1)))),
        },
        SkillPriorityInfo {
            skill_id: db.yuki_meikyo.get_id(),
            prerequisite: Some(Not(Box::new(HasResource(4, 1)))),
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
            prerequisite: Some(And(
                Box::new(Not(Box::new(HasResource(2, 1)))),
                Box::new(Combo(Some(2))),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.shifu.get_id(),
            prerequisite: Some(And(
                Box::new(Not(Box::new(HasResource(3, 1)))),
                Box::new(Combo(Some(2))),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.gyofu.get_id(),
            prerequisite: None,
        },
    ]
}

#[allow(unused)]
pub(crate) fn make_samurai_ogcd_priority_table(
    db: &SamuraiDatabase,
    use_pots: bool,
) -> Vec<SkillPriorityInfo> {
    let has_no_sen = And(
        Box::new(Not(Box::new(HasResource(2, 1)))),
        Box::new(And(
            Box::new(Not(Box::new(HasResource(3, 1)))),
            Box::new(Not(Box::new(HasResource(4, 1)))),
        )),
    );

    let has_only_first_sen = And(
        Box::new(HasResource(2, 1)),
        Box::new(And(
            Box::new(Not(Box::new(HasResource(3, 1)))),
            Box::new(Not(Box::new(HasResource(4, 1)))),
        )),
    );
    let has_only_second_sen = And(
        Box::new(HasResource(3, 1)),
        Box::new(And(
            Box::new(Not(Box::new(HasResource(2, 1)))),
            Box::new(Not(Box::new(HasResource(4, 1)))),
        )),
    );
    let has_only_third_sen = And(
        Box::new(HasResource(4, 1)),
        Box::new(And(
            Box::new(Not(Box::new(HasResource(3, 1)))),
            Box::new(Not(Box::new(HasResource(2, 1)))),
        )),
    );

    let has_one_two_sen = And(
        Box::new(HasResource(2, 1)),
        Box::new(And(
            Box::new(HasResource(3, 1)),
            Box::new(Not(Box::new(HasResource(4, 1)))),
        )),
    );
    let has_one_three_sen = And(
        Box::new(HasResource(2, 1)),
        Box::new(And(
            Box::new(HasResource(4, 1)),
            Box::new(Not(Box::new(HasResource(3, 1)))),
        )),
    );
    let has_two_three_sen = And(
        Box::new(HasResource(3, 1)),
        Box::new(And(
            Box::new(HasResource(4, 1)),
            Box::new(Not(Box::new(HasResource(2, 1)))),
        )),
    );

    let has_one_sen = Or(
        Box::new(Or(
            Box::new(has_only_first_sen),
            Box::new(has_only_second_sen),
        )),
        Box::new(has_only_third_sen),
    );
    let has_two_sens = Or(
        Box::new(Or(Box::new(has_one_two_sen), Box::new(has_one_three_sen))),
        Box::new(has_two_three_sen),
    );

    let has_three_sens = And(
        Box::new(HasResource(2, 1)),
        Box::new(And(
            Box::new(HasResource(3, 1)),
            Box::new(HasResource(4, 1)),
        )),
    );

    let mut skill_ogcd_priorities = if use_pots {
        vec![SkillPriorityInfo {
            skill_id: db.potion.get_id(),
            prerequisite: Some(MillisecondsBeforeBurst(7000)),
        }]
    } else {
        vec![]
    };

    skill_ogcd_priorities.extend(vec![
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
                Box::new(And(
                    Box::new(Combo(Some(0))),
                    Box::new(And(
                        Box::new(Not(Box::new(has_no_sen))),
                        Box::new(Not(Box::new(HasBufforDebuff(db.meikyo_shisui_buff.id)))),
                    )),
                )),
                Box::new(Or(
                    Box::new(BufforDebuffLessThan(db.higanbana_dot.get_id(), 9000)),
                    Box::new(MillisecondsBeforeBurst(5000)),
                )),
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
            skill_id: db.zanshin.get_id(),
            prerequisite: Some(MillisecondsBeforeBurst(0)),
        },
        SkillPriorityInfo {
            skill_id: db.hissatsu_shinten.get_id(),
            prerequisite: Some(Or(
                Box::new(MillisecondsBeforeBurst(0)),
                Box::new(HasResource(0, 50)),
            )),
        },
    ]);

    skill_ogcd_priorities
}

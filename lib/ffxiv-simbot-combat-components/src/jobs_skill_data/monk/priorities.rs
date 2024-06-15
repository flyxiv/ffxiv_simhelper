use crate::id_entity::IdEntity;
use crate::jobs_skill_data::monk::abilities::MonkDatabase;
use crate::rotation::priority_table::Opener::{GcdOpener, OgcdOpener};
use crate::rotation::priority_table::SkillPrerequisite::{
    And, BufforDebuffLessThan, Combo, HasBufforDebuff, HasResource, Not, Or,
    RelatedSkillCooldownLessThan,
};
use crate::rotation::priority_table::{Opener, PriorityTable};
use crate::rotation::SkillPriorityInfo;
use crate::{IdType, TurnCount};
use std::cell::RefCell;

#[derive(Clone)]
pub(crate) struct MonkPriorityTable {
    turn_count: RefCell<TurnCount>,
    opener: Vec<Opener>,

    gcd_priority_table: Vec<SkillPriorityInfo>,
    ogcd_priority_table: Vec<SkillPriorityInfo>,
}

impl PriorityTable for MonkPriorityTable {
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

impl MonkPriorityTable {
    pub fn new(player_id: IdType) -> Self {
        let db = MonkDatabase::new(player_id);
        Self {
            turn_count: RefCell::new(0),
            opener: make_monk_opener(&db),
            gcd_priority_table: make_monk_gcd_priority_table(&db),
            ogcd_priority_table: make_monk_ogcd_priority_table(&db),
        }
    }
}

pub(crate) fn make_monk_opener(db: &MonkDatabase) -> Vec<Opener> {
    vec![
        GcdOpener(db.dragon_kick.get_id()),
        OgcdOpener((Some(db.potion.get_id()), None)),
        GcdOpener(db.twin_snakes.get_id()),
        OgcdOpener((Some(db.riddle_of_fire.get_id()), None)),
        GcdOpener(db.demolish.get_id()),
        OgcdOpener((
            Some(db.riddle_of_wind.get_id()),
            Some(db.the_forbidden_chakra.get_id()),
        )),
        GcdOpener(db.bootshine.get_id()),
        OgcdOpener((
            Some(db.brotherhood.get_id()),
            Some(db.perfect_balance.get_id()),
        )),
    ]
}

pub(crate) fn make_monk_gcd_priority_table(db: &MonkDatabase) -> Vec<SkillPriorityInfo> {
    let in_lunar_perfect_balance = Box::new(And(
        Box::new(Not(Box::new(HasResource(4, 1)))),
        Box::new(Not(Box::new(HasResource(5, 1)))),
    ));
    let in_solar_perfect_balance = Box::new(And(
        Box::new(HasResource(4, 1)),
        Box::new(Not(Box::new(HasResource(5, 1)))),
    ));
    let in_last_perfect_balance = Box::new(And(
        Box::new(HasResource(4, 1)),
        Box::new(HasResource(5, 1)),
    ));

    vec![
        SkillPriorityInfo {
            skill_id: db.perfect_bootshine.get_id(),
            prerequisite: Some(And(
                Box::new(HasBufforDebuff(db.perfect_balance_buff.id)),
                Box::new(And(
                    Box::new(HasBufforDebuff(db.leaden_fist.id)),
                    Box::new(Or(
                        in_lunar_perfect_balance.clone(),
                        Box::new(Or(
                            Box::new(And(
                                in_solar_perfect_balance.clone(),
                                Box::new(Not(Box::new(HasResource(1, 1)))),
                            )),
                            in_last_perfect_balance.clone(),
                        )),
                    )),
                )),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.perfect_dragon_kick.get_id(),
            prerequisite: Some(And(
                Box::new(HasBufforDebuff(db.perfect_balance_buff.id)),
                Box::new(And(
                    Box::new(Not(Box::new(HasBufforDebuff(db.leaden_fist.id)))),
                    Box::new(Or(
                        in_lunar_perfect_balance.clone(),
                        Box::new(Or(
                            Box::new(And(
                                in_solar_perfect_balance.clone(),
                                Box::new(Not(Box::new(HasResource(1, 1)))),
                            )),
                            in_last_perfect_balance.clone(),
                        )),
                    )),
                )),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.perfect_twin_snakes.get_id(),
            prerequisite: Some(And(
                Box::new(HasBufforDebuff(db.perfect_balance_buff.id)),
                Box::new(And(
                    Box::new(BufforDebuffLessThan(db.disciplined_fist.id, 6000)),
                    Box::new(And(
                        in_solar_perfect_balance.clone(),
                        Box::new(Not(Box::new(HasResource(2, 1)))),
                    )),
                )),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.perfect_demolish.get_id(),
            prerequisite: Some(And(
                Box::new(HasBufforDebuff(db.perfect_balance_buff.id)),
                Box::new(And(
                    Box::new(BufforDebuffLessThan(db.demolish_dot.id, 6000)),
                    Box::new(And(
                        in_solar_perfect_balance.clone(),
                        Box::new(Not(Box::new(HasResource(3, 1)))),
                    )),
                )),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.perfect_snap_punch.get_id(),
            prerequisite: Some(And(
                Box::new(HasBufforDebuff(db.perfect_balance_buff.id)),
                Box::new(And(
                    in_solar_perfect_balance.clone(),
                    Box::new(Not(Box::new(HasResource(3, 1)))),
                )),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.perfect_true_strike.get_id(),
            prerequisite: Some(And(
                Box::new(HasBufforDebuff(db.perfect_balance_buff.id)),
                Box::new(And(
                    in_solar_perfect_balance.clone(),
                    Box::new(Not(Box::new(HasResource(2, 1)))),
                )),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.phantom_rush.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.rising_pheonix.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.elixir_field.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.bootshine.get_id(),
            prerequisite: Some(And(
                Box::new(HasBufforDebuff(db.leaden_fist.id)),
                Box::new(Combo(Some(1))),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.dragon_kick.get_id(),
            prerequisite: Some(Combo(Some(1))),
        },
        SkillPriorityInfo {
            skill_id: db.demolish.get_id(),
            prerequisite: Some(And(
                Box::new(BufforDebuffLessThan(db.demolish_dot.id, 6000)),
                Box::new(Combo(Some(3))),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.snap_punch.get_id(),
            prerequisite: Some(Combo(Some(3))),
        },
        SkillPriorityInfo {
            skill_id: db.twin_snakes.get_id(),
            prerequisite: Some(And(
                Box::new(BufforDebuffLessThan(db.disciplined_fist.id, 6000)),
                Box::new(Combo(Some(2))),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.true_strike.get_id(),
            prerequisite: Some(Combo(Some(2))),
        },
    ]
}

pub(crate) fn make_monk_ogcd_priority_table(db: &MonkDatabase) -> Vec<SkillPriorityInfo> {
    vec![
        SkillPriorityInfo {
            skill_id: db.potion.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.brotherhood.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.riddle_of_fire.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.riddle_of_wind.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.the_forbidden_chakra.get_id(),
            prerequisite: Some(HasResource(0, 5)),
        },
        SkillPriorityInfo {
            skill_id: db.perfect_balance.get_id(),
            prerequisite: Some(And(
                Box::new(Combo(Some(2))),
                Box::new(Or(
                    Box::new(RelatedSkillCooldownLessThan(
                        db.riddle_of_fire.get_id(),
                        7000,
                    )),
                    Box::new(HasBufforDebuff(db.riddle_of_fire.get_id())),
                )),
            )),
        },
    ]
}

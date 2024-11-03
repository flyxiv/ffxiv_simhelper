use crate::rotation::priority_table::{Opener, PriorityTable};
use crate::rotation::SkillPriorityInfo;
use std::cell::RefCell;

use crate::id_entity::IdEntity;
use crate::jobs_skill_data::viper::abilities::ViperDatabase;
use crate::rotation::priority_table::Opener::{GcdOpener, OgcdOpener};
use crate::rotation::priority_table::SkillPrerequisite::{
    And, BufforDebuffLessThan, Combo, ComboTimeLeftLessOrEqualTo, HasBufforDebuff, HasResource,
    HasResourceExactly, MillisecondsBeforeBurst, Not, Or,
};
use crate::types::{PlayerIdType, SkillIdType, TimeType};

#[derive(Clone)]
pub struct ViperPriorityTable {
    turn_count: RefCell<SkillIdType>,
    opener: Vec<Opener>,

    gcd_priority_table: Vec<SkillPriorityInfo>,
    ogcd_priority_table: Vec<SkillPriorityInfo>,
}

impl PriorityTable for ViperPriorityTable {
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

impl ViperPriorityTable {
    pub fn new(player_id: PlayerIdType, use_pots: bool) -> Self {
        let db = ViperDatabase::new(player_id);
        Self {
            turn_count: RefCell::new(0),
            opener: make_viper_opener(&db, use_pots),
            gcd_priority_table: make_viper_gcd_priority_table(&db),
            ogcd_priority_table: make_viper_ogcd_priority_table(&db, use_pots),
        }
    }
}

pub(crate) fn make_viper_opener(db: &ViperDatabase, use_pots: bool) -> Vec<Opener> {
    let mut opener = if use_pots {
        vec![
            GcdOpener(db.dread_fangs.get_id()),
            OgcdOpener((None, None)),
            GcdOpener(db.swiftskins_sting.get_id()),
            OgcdOpener((Some(db.serpents_ire.get_id()), None)),
            GcdOpener(db.vicewinder.get_id()),
            OgcdOpener((Some(db.potion.get_id()), None)),
        ]
    } else {
        vec![
            GcdOpener(db.dread_fangs.get_id()),
            OgcdOpener((None, None)),
            GcdOpener(db.swiftskins_sting.get_id()),
            OgcdOpener((Some(db.serpents_ire.get_id()), None)),
            GcdOpener(db.vicewinder.get_id()),
            OgcdOpener((None, None)),
        ]
    };

    opener.extend(vec![
        GcdOpener(db.hunters_coil.get_id()),
        OgcdOpener((
            Some(db.normal_filler1.get_id()),
            Some(db.normal_filler2.get_id()),
        )),
        GcdOpener(db.swiftskins_coil.get_id()),
        OgcdOpener((
            Some(db.normal_filler1.get_id()),
            Some(db.normal_filler2.get_id()),
        )),
        GcdOpener(db.reawaken_proc.get_id()),
        OgcdOpener((None, None)),
        GcdOpener(db.first_generation.get_id()),
        OgcdOpener((Some(db.reawaken_filler.get_id()), None)),
        GcdOpener(db.second_generation.get_id()),
        OgcdOpener((Some(db.reawaken_filler.get_id()), None)),
        GcdOpener(db.third_generation.get_id()),
        OgcdOpener((Some(db.reawaken_filler.get_id()), None)),
        GcdOpener(db.fourth_generation.get_id()),
        OgcdOpener((Some(db.reawaken_filler.get_id()), None)),
        GcdOpener(db.ouroboros.get_id()),
        OgcdOpener((None, None)),
        GcdOpener(db.hindsting_strike.get_id()),
        OgcdOpener((Some(db.normal_filler1.get_id()), None)),
    ]);

    opener
}

const COMBO_REFRESH_TIME_MILLISECOND: TimeType = 9000;
const COMBO_MAX_TIME_LEFT_MILLISECOND: TimeType = 9000;

pub(crate) fn make_viper_gcd_priority_table(db: &ViperDatabase) -> Vec<SkillPriorityInfo> {
    let steel_needs_refresh = Box::new(And(
        Box::new(Not(Box::new(HasBufforDebuff(db.honed_reavers.get_id())))),
        Box::new(BufforDebuffLessThan(
            db.honed_steels.get_id(),
            COMBO_MAX_TIME_LEFT_MILLISECOND,
        )),
    ));
    let dread_needs_refresh = Box::new(And(
        Box::new(Not(Box::new(HasBufforDebuff(db.honed_steels.get_id())))),
        Box::new(BufforDebuffLessThan(
            db.honed_reavers.get_id(),
            COMBO_MAX_TIME_LEFT_MILLISECOND,
        )),
    ));

    let needs_refresh = Or(steel_needs_refresh, dread_needs_refresh);

    vec![
        SkillPriorityInfo {
            skill_id: db.reawaken_proc.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.ouroboros.get_id(),
            prerequisite: Some(HasResourceExactly(7, 1)),
        },
        SkillPriorityInfo {
            skill_id: db.fourth_generation.get_id(),
            prerequisite: Some(HasResourceExactly(7, 2)),
        },
        SkillPriorityInfo {
            skill_id: db.third_generation.get_id(),
            prerequisite: Some(HasResourceExactly(7, 3)),
        },
        SkillPriorityInfo {
            skill_id: db.second_generation.get_id(),
            prerequisite: Some(HasResourceExactly(7, 4)),
        },
        SkillPriorityInfo {
            skill_id: db.first_generation.get_id(),
            prerequisite: Some(HasResourceExactly(7, 5)),
        },
        SkillPriorityInfo {
            skill_id: db.swiftskins_coil.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.hunters_coil.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.reawaken.get_id(),
            prerequisite: Some(Or(
                Box::new(HasResource(0, 100)),
                Box::new(Or(
                    Box::new(And(
                        Box::new(MillisecondsBeforeBurst(70000)),
                        Box::new(Not(Box::new(MillisecondsBeforeBurst(45000)))),
                    )),
                    Box::new(MillisecondsBeforeBurst(0)),
                )),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.uncoiled_fury.get_id(),
            prerequisite: Some(Or(
                Box::new(MillisecondsBeforeBurst(0)),
                Box::new(Not(Box::new(MillisecondsBeforeBurst(3000)))),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.flankstings_strike.get_id(),
            prerequisite: Some(And(
                Box::new(Combo(Some(3))),
                Box::new(Or(
                    Box::new(needs_refresh.clone()),
                    Box::new(ComboTimeLeftLessOrEqualTo(COMBO_REFRESH_TIME_MILLISECOND)),
                )),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.flanksbane_fang.get_id(),
            prerequisite: Some(And(
                Box::new(Combo(Some(3))),
                Box::new(Or(
                    Box::new(needs_refresh.clone()),
                    Box::new(ComboTimeLeftLessOrEqualTo(COMBO_REFRESH_TIME_MILLISECOND)),
                )),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.hindsting_strike.get_id(),
            prerequisite: Some(And(
                Box::new(Combo(Some(3))),
                Box::new(Or(
                    Box::new(needs_refresh.clone()),
                    Box::new(ComboTimeLeftLessOrEqualTo(COMBO_REFRESH_TIME_MILLISECOND)),
                )),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.hindsbane_fang.get_id(),
            prerequisite: Some(And(
                Box::new(Combo(Some(3))),
                Box::new(Or(
                    Box::new(needs_refresh.clone()),
                    Box::new(ComboTimeLeftLessOrEqualTo(COMBO_REFRESH_TIME_MILLISECOND)),
                )),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.hunters_sting.get_id(),
            prerequisite: Some(And(
                Box::new(Combo(Some(2))),
                Box::new(Or(
                    Box::new(needs_refresh.clone()),
                    Box::new(ComboTimeLeftLessOrEqualTo(COMBO_REFRESH_TIME_MILLISECOND)),
                )),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.swiftskins_sting.get_id(),
            prerequisite: Some(And(
                Box::new(Combo(Some(2))),
                Box::new(Or(
                    Box::new(needs_refresh.clone()),
                    Box::new(ComboTimeLeftLessOrEqualTo(COMBO_REFRESH_TIME_MILLISECOND)),
                )),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.steel_fangs.get_id(),
            prerequisite: Some(Or(
                Box::new(needs_refresh.clone()),
                Box::new(ComboTimeLeftLessOrEqualTo(COMBO_REFRESH_TIME_MILLISECOND)),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.dread_fangs.get_id(),
            prerequisite: Some(Or(
                Box::new(needs_refresh.clone()),
                Box::new(ComboTimeLeftLessOrEqualTo(COMBO_REFRESH_TIME_MILLISECOND)),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.vicewinder.get_id(),
            prerequisite: Some(Not(Box::new(MillisecondsBeforeBurst(10000)))),
        },
        SkillPriorityInfo {
            skill_id: db.flankstings_strike.get_id(),
            prerequisite: Some(Combo(Some(3))),
        },
        SkillPriorityInfo {
            skill_id: db.flanksbane_fang.get_id(),
            prerequisite: Some(Combo(Some(3))),
        },
        SkillPriorityInfo {
            skill_id: db.hindsting_strike.get_id(),
            prerequisite: Some(Combo(Some(3))),
        },
        SkillPriorityInfo {
            skill_id: db.hindsbane_fang.get_id(),
            prerequisite: Some(Combo(Some(3))),
        },
        SkillPriorityInfo {
            skill_id: db.hunters_sting.get_id(),
            prerequisite: Some(Combo(Some(2))),
        },
        SkillPriorityInfo {
            skill_id: db.swiftskins_sting.get_id(),
            prerequisite: Some(Combo(Some(2))),
        },
        SkillPriorityInfo {
            skill_id: db.steel_fangs.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.dread_fangs.get_id(),
            prerequisite: None,
        },
    ]
}

pub(crate) fn make_viper_ogcd_priority_table(
    db: &ViperDatabase,
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
            skill_id: db.reawaken_filler.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.death_rattle.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.normal_filler1.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.normal_filler2.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.serpents_ire.get_id(),
            prerequisite: None,
        },
    ]);

    ogcd_priorities
}

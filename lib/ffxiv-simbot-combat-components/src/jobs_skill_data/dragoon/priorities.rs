use crate::rotation::priority_table::{Opener, PriorityTable};
use crate::rotation::SkillPriorityInfo;
use crate::types::PlayerIdType;
use std::cell::RefCell;

use crate::id_entity::IdEntity;
use crate::jobs_skill_data::dragoon::abilities::DragoonDatabase;
use crate::rotation::priority_table::Opener::{GcdOpener, OgcdOpener};
use crate::rotation::priority_table::SkillPrerequisite::{
    And, BufforDebuffLessThan, Combo, HasBufforDebuff, MillisecondsBeforeBurst, Or,
    RelatedSkillCooldownLessOrEqualThan,
};
use crate::types::SkillIdType;

#[derive(Clone)]
pub struct DragoonPriorityTable {
    turn_count: RefCell<SkillIdType>,
    opener: Vec<Opener>,

    gcd_priority_table: Vec<SkillPriorityInfo>,
    ogcd_priority_table: Vec<SkillPriorityInfo>,
}

impl PriorityTable for DragoonPriorityTable {
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

impl DragoonPriorityTable {
    pub fn new(player_id: PlayerIdType) -> Self {
        let db = DragoonDatabase::new(player_id);
        Self {
            turn_count: RefCell::new(0),
            opener: make_dragoon_opener(&db),
            gcd_priority_table: make_dragoon_gcd_priority_table(&db),
            ogcd_priority_table: make_dragoon_ogcd_priority_table(&db),
        }
    }
}

pub(crate) fn make_dragoon_opener(db: &DragoonDatabase) -> Vec<Opener> {
    let dragoon_opener: Vec<Opener> = vec![
        GcdOpener(db.true_thrust.get_id()),
        OgcdOpener((None, None)),
        GcdOpener(db.spiral_blow.get_id()),
        OgcdOpener((None, Some(db.potion.get_id()))),
        GcdOpener(db.chaotic_spring.get_id()),
        OgcdOpener((
            Some(db.lance_charge.get_id()),
            Some(db.battle_litany.get_id()),
        )),
        GcdOpener(db.wheeling_thrust.get_id()),
        OgcdOpener((Some(db.high_jump.get_id()), Some(db.life_surge.get_id()))),
        GcdOpener(db.drakesbane_surge.get_id()),
        OgcdOpener((
            Some(db.geirskogul.get_id()),
            Some(db.dragonfire_dive.get_id()),
        )),
        GcdOpener(db.raiden_thrust.get_id()),
        OgcdOpener((Some(db.stardiver.get_id()), None)),
        GcdOpener(db.lance_barrage.get_id()),
        OgcdOpener((Some(db.life_surge.get_id()), Some(db.starcross.get_id()))),
        GcdOpener(db.heavens_thrust.get_id()),
        OgcdOpener((
            Some(db.nastrond.get_id()),
            Some(db.life_of_the_dragon.get_id()),
        )),
        GcdOpener(db.fang_and_claw.get_id()),
        OgcdOpener((Some(db.nastrond.get_id()), Some(db.mirage_dive.get_id()))),
        GcdOpener(db.drakesbane.get_id()),
        OgcdOpener((Some(db.nastrond.get_id()), None)),
        GcdOpener(db.raiden_thrust.get_id()),
        OgcdOpener((Some(db.wyrmwind_thrust.get_id()), None)),
    ];

    dragoon_opener
}

pub(crate) fn make_dragoon_gcd_priority_table(db: &DragoonDatabase) -> Vec<SkillPriorityInfo> {
    vec![
        SkillPriorityInfo {
            skill_id: db.heavens_thrust_surge.get_id(),
            prerequisite: Some(And(
                Box::new(HasBufforDebuff(db.life_surge_buff.get_id())),
                Box::new(Combo(Some(3))),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.drakesbane_surge.get_id(),
            prerequisite: Some(And(
                Box::new(Combo(Some(7))),
                Box::new(HasBufforDebuff(db.life_surge_buff.get_id())),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.drakesbane.get_id(),
            prerequisite: Some(Combo(Some(7))),
        },
        SkillPriorityInfo {
            skill_id: db.wheeling_thrust.get_id(),
            prerequisite: Some(Combo(Some(5))),
        },
        SkillPriorityInfo {
            skill_id: db.fang_and_claw.get_id(),
            prerequisite: Some(Combo(Some(6))),
        },
        SkillPriorityInfo {
            skill_id: db.chaotic_spring.get_id(),
            prerequisite: Some(Combo(Some(4))),
        },
        SkillPriorityInfo {
            skill_id: db.spiral_blow.get_id(),
            prerequisite: Some(And(
                Box::new(Combo(Some(2))),
                Box::new(BufforDebuffLessThan(801, 10000)),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.heavens_thrust.get_id(),
            prerequisite: Some(Combo(Some(3))),
        },
        SkillPriorityInfo {
            skill_id: db.lance_barrage.get_id(),
            prerequisite: Some(Combo(Some(2))),
        },
        SkillPriorityInfo {
            skill_id: db.raiden_thrust.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.true_thrust.get_id(),
            prerequisite: None,
        },
    ]
}

pub(crate) fn make_dragoon_ogcd_priority_table(db: &DragoonDatabase) -> Vec<SkillPriorityInfo> {
    vec![
        SkillPriorityInfo {
            skill_id: db.potion.get_id(),
            prerequisite: Some(MillisecondsBeforeBurst(9000)),
        },
        SkillPriorityInfo {
            skill_id: db.wyrmwind_thrust.get_id(),
            prerequisite: Some(Or(
                Box::new(HasBufforDebuff(db.draconian_fire.get_id())),
                Box::new(MillisecondsBeforeBurst(0)),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.battle_litany.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.lance_charge.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.geirskogul.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.high_jump.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.nastrond.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.mirage_dive.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.life_surge.get_id(),
            prerequisite: Some(And(
                Box::new(Or(Box::new(Combo(Some(3))), Box::new(Combo(Some(7))))),
                Box::new(Or(
                    Box::new(HasBufforDebuff(db.lance_charge_buff.id)),
                    Box::new(RelatedSkillCooldownLessOrEqualThan(
                        db.life_surge.get_id(),
                        10000,
                    )),
                )),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.wyrmwind_thrust.get_id(),
            prerequisite: Some(HasBufforDebuff(db.battle_litany_buff.get_id())),
        },
        SkillPriorityInfo {
            skill_id: db.starcross.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.rise_of_the_dragon.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.dragonfire_dive.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.stardiver.get_id(),
            prerequisite: None,
        },
    ]
}

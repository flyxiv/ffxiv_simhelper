use crate::live_objects::player::ffxiv_player::FfxivPlayer;
use crate::rotation::priority_table::{Opener, PriorityTable, SkillPrerequisite};
use crate::rotation::SkillPriorityInfo;
use crate::{IdType, TimeType, TurnCount};
use std::cell::RefCell;

use crate::id_entity::IdEntity;
use crate::jobs_skill_data::dragoon::abilities::DragoonDatabase;
use crate::rotation::priority_table::Opener::{GcdOpener, OgcdOpener};
use crate::rotation::priority_table::SkillPrerequisite::{
    And, BufforDebuffLessThan, Combo, HasBufforDebuff, HasResource, Not, Or,
    RelatedSkillCooldownLessThan,
};

#[derive(Clone)]
pub(crate) struct DragoonPriorityTable {
    turn_count: RefCell<TurnCount>,
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

impl DragoonPriorityTable {
    pub fn new(player_id: IdType, partner_player_id: IdType) -> Self {
        let db = DragoonDatabase::new(player_id, partner_player_id);
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
        OgcdOpener((Some(db.potion.get_id()), None)),
        GcdOpener(db.disembowel.get_id()),
        OgcdOpener((
            Some(db.lance_charge.get_id()),
            Some(db.dragon_sight.get_id()),
        )),
        GcdOpener(db.chaotic_spring.get_id()),
        OgcdOpener((Some(db.battle_litany.get_id()), Some(db.high_jump.get_id()))),
        GcdOpener(db.wheeling_thrust.get_id()),
        OgcdOpener((Some(db.geirskogul.get_id()), Some(db.life_surge.get_id()))),
        GcdOpener(db.fang_and_claw_plus_surge.get_id()),
        OgcdOpener((
            Some(db.spineshatter_dive.get_id()),
            Some(db.dragonfire_dive.get_id()),
        )),
        GcdOpener(db.raiden_thrust.get_id()),
        OgcdOpener((
            Some(db.spineshatter_dive.get_id()),
            Some(db.mirage_dive.get_id()),
        )),
        GcdOpener(db.vorpal_thrust.get_id()),
        OgcdOpener((Some(db.life_surge.get_id()), None)),
        GcdOpener(db.heavens_thrust.get_id()),
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
            skill_id: db.wheeling_thrust_plus_surge.get_id(),
            prerequisite: Some(And(
                Box::new(HasBufforDebuff(db.lance_mastery.get_id())),
                Box::new(And(
                    Box::new(Combo(Some(5))),
                    Box::new(HasBufforDebuff(db.life_surge_buff.get_id())),
                )),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.fang_and_claw_plus_surge.get_id(),
            prerequisite: Some(And(
                Box::new(HasBufforDebuff(db.lance_mastery.get_id())),
                Box::new(And(
                    Box::new(Combo(Some(6))),
                    Box::new(HasBufforDebuff(db.life_surge_buff.get_id())),
                )),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.wheeling_thrust_plus.get_id(),
            prerequisite: Some(And(
                Box::new(HasBufforDebuff(db.lance_mastery.get_id())),
                Box::new(Combo(Some(5))),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.fang_and_claw_plus.get_id(),
            prerequisite: Some(And(
                Box::new(HasBufforDebuff(db.lance_mastery.get_id())),
                Box::new(Combo(Some(6))),
            )),
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
            skill_id: db.disembowel.get_id(),
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
            skill_id: db.vorpal_thrust.get_id(),
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
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.nastrond.get_id(),
            prerequisite: Some(BufforDebuffLessThan(db.battle_litany_buff.get_id(), 12000)),
        },
        SkillPriorityInfo {
            skill_id: db.stardiver.get_id(),
            prerequisite: Some(SkillPrerequisite::BufforDebuffLessThan(
                db.battle_litany_buff.get_id(),
                12000,
            )),
        },
        SkillPriorityInfo {
            skill_id: db.wyrmwind_thrust.get_id(),
            prerequisite: Some(SkillPrerequisite::HasBufforDebuff(
                db.draconian_fire.get_id(),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.battle_litany.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.dragon_sight.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.lance_charge.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.mirage_dive.get_id(),
            prerequisite: Some(Or(
                Box::new(And(
                    Box::new(Not(Box::new(HasResource(0, 2)))),
                    Box::new(RelatedSkillCooldownLessThan(
                        db.lance_charge.get_id(),
                        20000,
                    )),
                )),
                Box::new(BufforDebuffLessThan(db.dive_ready.get_id(), 3000)),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.geirskogul_plus.get_id(),
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
            skill_id: db.life_surge.get_id(),
            prerequisite: Some(And(
                Box::new(SkillPrerequisite::MillisecondsBeforeBurst(0)),
                Box::new(Or(
                    Box::new(Combo(Some(2))),
                    Box::new(HasBufforDebuff(db.lance_mastery.id)),
                )),
            )),
        },
        SkillPriorityInfo {
            skill_id: db.spineshatter_dive.get_id(),
            prerequisite: Some(RelatedSkillCooldownLessThan(
                db.spineshatter_dive.get_id(),
                3000,
            )),
        },
        SkillPriorityInfo {
            skill_id: db.wyrmwind_thrust.get_id(),
            prerequisite: Some(HasBufforDebuff(db.battle_litany_buff.get_id())),
        },
        SkillPriorityInfo {
            skill_id: db.dragonfire_dive.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.stardiver.get_id(),
            prerequisite: None,
        },
        SkillPriorityInfo {
            skill_id: db.life_surge.get_id(),
            prerequisite: Some(And(
                Box::new(RelatedSkillCooldownLessThan(
                    db.life_surge_buff.get_id(),
                    20000,
                )),
                Box::new(Combo(Some(2))),
            )),
        },
    ]
}

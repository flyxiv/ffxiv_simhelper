use crate::id_entity::IdEntity;
use crate::live_objects::turn_type::{FfxivTurnType, PlayerTurn};
use crate::rotation::priority_table::SkillResult;
use crate::skill::Skill;
use crate::status::buff_status::BuffStatus;
use crate::status::debuff_status::DebuffStatus;
use crate::status::status_holder::StatusHolder;
use crate::{IdType, TimeType};
use std::cell::RefCell;
use std::rc::Rc;

mod combat_resources;
mod create_player;
/// If the delay is over 3 * OGCD delay, then it is turn to use a GCD skill,
/// Since in FFXIV a player can use at most 2 OGCD skills between GCD skills.
/// so 1 GCD delay + 2 oGCD delay = 3 * oGCD delay.
pub mod ffxiv_player;
pub mod gcd_calculator;
pub mod player_turn_calculator;

static MAX_MANA: i32 = 10000;

#[derive(Hash, Clone, Copy, PartialEq, Eq)]
pub(crate) struct StatusKey {
    skill_id: IdType,
    player_id: IdType,
}

impl StatusKey {
    pub fn new(skill_id: IdType, player_id: IdType) -> StatusKey {
        StatusKey {
            skill_id,
            player_id,
        }
    }
}

/// Saves information about the player: buffs, stat multipliers, rotation.
pub trait Player: Sized + StatusHolder<BuffStatus> + IdEntity {
    fn use_turn(
        &self,
        turn: FfxivTurnType,
        debuffs: Rc<RefCell<Vec<DebuffStatus>>>,
        combat_time_millisecond: TimeType,
    );
    fn use_skill(
        &self,
        skill_id: IdType,
        debuffs: Rc<RefCell<Vec<DebuffStatus>>>,
        combat_time_millisecond: TimeType,
    );
    fn consume_internal_events(&self, debuffs: Rc<RefCell<Vec<DebuffStatus>>>);
}

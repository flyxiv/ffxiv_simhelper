use crate::event::ffxiv_event::FfxivEvent;
use crate::id_entity::IdEntity;
use crate::status::buff_status::BuffStatus;
use crate::status::debuff_status::DebuffStatus;
use crate::status::status_holder::StatusHolder;
use crate::{IdType, StatusTable};

pub(crate) mod create_player;
/// If the delay is over 3 * OGCD delay, then it is turn to use a GCD skill,
/// Since in FFXIV a player can use at most 2 OGCD skills between GCD skills.
/// so 1 GCD delay + 2 oGCD delay = 3 * oGCD delay.
pub mod ffxiv_player;
pub mod gcd_calculator;
pub mod logs;
mod player_damage_profile;
pub mod player_turn_calculator;

static MAX_MANA: i32 = 10000;

#[derive(Hash, Clone, Copy, PartialEq, Eq)]
pub struct StatusKey {
    pub status_id: IdType,
    pub player_id: IdType,
}

impl StatusKey {
    pub fn new(status_id: IdType, player_id: IdType) -> StatusKey {
        StatusKey {
            status_id,
            player_id,
        }
    }
}

/// Saves information about the player: buffs, stat multipliers, rotation.
pub trait Player: Sized + StatusHolder<BuffStatus> + IdEntity {
    fn consume_internal_events(&self, debuffs: StatusTable<DebuffStatus>);
    fn handle_ffxiv_event(&mut self, event: FfxivEvent, debuffs: StatusTable<DebuffStatus>);
}

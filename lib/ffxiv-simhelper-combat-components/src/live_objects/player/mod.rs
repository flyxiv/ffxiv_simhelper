use crate::event::ffxiv_event::FfxivEvent;
use crate::status::buff_status::BuffStatus;
use crate::status::debuff_status::DebuffStatus;
use crate::status::status_holder::StatusHolder;
use crate::types::StatusTable;
use crate::types::{PlayerIdType, SkillIdType};

pub(crate) mod create_player;
/// If the delay is over 3 * OGCD delay, then it is turn to use a GCD skill,
/// Since in FFXIV a player can use at most 2 OGCD skills between GCD skills.
/// so 1 GCD delay + 2 oGCD delay = 3 * oGCD delay.
pub mod ffxiv_player;
pub mod gcd_calculator;
pub mod logs;
pub mod player_power;
pub mod player_turn_calculator;
pub mod role;

#[derive(Hash, Clone, Copy, PartialEq, Eq, Debug)]
pub struct StatusKey {
    pub status_id: SkillIdType,
    pub player_id: PlayerIdType,
}

impl StatusKey {
    pub fn new(status_id: SkillIdType, player_id: PlayerIdType) -> StatusKey {
        StatusKey {
            status_id,
            player_id,
        }
    }
}

/// Saves information about the player: buffs, stat multipliers, rotation.
pub trait Player: Sized + StatusHolder<BuffStatus> {
    fn consume_internal_events(&self, debuffs: StatusTable<DebuffStatus>);
    fn handle_ffxiv_event(&mut self, event: FfxivEvent, debuffs: StatusTable<DebuffStatus>);
}

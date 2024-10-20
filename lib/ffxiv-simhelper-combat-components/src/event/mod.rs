use crate::event::ffxiv_event::FfxivEvent;
use sorted_vec::ReverseSortedVec;

pub mod ffxiv_event;
pub(crate) mod ffxiv_player_internal_event;
pub mod turn_info;

pub type FfxivEventQueue = ReverseSortedVec<FfxivEvent>;

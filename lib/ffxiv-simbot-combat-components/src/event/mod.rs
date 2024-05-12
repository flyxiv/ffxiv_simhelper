use crate::event::ffxiv_event::FfxivEvent;
use sorted_vec::SortedVec;

pub mod ffxiv_event;
pub(crate) mod ffxiv_player_internal_event;
pub(crate) mod turn_info;

pub type FfxivEventQueue = SortedVec<FfxivEvent>;

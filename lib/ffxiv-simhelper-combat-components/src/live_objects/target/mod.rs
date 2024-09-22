use crate::event::ffxiv_event::FfxivEvent;
use crate::status::debuff_status::DebuffStatus;
use crate::status::status_holder::StatusHolder;

pub trait Target: StatusHolder<DebuffStatus> + Sized {
    fn handle_ffxiv_event(&mut self, event: FfxivEvent);
}

pub mod ffxiv_target;

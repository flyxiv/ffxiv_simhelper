use crate::status::debuff_status::DebuffStatus;
use crate::status::status_holder::StatusHolder;

pub trait Target: StatusHolder<DebuffStatus> + Sized {}

pub mod ffxiv_target;

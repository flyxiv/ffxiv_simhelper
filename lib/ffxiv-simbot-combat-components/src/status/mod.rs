use crate::id_entity::IdEntity;
use crate::live_objects::player::Player;
use crate::live_objects::target::Target;
use crate::owner_tracker::OwnerTracker;
use crate::skill::skill::Skill;
use crate::status::status_holder::StatusHolder;
use crate::status::status_info::StatusInfo;
use crate::TimeType;

pub mod buff_status;
pub mod debuff_status;
pub mod status_holder;
pub mod status_info;
pub(crate) mod status_timer;

/// Interface for player buffs and target debuffs
pub trait Status: Sized + IdEntity {
    /// in miliseconds
    fn get_duration_left_millisecond(&self) -> TimeType;
    fn set_duration_left_millisecond(&mut self, duration: TimeType);
    /// get the type of status and amount
    /// ex) Battle Litany: 10% Crit Buff = CritHitRatePercent(10)
    fn get_status_info(&self) -> StatusInfo;
    fn get_duration_millisecond(&self) -> TimeType;
}

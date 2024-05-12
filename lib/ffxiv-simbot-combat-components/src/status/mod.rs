use crate::id_entity::IdEntity;
use crate::skill::{Skill, SkillEvents};
use crate::status::buff_status::BuffStatus;
use crate::status::debuff_status::DebuffStatus;
use crate::status::status_info::StatusInfo;
use crate::{IdType, TimeType};

pub mod buff_status;
pub mod debuff_status;
pub mod status_holder;
pub mod status_info;
pub mod status_timer;

/// Interface for player buffs and target debuffs
pub trait Status: Sized + IdEntity {
    /// in miliseconds
    fn get_duration_left_millisecond(&self) -> TimeType;
    fn set_duration_left_millisecond(&mut self, duration: TimeType);
    /// add trigger event to skill use
    fn get_name(&self) -> &String;
    fn start_duration(&mut self) {
        self.set_duration_left_millisecond(self.get_duration_millisecond());
    }
    /// get the type of status and amount
    /// ex) Battle Litany: 10% Crit Buff = CritHitRatePercent(10)
    fn get_status_info(&self) -> StatusInfo;
    fn get_duration_millisecond(&self) -> TimeType;
    fn is_raidwide(&self) -> bool;
}

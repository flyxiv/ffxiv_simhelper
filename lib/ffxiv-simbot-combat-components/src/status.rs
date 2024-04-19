use crate::owner_tracker::OwnerTracker;
use crate::player::Player;
use crate::skill::Skill;
use crate::target::Target;
use crate::{BuffIncreaseType, DamageType, IdType, TimeType};
use std::cell::{RefCell, RefMut};
use std::rc::Rc;

#[derive(Copy, Clone, Debug)]
pub enum StatusInfo {
    DamagePercent(BuffIncreaseType),
    CritHitRatePercent(BuffIncreaseType),
    DirectHitRatePercent(BuffIncreaseType),
    SpeedPercent(BuffIncreaseType),
}

impl PartialEq<Self> for StatusInfo {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (StatusInfo::DamagePercent(a), StatusInfo::DamagePercent(b)) => a == b,
            (StatusInfo::CritHitRatePercent(a), StatusInfo::CritHitRatePercent(b)) => a == b,
            (StatusInfo::DirectHitRatePercent(a), StatusInfo::DirectHitRatePercent(b)) => a == b,
            (StatusInfo::SpeedPercent(a), StatusInfo::SpeedPercent(b)) => a == b,
            _ => false,
        }
    }
}

impl Eq for StatusInfo {}

/// Interface for player buffs and target debuffs
pub trait Status: Sized {
    fn get_id(&self) -> usize;
    /// in miliseconds
    fn get_duration_left_millisecond(&self) -> TimeType;
    fn set_duration_left_millisecond(&mut self, duration: TimeType);
    /// get the type of status and amount
    /// ex) Battle Litany: 10% Crit Buff = CritHitRatePercent(10)
    fn get_status_info(&self) -> StatusInfo;
    fn get_duration_millisecond(&self) -> TimeType;
}

/// Implements entity that hold buff/debuff status
/// which are characters and attack targets.
pub trait StatusHolder<S: Status>: Sized {
    fn get_status_list(&self) -> Rc<RefCell<Vec<S>>>;

    fn add_status(&mut self, status: S) {
        let status_list = self.get_status_list();
        let mut status_list = status_list.borrow_mut();

        status_list.push(status);
    }
}

/// Every time combat time updates,
/// Update the remaining time of buffs and debuffs and remove status that has expired.
pub trait StatusTimer<T: Status>: StatusHolder<T> {
    /// Update combat time by getting the time different and decreasing the
    /// time left on each buff and debuff.
    fn update_status_time(&mut self, elapsed_time: TimeType) {
        if elapsed_time <= 0 {
            return;
        }

        let status_list = self.get_status_list();
        let mut status_list = status_list.borrow_mut();

        for status in status_list.iter_mut() {
            status.set_duration_left_millisecond(
                status.get_duration_left_millisecond() - elapsed_time,
            );
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct BuffStatus {
    pub(crate) id: IdType,
    pub(crate) owner_id: IdType,
    pub(crate) duration_left_millisecond: TimeType,
    pub(crate) status_info: StatusInfo,
    pub(crate) duration_millisecond: TimeType,
    pub is_raidwide: bool,
    pub(crate) owner_player_id: IdType,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct DebuffStatus {
    pub(crate) id: IdType,
    pub(crate) owner_id: IdType,
    pub(crate) duration_left_millisecond: TimeType,
    pub(crate) status_info: StatusInfo,
    pub(crate) duration_millisecond: TimeType,
    pub(crate) owner_player_id: IdType,
}

impl Status for BuffStatus {
    fn get_id(&self) -> IdType {
        self.id
    }

    fn get_duration_left_millisecond(&self) -> i32 {
        self.duration_left_millisecond
    }
    fn set_duration_left_millisecond(&mut self, duration: TimeType) {
        self.duration_left_millisecond = duration;
    }

    fn get_status_info(&self) -> StatusInfo {
        self.status_info
    }

    fn get_duration_millisecond(&self) -> i32 {
        self.duration_millisecond
    }
}

impl OwnerTracker for BuffStatus {
    fn get_owner_id(&self) -> IdType {
        self.owner_player_id
    }
}

impl OwnerTracker for DebuffStatus {
    fn get_owner_id(&self) -> IdType {
        self.owner_player_id
    }
}

impl Status for DebuffStatus {
    fn get_id(&self) -> IdType {
        self.id
    }

    fn get_duration_left_millisecond(&self) -> i32 {
        self.duration_left_millisecond
    }
    fn set_duration_left_millisecond(&mut self, duration: TimeType) {
        self.duration_left_millisecond = duration;
    }

    fn get_status_info(&self) -> StatusInfo {
        self.status_info
    }

    fn get_duration_millisecond(&self) -> TimeType {
        self.duration_millisecond
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn buff_status_test() {
        let buff = BuffStatus {
            id: 1,
            duration_left_millisecond: 3000,
            status_info: StatusInfo::CritHitRatePercent(10),
            duration_millisecond: 15000,
            is_raidwide: false,
            cumulative_damage: None,
            owner_player_id: 0,
        };

        assert_eq!(buff.get_id(), 1);
        assert_eq!(buff.get_duration_left_millisecond(), 3000);
        assert_eq!(buff.get_status_info(), StatusInfo::CritHitRatePercent(10));
        assert_eq!(buff.get_duration_millisecond(), 15000);
    }

    fn debuff_status_test() {
        let debuff = DebuffStatus {
            id: 1,
            duration_left_millisecond: 3000,
            status_info: StatusInfo::CritHitRatePercent(10),
            duration_millisecond: 15000,
            cumulative_damage: None,
            owner_player_id: 0,
        };

        assert_eq!(debuff.get_id(), 1);
        assert_eq!(debuff.get_duration_left_millisecond(), 3000);
        assert_eq!(debuff.get_status_info(), StatusInfo::CritHitRatePercent(10));
        assert_eq!(debuff.get_duration_millisecond(), 15000);
    }
}

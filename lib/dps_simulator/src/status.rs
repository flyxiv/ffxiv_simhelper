use std::cmp::Ordering;

pub(crate) type TimeType = i32;

#[derive(Copy, Clone, PartialEq, Eq)]
pub(crate) enum StatusInfo {
    DamagePercent(usize),
    CritHitRatePercent(usize),
    DirectHitRatePercent(usize),
    SpeedPercent(usize),
}

/// Interface for player buffs and target debuffs
pub trait Status {
    fn get_id(&self) -> i32;
    /// in miliseconds
    fn get_duration_left_millisecond(&self) -> TimeType;
    /// get the type of status and amount
    /// ex) Battle Litany: 10% Crit Buff = CritHitRatePercent(10)
    fn get_status_info(&self) -> StatusInfo;
    fn get_duration_millisecond(&self) -> TimeType;
}

/// Implements entity that hold buff/debuff status
/// which are characters and attack targets.
pub trait StatusHolder<T: Status + Sized>: Sized {
    fn get_status_list(&mut self) -> &mut Vec<T>;
    fn get_combat_time(&self) -> i32;
    fn add_status(&mut self, status: T) {
        self.get_status_list().push(status);
    }
}

/// Every time combat time updates,
/// Update the remaining time of buffs and debuffs and remove status that has expired.
pub trait StatusTimer<T: Status>: StatusHolder<T> {
    /// Update combat time by getting the time different and decreasing the
    /// time left on each buff and debuff.
    fn update_combat_time<T>(&mut self, current_combat_time_millisecond: i32) {
        if self.get_combat_time() >= current_combat_time_millisecond {
            return;
        }

        let time_diff = current_combat_time_millisecond - self.get_combat_time();
        let mut buff_list: &mut T = &self.get_buff_list();

        for buff in buff_list.iter_mut() {
            buff.duration_left_millisecond -= time_diff;
        }

        buff_list.retain(|buff| buff.get_duration_left_millisecond() > 0);
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct BuffStatus {
    pub(crate) id: i32,
    pub(crate) duration_left_millisecond: TimeType,
    pub(crate) status_data: StatusInfo,
    pub(crate) duration_millisecond: TimeType,
    pub(crate) is_raidwide: bool,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct DebuffStatus {
    pub(crate) id: i32,
    pub(crate) duration_left_millisecond: TimeType,
    pub(crate) status_data: StatusInfo,
    pub(crate) duration_millisecond: TimeType,
}

impl Status for BuffStatus {
    fn get_id(&self) -> i32 {
        self.id
    }

    fn get_duration_left_millisecond(&self) -> i32 {
        self.duration_left_millisecond
    }

    fn get_status_info(&self) -> StatusInfo {
        self.status_data
    }

    fn get_duration_millisecond(&self) -> i32 {
        self.duration_millisecond
    }
}

impl Status for DebuffStatus {
    fn get_id(&self) -> i32 {
        self.id
    }

    fn get_duration_left_millisecond(&self) -> i32 {
        self.duration_left_millisecond
    }

    fn get_status_info(&self) -> StatusInfo {
        self.status_data
    }

    fn get_duration_millisecond(&self) -> i32 {
        self.duration_millisecond
    }
}

impl Ord for BuffStatus {
    fn cmp(&self, other: &Self) -> Ordering {
        self.duration_left_millisecond
            .cmp(&other.duration_left_millisecond)
    }
}

impl Ord for DebuffStatus {
    fn cmp(&self, other: &Self) -> Ordering {
        self.duration_left_millisecond
            .cmp(&other.duration_left_millisecond)
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
            status_data: StatusInfo::CritHitRatePercent(10),
            duration_millisecond: 15000,
            is_raidwide: false,
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
            status_data: StatusInfo::CritHitRatePercent(10),
            duration_millisecond: 15000,
        };

        assert_eq!(debuff.get_id(), 1);
        assert_eq!(debuff.get_duration_left_millisecond(), 3000);
        assert_eq!(debuff.get_status_info(), StatusInfo::CritHitRatePercent(10));
        assert_eq!(debuff.get_duration_millisecond(), 15000);
    }
}

use crate::id_entity::IdEntity;
use crate::owner_tracker::OwnerTracker;
use crate::status::status_info::StatusInfo;
use crate::status::Status;
use crate::{IdType, TimeType};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct BuffStatus {
    pub(crate) id: IdType,
    pub(crate) owner_id: IdType,
    pub(crate) duration_left_millisecond: TimeType,
    pub(crate) status_info: StatusInfo,
    pub(crate) duration_millisecond: TimeType,
    pub is_raidwide: bool,
}

impl Status for BuffStatus {
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

impl IdEntity for BuffStatus {
    fn get_id(&self) -> IdType {
        self.id
    }
}

impl OwnerTracker for BuffStatus {
    fn get_owner_id(&self) -> IdType {
        self.owner_id
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
}

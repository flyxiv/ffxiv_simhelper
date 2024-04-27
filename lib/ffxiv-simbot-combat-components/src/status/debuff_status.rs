use crate::id_entity::IdEntity;
use crate::owner_tracker::OwnerTracker;
use crate::status::status_info::StatusInfo;
use crate::{IdType, TimeType};
use crate::status::Status;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct DebuffStatus {
    pub(crate) id: IdType,
    pub(crate) owner_id: IdType,
    pub(crate) duration_left_millisecond: TimeType,
    pub(crate) status_info: StatusInfo,
    pub(crate) duration_millisecond: TimeType,
    pub(crate) owner_player_id: IdType,
}

impl Status for DebuffStatus {
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

impl IdEntity for DebuffStatus {
    fn get_id(&self) -> IdType {
        self.id
    }
}

impl OwnerTracker for DebuffStatus {
    fn get_owner_id(&self) -> IdType {
        self.owner_player_id
    }
}

#[cfg(test)]
mod tests {


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
}
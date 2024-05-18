use crate::id_entity::IdEntity;
use crate::owner_tracker::OwnerTracker;
use crate::status::status_info::StatusInfo;
use crate::status::Status;
use crate::{IdType, ResourceType, TimeType};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct BuffStatus {
    pub(crate) id: IdType,
    pub(crate) owner_id: IdType,
    pub(crate) duration_left_millisecond: TimeType,
    pub(crate) status_info: StatusInfo,
    pub(crate) duration_millisecond: TimeType,
    pub is_raidwide: bool,
    pub(crate) name: String,
    pub(crate) stacks: ResourceType,
}

impl Status for BuffStatus {
    fn get_duration_left_millisecond(&self) -> TimeType {
        self.duration_left_millisecond
    }
    fn set_duration_left_millisecond(&mut self, duration: TimeType) {
        self.duration_left_millisecond = duration;
    }

    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_status_info(&self) -> StatusInfo {
        self.status_info
    }

    fn get_duration_millisecond(&self) -> i32 {
        self.duration_millisecond
    }

    fn is_raidwide(&self) -> bool {
        self.is_raidwide
    }
    fn add_stack(&mut self, stack: ResourceType) {
        self.stacks += stack;
    }

    fn get_stack(&self) -> ResourceType {
        self.stacks
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

use crate::id_entity::IdEntity;
use crate::live_objects::player::StatusKey;
use crate::owner_tracker::OwnerTracker;
use crate::status::buff_status::BuffStatus;
use crate::status::status_info::StatusInfo;
use crate::status::Status;
use crate::{DamageType, IdType, ResourceType, StatusTable, TimeType};
use std::collections::HashMap;

pub(crate) type SnapshotTable = HashMap<IdType, Vec<DamageType>>;
#[derive(PartialEq, Eq, Clone)]
pub struct DebuffStatus {
    pub(crate) id: IdType,
    pub(crate) owner_id: IdType,
    pub(crate) potency: Option<DamageType>,
    pub(crate) duration_left_millisecond: TimeType,
    pub(crate) status_info: StatusInfo,
    pub(crate) duration_millisecond: TimeType,
    pub(crate) is_raidwide: bool,
    pub(crate) stacks: ResourceType,
    pub(crate) name: String,
    pub(crate) snapshotted_buffs: HashMap<StatusKey, BuffStatus>,
    pub(crate) snapshotted_debuffs: HashMap<StatusKey, DebuffStatus>,
}

impl Status for DebuffStatus {
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

    fn get_duration_millisecond(&self) -> TimeType {
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

impl IdEntity for DebuffStatus {
    fn get_id(&self) -> IdType {
        self.id
    }
}

impl OwnerTracker for DebuffStatus {
    fn get_owner_id(&self) -> IdType {
        self.owner_id
    }
}

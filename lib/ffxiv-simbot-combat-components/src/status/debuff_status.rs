use crate::event_ticker::PercentType;
use crate::id_entity::IdEntity;
use crate::live_objects::player::StatusKey;
use crate::owner_tracker::OwnerTracker;
use crate::skill::damage_category::DamageCategory;
use crate::status::buff_status::BuffStatus;
use crate::status::status_info::StatusInfo;
use crate::status::Status;
use crate::types::{IdType, TimeType};
use crate::types::{PotencyType, ResourceType};
use std::cmp::min;
use std::collections::HashMap;

pub(crate) type SnapshotTable = HashMap<IdType, Vec<PotencyType>>;
#[derive(PartialEq, Eq, Clone)]
pub struct DebuffStatus {
    pub(crate) id: IdType,
    pub(crate) owner_id: IdType,
    pub(crate) damage_skill_id: Option<IdType>,
    pub(crate) potency: Option<PotencyType>,
    pub(crate) trait_percent: Option<PercentType>,
    pub(crate) damage_category: Option<DamageCategory>,
    pub(crate) duration_left_millisecond: TimeType,
    pub(crate) status_info: Vec<StatusInfo>,
    pub(crate) duration_millisecond: TimeType,
    pub(crate) is_raidwide: bool,
    pub(crate) stacks: ResourceType,
    pub(crate) max_stacks: ResourceType,
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

    fn get_status_info(&self) -> &Vec<StatusInfo> {
        &self.status_info
    }

    fn get_duration_millisecond(&self) -> TimeType {
        self.duration_millisecond
    }

    fn is_raidwide(&self) -> bool {
        self.is_raidwide
    }
    fn add_stack(&mut self, stack: ResourceType) {
        self.stacks = min(self.stacks + stack, self.max_stacks);
    }
    fn get_stack(&self) -> ResourceType {
        self.stacks
    }

    fn get_damage_skill_id(&self) -> Option<IdType> {
        self.damage_skill_id
    }
}

impl DebuffStatus {
    pub fn is_damage_debuff(&self, player_id: IdType) -> bool {
        if self.owner_id != player_id && !self.is_raidwide {
            return false;
        }

        self.status_info
            .iter()
            .any(|status_info| match status_info {
                StatusInfo::DirectHitRatePercent(_)
                | StatusInfo::CritHitRatePercent(_)
                | StatusInfo::DamagePercent(_) => true,
                _ => false,
            })
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

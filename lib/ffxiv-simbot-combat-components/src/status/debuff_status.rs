use crate::event_ticker::PercentType;
use crate::id_entity::IdEntity;
use crate::owner_tracker::OwnerTracker;
use crate::skill::damage_category::DamageCategory;
use crate::status::status_info::StatusInfo;
use crate::status::Status;
use crate::types::{BuffIncreasePercentType, IdType, PlayerIdType, SkillStackType, TimeType};
use crate::types::{PotencyType, SnapshotTable};
use std::cmp::min;

#[derive(PartialEq, Eq, Clone)]
pub struct DebuffStatus {
    pub(crate) id: IdType,
    pub(crate) owner_id: PlayerIdType,
    pub(crate) damage_skill_id: Option<IdType>,
    pub(crate) potency: Option<PotencyType>,
    pub(crate) trait_percent: Option<PercentType>,
    pub(crate) damage_category: Option<DamageCategory>,
    pub(crate) duration_left_millisecond: TimeType,
    pub(crate) status_info: Vec<StatusInfo>,
    pub(crate) duration_millisecond: TimeType,
    pub(crate) is_raidwide: bool,
    pub(crate) stacks: SkillStackType,
    pub(crate) max_stacks: SkillStackType,
    pub(crate) name: String,
    pub(crate) snapshotted_infos: SnapshotTable,
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
    fn add_stack(&mut self, stack: SkillStackType) {
        self.stacks = min(self.stacks + stack, self.max_stacks);
    }
    fn get_stack(&self) -> SkillStackType {
        self.stacks
    }

    fn get_damage_skill_id(&self) -> Option<IdType> {
        self.damage_skill_id
    }
}

impl DebuffStatus {
    pub fn get_damage_buff_infos(&self, player_id: PlayerIdType) -> Vec<StatusInfo> {
        if self.owner_id != player_id && !self.is_raidwide {
            return vec![];
        }
        self.status_info
            .clone()
            .into_iter()
            .filter_map(|status_info| match status_info {
                StatusInfo::DirectHitRatePercent(_)
                | StatusInfo::CritHitRatePercent(_)
                | StatusInfo::IncreaseMainStat(_, _) => Some(status_info),
                StatusInfo::DamagePercent(increase) => Some(StatusInfo::DamagePercent(
                    increase * self.get_stack() as BuffIncreasePercentType,
                )),
                _ => None,
            })
            .collect()
    }
}

impl IdEntity for DebuffStatus {
    fn get_id(&self) -> IdType {
        self.id
    }
}

impl OwnerTracker for DebuffStatus {
    fn get_owner_id(&self) -> PlayerIdType {
        self.owner_id
    }
}

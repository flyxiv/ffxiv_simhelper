use crate::status::status_event::StatusApplyType;
use crate::status::Status;
use crate::{IdType, TimeType};

#[derive(Clone)]
pub struct ApplyInfo<S: Status> {
    pub status: S,
    pub refresh_duration: TimeType,
    pub max_duration: TimeType,
}

#[derive(Clone)]
pub enum StatusApply<S: Status> {
    AddOrRefreshStatus(ApplyInfo<S>, StatusApplyType),
    AddStatus(ApplyInfo<S>, StatusApplyType),
}

impl<S: Status> StatusApply<S> {
    pub fn is_raidwide(&self) -> bool {
        match self {
            StatusApply::AddOrRefreshStatus(apply_info, _) => apply_info.status.is_raidwide(),
            StatusApply::AddStatus(apply_info, _) => apply_info.status.is_raidwide(),
        }
    }

    pub fn get_apply_type(&self) -> &StatusApplyType {
        match self {
            StatusApply::AddOrRefreshStatus(_, status_apply_type) => status_apply_type,
            StatusApply::AddStatus(_, status_apply_type) => status_apply_type,
        }
    }

    pub fn get_status(&self) -> &S {
        match self {
            StatusApply::AddOrRefreshStatus(apply_info, _) => &apply_info.status,
            StatusApply::AddStatus(apply_info, _) => &apply_info.status,
        }
    }

    pub fn set_buff(&mut self, buff: S) {
        match self {
            StatusApply::AddOrRefreshStatus(apply_info, _) => apply_info.status = buff,
            StatusApply::AddStatus(apply_info, _) => apply_info.status = buff,
        }
    }

    /// 0 for AddOrRefreshStatus, 1 for AddStatus
    pub(crate) fn new(status: S, status_type: IdType, status_apply_type: StatusApplyType) -> Self {
        let duration = status.get_duration_millisecond();
        let apply_info = ApplyInfo {
            status,
            refresh_duration: duration,
            max_duration: duration,
        };

        if status_type == 0 {
            StatusApply::AddOrRefreshStatus(apply_info, status_apply_type)
        } else {
            StatusApply::AddStatus(apply_info, status_apply_type)
        }
    }

    /// 0 for AddOrRefreshStatus, 1 for AddStatus
    pub(crate) fn new_with_duration(
        status: S,
        refresh_duration: TimeType,
        max_duration: TimeType,
        status_type: IdType,
        status_apply_type: StatusApplyType,
    ) -> Self {
        let apply_info = ApplyInfo {
            status,
            refresh_duration,
            max_duration,
        };

        if status_type == 0 {
            StatusApply::AddOrRefreshStatus(apply_info, status_apply_type)
        } else {
            StatusApply::AddStatus(apply_info, status_apply_type)
        }
    }
}

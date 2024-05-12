use crate::status::status_apply::StatusApply;
use crate::status::Status;
use crate::IdType;
use std::cell::RefCell;
use std::cmp::min;
use std::collections::HashMap;
use std::rc::Rc;

/// Implements entity that hold buff/debuff status
/// which are characters and attack targets.
pub trait StatusHolder<S: Status>: Sized {
    fn get_status_list(&self) -> Rc<RefCell<HashMap<IdType, S>>>;

    fn add_status(&mut self, status: S) {
        let status_list = self.get_status_list();
        let mut status_list = status_list.borrow_mut();

        match status {
            StatusApply::AddOrRefreshStatus(apply_info) => {
                let mut status = apply_info.status;
                let status_id = status.get_id();
                let mut found = false;
                for status in status_list.iter_mut() {
                    if status.get_id() == status_id {
                        let refreshed_duration = min(
                            apply_info.refresh_duration + status.get_duration_left_millisecond(),
                            apply_info.max_duration,
                        );
                        status.set_duration_left_millisecond(refreshed_duration);
                        found = true;
                        break;
                    }
                }
                if !found {
                    status.start_duration();
                    status_list.push(status);
                }
            }
            StatusApply::AddStatus(mut apply_info) => {
                apply_info.status.start_duration();
                status_list.push(apply_info.status);
            }
        }
    }
}

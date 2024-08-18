use crate::status::status_holder::StatusHolder;
use crate::status::Status;
use crate::types::TimeType;
use log::debug;

/// Every time combat time updates,
/// Update the remaining time of buffs and debuffs and remove status that has expired.
pub trait StatusTimer<T: Status>: StatusHolder<T> {
    /// Update combat time by getting the time different and decreasing the
    /// time left on each buff and debuff.
    fn update_status_time(&mut self, elapsed_time: TimeType) {
        if elapsed_time <= 0 {
            return;
        }

        let status_list = self.get_status_table();
        let mut status_list = status_list.borrow_mut();

        for status in status_list.values_mut() {
            status.set_duration_left_millisecond(
                status.get_duration_left_millisecond() - elapsed_time,
            );
        }

        status_list.retain(|_, status| status.get_duration_left_millisecond() > 0);

        for status in status_list.values_mut() {
            debug!(
                "status {} remaining time: {}",
                status.get_id(),
                status.get_duration_left_millisecond()
            )
        }
    }
}

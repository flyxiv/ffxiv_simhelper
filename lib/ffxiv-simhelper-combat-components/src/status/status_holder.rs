use crate::live_objects::player::StatusKey;
use crate::status::Status;
use crate::types::{PlayerIdType, TimeType};
use std::cell::RefCell;
use std::cmp::min;
use std::collections::HashMap;
use std::rc::Rc;

/// Implements entity that hold buff/debuff status
/// which are characters and attack targets.
pub trait StatusHolder<S: Status>: Sized {
    fn get_status_table(&self) -> Rc<RefCell<HashMap<StatusKey, S>>>;

    fn add_status(
        &mut self,
        mut status: S,
        duration_millisecond: TimeType,
        max_duration_millisecond: TimeType,
        player_id: PlayerIdType,
    ) {
        let key = StatusKey::new(status.get_id(), player_id);
        let status_table = self.get_status_table();

        if status_table.borrow().contains_key(&key) {
            let mut status_table = status_table.borrow_mut();
            let old_status = status_table.get(&key).unwrap();

            let new_duration = min(
                old_status.get_duration_left_millisecond() + duration_millisecond,
                max_duration_millisecond,
            );

            status.set_duration_left_millisecond(new_duration);
            status_table.remove(&key);
            status_table.insert(key, status);
        } else {
            status.start_duration();
            status_table.borrow_mut().insert(key, status);
        }
    }

    fn add_status_stack(
        &mut self,
        mut status: S,
        duration_millisecond: TimeType,
        refresh: bool,
        player_id: PlayerIdType,
    ) {
        let key = StatusKey::new(status.get_id(), player_id);
        let status_table = self.get_status_table();
        if status_table.borrow().contains_key(&key) {
            let mut live_status = status_table.borrow_mut();
            let status = live_status.get_mut(&key).unwrap();
            let stack = status.get_stack();

            let new_duration = if refresh {
                duration_millisecond
            } else {
                status.get_duration_left_millisecond()
            };
            status.set_duration_left_millisecond(new_duration);
            status.add_stack(stack);
        } else {
            status.start_duration();
            status_table.borrow_mut().insert(key, status);
        }
    }
}

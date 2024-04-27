use crate::status::Status;
use std::cell::RefCell;
use std::rc::Rc;

/// Implements entity that hold buff/debuff status
/// which are characters and attack targets.
pub trait StatusHolder<S: Status>: Sized {
    fn get_status_list(&self) -> Rc<RefCell<Vec<S>>>;

    fn add_status(&mut self, status: S) {
        let status_list = self.get_status_list();
        let mut status_list = status_list.borrow_mut();

        status_list.push(status);
    }
}

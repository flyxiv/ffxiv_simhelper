use crate::live_objects::target::Target;
use crate::status::debuff_status::DebuffStatus;
use crate::status::status_holder::StatusHolder;
use crate::status::status_timer::StatusTimer;
use crate::TimeType;
use std::cell::RefCell;
use std::rc::Rc;

/// Stores the debuff list of the target
/// debuff list will be sorted in the order of debuff time left so that
/// it is easy to search which debuffs will be removed.
pub struct FfxivTarget {
    pub debuff_list: Rc<RefCell<Vec<DebuffStatus>>>,
    // TODO: Add Dots
    pub combat_time_millisecond: TimeType,
}

impl Target for FfxivTarget {}

impl StatusHolder<DebuffStatus> for FfxivTarget {
    fn get_status_list(&self) -> Rc<RefCell<Vec<DebuffStatus>>> {
        self.debuff_list.clone()
    }
}

impl StatusTimer<DebuffStatus> for FfxivTarget {}

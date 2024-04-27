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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::status::debuff_status::DebuffStatus;
    use crate::status::status_info::StatusInfo;

    #[test]
    fn target_basic_test() {
        let mut target = FfxivTarget {
            debuff_list: RefCell::new(vec![]),
            combat_time_millisecond: 0,
        };

        let debuff1 = DebuffStatus {
            id: 1,
            duration_left_millisecond: 1000,
            status_data: StatusInfo::CritHitRatePercent(10),
            duration_millisecond: 1000,
            cumulative_damage: None,
            owner_player_id: 0,
        };

        target.add_status(debuff1);

        let debuff_list = target.get_status_list();
        assert_eq!(debuff_list.len(), 1);

        let debuff = &debuff_list[0];
        assert_eq!(debuff.id, 1);
        assert_eq!(debuff.get_duration_left_millisecond(), 1000);
        assert_eq!(debuff.get_status_info(), StatusInfo::CritHitRatePercent(10));
    }

    #[test]
    fn target_debuff_timer_test() {
        let mut target = FfxivTarget {
            debuff_list: RefCell::new(vec![]),
            combat_time_millisecond: 50000,
        };

        let two_seconds_left_debuff = DebuffStatus {
            id: 1,
            duration_left_millisecond: 2000,
            status_data: StatusInfo::CritHitRatePercent(10),
            duration_millisecond: 10000,
            cumulative_damage: None,
            owner_player_id: 0,
        };

        let five_seconds_left_debuff = DebuffStatus {
            id: 2,
            duration_left_millisecond: 5000,
            status_data: StatusInfo::CritHitRatePercent(10),
            duration_millisecond: 10000,
            cumulative_damage: None,
            owner_player_id: 0,
        };

        target.add_status(two_seconds_left_debuff);
        target.add_status(five_seconds_left_debuff);

        target.update_combat_time(3000);
        assert_eq!(target.get_status_list().len(), 2);

        target.update_combat_time(53000);
        assert_eq!(target.get_status_list().len(), 1);

        let debuff = &target.get_status_list()[0];
        assert_eq!(debuff.get_id(), 2);
        assert_eq!(debuff.get_duration_left_millisecond(), 2000);
    }
}

use crate::status::{BuffStatus, StatusHolder, StatusTimer};
use crate::TimeType;
use ffxiv_simbot_lib_db::job::Job;
use ffxiv_simbot_lib_db::stat_calculator::CharacterPower;
use sorted_vec::SortedVec;
use std::cell::{Ref, RefCell, RefMut};

/// Saves information about the player: buffs, stat multipliers, jobs.
pub trait Player {
    fn get_job(&self) -> &Job;
    fn get_power(&self) -> &CharacterPower;
    fn apply_buff(&mut self, buff: BuffStatus);
}

pub struct FfxivPlayer {
    job: Job,
    power: CharacterPower,
    buff_list: RefCell<Vec<BuffStatus>>,
    combat_time_millisecond: i32,
}

impl Player for FfxivPlayer {
    fn get_job(&self) -> &Job {
        &self.job
    }

    fn get_power(&self) -> &CharacterPower {
        &self.power
    }
    fn apply_buff(&mut self, buff: BuffStatus) {
        self.get_status_list_mut().push(buff);
    }
}

impl StatusHolder<BuffStatus> for FfxivPlayer {
    fn get_status_list(&self) -> Ref<Vec<BuffStatus>> {
        self.buff_list.borrow()
    }

    fn get_status_list_mut(&self) -> RefMut<Vec<BuffStatus>> {
        self.buff_list.borrow_mut()
    }

    fn get_combat_time_millisecond(&self) -> TimeType {
        self.combat_time_millisecond
    }
}

impl StatusTimer<BuffStatus> for FfxivPlayer {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::status::{Status, StatusInfo};

    #[test]
    fn target_basic_test() {
        let mut target = FfxivPlayer {
            job: Job::default(),
            power: CharacterPower::default(),
            buff_list: RefCell::new(vec![]),
            combat_time_millisecond: 0,
        };

        let buff1 = BuffStatus {
            id: 1,
            duration_left_millisecond: 1000,
            status_data: StatusInfo::CritHitRatePercent(10),
            duration_millisecond: 1000,
            is_raidwide: false,
        };

        target.add_status(buff1);
        assert_eq!(target.get_status_list().len(), 1);

        let buff = &target.get_status_list()[0];
        assert_eq!(buff.id, 1);
        assert_eq!(buff.get_duration_left_millisecond(), 1000);
        assert_eq!(buff.get_status_info(), StatusInfo::CritHitRatePercent(10));
    }

    #[test]
    fn target_debuff_timer_test() {
        let mut target = FfxivPlayer {
            job: Job::default(),
            power: CharacterPower::default(),
            buff_list: RefCell::new(vec![]),
            combat_time_millisecond: 0,
        };

        let two_seconds_left_buff = BuffStatus {
            id: 1,
            duration_left_millisecond: 2000,
            status_data: StatusInfo::CritHitRatePercent(10),
            duration_millisecond: 10000,
            is_raidwide: false,
        };

        let five_seconds_left_buff = BuffStatus {
            id: 2,
            duration_left_millisecond: 5000,
            status_data: StatusInfo::CritHitRatePercent(10),
            duration_millisecond: 10000,
            is_raidwide: true,
        };

        target.add_status(two_seconds_left_buff);
        target.add_status(five_seconds_left_buff);

        target.update_combat_time(3000);

        assert_eq!(target.get_status_list().len(), 1);

        let buff = &target.get_status_list()[0];
        assert_eq!(buff.id, 2);
        assert_eq!(buff.get_duration_left_millisecond(), 2000);
    }
}

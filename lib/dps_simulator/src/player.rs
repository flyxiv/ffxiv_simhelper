use crate::status::{BuffStatus, StatusHolder, StatusTimer};
use ffxiv_simbot_lib_db::job::Job;
use ffxiv_simbot_lib_db::stat_calculator::CharacterPower;

/// Saves information about the player: buffs, stat multipliers, jobs.
pub trait Player {
    fn get_job(&self) -> &Job;
    fn get_power(&self) -> &CharacterPower;
}

pub struct FfxivPlayer {
    job: Job,
    power: CharacterPower,
    buff_list: Vec<BuffStatus>,
    combat_time_millisecond: i32,
}

impl Player for FfxivPlayer {
    fn get_job(&self) -> &Job {
        &self.job
    }

    fn get_power(&self) -> &CharacterPower {
        &self.power
    }
}

impl StatusHolder<BuffStatus> for FfxivPlayer {
    fn get_status_list(&mut self) -> &mut Vec<BuffStatus> {
        &mut self.buff_list
    }

    fn get_combat_time(&self) -> i32 {
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
            buff_list: vec![],
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
        assert_eq!(target.get_status_list(), 1);

        let buff = &target.get_status_list()[0];
        assert_eq!(target.get()[0].get_id(), 1);
        assert_eq!(
            target.get_debuff_list()[0].get_duration_left_millisecond(),
            1000
        );
        assert_eq!(
            target.get_debuff_list()[0].get_status_info(),
            StatusInfo::CritHitRatePercent(10)
        );
    }

    #[test]
    fn target_debuff_timer_test() {
        let mut target = FfxivPlayer {
            job: Job::default(),
            power: CharacterPower::default(),
            buff_list: vec![],
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

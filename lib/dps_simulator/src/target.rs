use crate::status::{DebuffStatus, Status, StatusHolder, StatusInfo, StatusTimer};
use crate::TimeType;
use ffxiv_simbot_lib_db::stat_calculator::CharacterPower;
use std::cell::{Ref, RefCell, RefMut};

static DIRECT_HIT_DAMAGE_MULTIPLIER: f64 = 0.25f64;

fn get_increase_rate(rate: usize) -> f64 {
    1.0f64 + (rate as f64 / 100f64)
}

pub trait Target: StatusHolder<DebuffStatus> {
    fn get_debuff_multiplier(&self, character: &CharacterPower) -> f64 {
        let debuffs = self.get_status_list();
        let debuffs: &Vec<DebuffStatus> = debuffs.as_ref();

        let critical_strike_damage = character.critical_strike_damage - 1.0f64;
        let mut critical_strike_rate_increase = 1.0f64;
        let mut direct_hit_rate_increase = 1.0f64;
        let mut damage_increase = 1.0f64;

        for debuff in debuffs {
            match debuff.get_status_info() {
                StatusInfo::CritHitRatePercent(rate) => {
                    critical_strike_rate_increase *= get_increase_rate(rate)
                }
                StatusInfo::DirectHitRatePercent(rate) => {
                    direct_hit_rate_increase *= get_increase_rate(rate)
                }
                StatusInfo::DamagePercent(rate) => damage_increase *= get_increase_rate(rate),
                StatusInfo::SpeedPercent(rate) => damage_increase *= get_increase_rate(rate),
            }
        }

        let critical_strike_multiplier = critical_strike_damage * critical_strike_rate_increase;
        let direct_hit_multiplier = DIRECT_HIT_DAMAGE_MULTIPLIER * direct_hit_rate_increase;
        let damage_multiplier = damage_increase;

        return damage_multiplier * direct_hit_multiplier * critical_strike_multiplier;
    }
}

/// Stores the debuff list of the target
/// debuff list will be sorted in the order of debuff time left so that
/// it is easy to search which debuffs will be removed.
pub struct FfxivTarget {
    debuff_list: RefCell<Vec<DebuffStatus>>,
    combat_time_millisecond: TimeType,
}

impl StatusHolder<DebuffStatus> for FfxivTarget {
    fn get_status_list(&self) -> Ref<Vec<DebuffStatus>> {
        self.debuff_list.borrow()
    }

    fn get_status_list_mut(&self) -> RefMut<Vec<DebuffStatus>> {
        self.debuff_list.borrow_mut()
    }

    fn get_combat_time_millisecond(&self) -> i32 {
        self.combat_time_millisecond
    }
}

impl StatusTimer<DebuffStatus> for FfxivTarget {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::status::DebuffStatus;

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
        };

        let five_seconds_left_debuff = DebuffStatus {
            id: 2,
            duration_left_millisecond: 5000,
            status_data: StatusInfo::CritHitRatePercent(10),
            duration_millisecond: 10000,
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

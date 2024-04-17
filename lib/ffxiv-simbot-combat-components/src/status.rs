use crate::damage_rdps_profile::RdpsTable;
use crate::multiplier_calculator::{FfxivMultiplierCalculator, MultiplierCalculator};
use crate::owner_tracker::OwnerTracker;
use crate::player::Player;
use crate::simulator::{DpsSimulator, FfxivDpsSimulator};
use crate::skill::Skill;
use crate::target::Target;
use crate::{BuffIncreaseType, BuffTable, DamageType, IdType, TimeType};
use ffxiv_simbot_lib_db::stat_calculator::CharacterPower;
use ffxiv_simbot_lib_db::DamageMultiplierType;
use std::cell::{Ref, RefCell, RefMut};

#[derive(Copy, Clone, Debug)]
pub(crate) enum StatusInfo {
    DamagePercent(BuffIncreaseType),
    CritHitRatePercent(BuffIncreaseType),
    DirectHitRatePercent(BuffIncreaseType),
    SpeedPercent(BuffIncreaseType),
}

impl PartialEq<Self> for StatusInfo {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (StatusInfo::DamagePercent(a), StatusInfo::DamagePercent(b)) => a == b,
            (StatusInfo::CritHitRatePercent(a), StatusInfo::CritHitRatePercent(b)) => a == b,
            (StatusInfo::DirectHitRatePercent(a), StatusInfo::DirectHitRatePercent(b)) => a == b,
            (StatusInfo::SpeedPercent(a), StatusInfo::SpeedPercent(b)) => a == b,
            _ => false,
        }
    }
}

impl Eq for StatusInfo {}

/// Interface for player buffs and target debuffs
pub trait Status {
    fn get_id(&self) -> usize;
    /// in miliseconds
    fn get_duration_left_millisecond(&self) -> TimeType;
    fn set_duration_left_millisecond(&mut self, duration: TimeType);
    /// get the type of status and amount
    /// ex) Battle Litany: 10% Crit Buff = CritHitRatePercent(10)
    fn get_status_info(&self) -> StatusInfo;
    fn get_duration_millisecond(&self) -> TimeType;
    fn get_owner_id(&self) -> IdType;
    fn add_damage_contribution(&self, damage: usize);
    fn record_damage_contribution<T: Target, P: Player, S: Skill>(
        &self,
        simulator: FfxivDpsSimulator<T, P, S>,
        damage: DamageType,
    ) {
        let player = simulator.get_player(self.get_owner_id());
        player.update_buff_score(self.get_id(), damage);
    }
}

/// Implements entity that hold buff/debuff status
/// which are characters and attack targets.
pub trait StatusHolder<T: Status + Sized + Ord>: Sized {
    fn get_status_list(&self) -> Ref<Vec<T>>;
    fn get_status_list_mut(&self) -> RefMut<Vec<T>>;

    fn get_combat_time_millisecond(&self) -> TimeType;
    fn add_status(&self, status: T) {
        let mut status_list = self.get_status_list_mut();

        status_list.push(status);
    }

    fn get_status_multiplier(
        &self,
        character: &CharacterPower,
        ffxiv_multiplier_calculator: &FfxivMultiplierCalculator,
    ) -> RdpsTable {
        let status_list = self.get_status_list();
        let status_list: &Vec<T> = status_list.as_ref();
        let mut buff_table = BuffTable::new();

        let mut total_damage_multiplier = 1.0f64;

        for status in status_list {
            match status.get_status_info() {
                StatusInfo::CritHitRatePercent(rate) => {
                    let crit_hit_multiplier = ffxiv_multiplier_calculator
                        .calculate_crit_hit_rate_multiplier(character, rate);
                    total_damage_multiplier *= crit_hit_multiplier;
                    buff_table.insert(status.get_id(), crit_hit_multiplier);
                }
                StatusInfo::DirectHitRatePercent(rate) => {
                    let direct_hit_multiplier =
                        ffxiv_multiplier_calculator.calculate_direct_hit_rate_multiplier(rate);
                    total_damage_multiplier *= direct_hit_multiplier;
                    buff_table.insert(status.get_id(), direct_hit_multiplier);
                }
                StatusInfo::DamagePercent(rate) => {
                    let damage_increase =
                        ffxiv_multiplier_calculator.calculate_damage_multiplier(rate);
                    total_damage_multiplier *= damage_increase;
                    buff_table.insert(status.get_id(), damage_increase);
                }
                StatusInfo::SpeedPercent(_) => {}
            }
        }

        RdpsTable {
            total_damage_multiplier,
            buff_table,
        }
    }
}

/// Every time combat time updates,
/// Update the remaining time of buffs and debuffs and remove status that has expired.
pub trait StatusTimer<T: Status + Ord>: StatusHolder<T> {
    /// Update combat time by getting the time different and decreasing the
    /// time left on each buff and debuff.
    fn update_combat_time(&mut self, current_combat_time_millisecond: i32) {
        if self.get_combat_time_millisecond() >= current_combat_time_millisecond {
            return;
        }

        let time_diff = current_combat_time_millisecond - self.get_combat_time_millisecond();
        let mut buff_list = self.get_status_list_mut();

        for buff in buff_list.iter_mut() {
            buff.set_duration_left_millisecond(buff.get_duration_left_millisecond() - time_diff);
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct BuffStatus {
    pub(crate) id: IdType,
    pub(crate) duration_left_millisecond: TimeType,
    pub(crate) status_info: StatusInfo,
    pub(crate) duration_millisecond: TimeType,
    pub(crate) is_raidwide: bool,
    pub(crate) cumulative_damage: Option<RefCell<usize>>,
    pub(crate) owner_player_id: IdType,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct PersonalBuffStatus {
    pub(crate) id: IdType,
    pub(crate) duration_left_millisecond: TimeType,
    pub(crate) status_info: StatusInfo,
    pub(crate) duration_millisecond: TimeType,
    pub(crate) cumulative_damage: Option<RefCell<usize>>,
    pub(crate) owner_player_id: IdType,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct DebuffStatus {
    pub(crate) id: IdType,
    pub(crate) duration_left_millisecond: TimeType,
    pub(crate) status_info: StatusInfo,
    pub(crate) duration_millisecond: TimeType,
    pub(crate) cumulative_damage: Option<RefCell<usize>>,
    pub(crate) owner_player_id: IdType,
}

impl Status for BuffStatus {
    fn get_id(&self) -> IdType {
        self.id
    }

    fn get_duration_left_millisecond(&self) -> i32 {
        self.duration_left_millisecond
    }
    fn set_duration_left_millisecond(&mut self, duration: TimeType) {
        self.duration_left_millisecond = duration;
    }

    fn get_status_info(&self) -> StatusInfo {
        self.status_info
    }

    fn get_duration_millisecond(&self) -> i32 {
        self.duration_millisecond
    }
    fn add_damage_contribution(&self, damage: usize) {
        if let Some(cumulative_damage) = &self.cumulative_damage {
            *cumulative_damage.borrow_mut() += damage;
        }
    }
}

impl OwnerTracker for BuffStatus {
    fn get_owner_id(&self) -> IdType {
        self.owner_player_id
    }
}

impl OwnerTracker for DebuffStatus {
    fn get_owner_id(&self) -> IdType {
        self.owner_player_id
    }
}

impl Status for DebuffStatus {
    fn get_id(&self) -> IdType {
        self.id
    }

    fn get_duration_left_millisecond(&self) -> i32 {
        self.duration_left_millisecond
    }
    fn set_duration_left_millisecond(&mut self, duration: TimeType) {
        self.duration_left_millisecond = duration;
    }

    fn get_status_info(&self) -> StatusInfo {
        self.status_info
    }

    fn get_duration_millisecond(&self) -> TimeType {
        self.duration_millisecond
    }
    fn get_owner_id(&self) -> IdType {
        self.owner_player_id
    }

    fn add_damage_contribution(&self, damage: usize) {
        if let Some(cumulative_damage) = &self.cumulative_damage {
            *cumulative_damage.borrow_mut() += damage;
        }
    }
}

impl Status for PersonalBuffStatus {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_duration_left_millisecond(&self) -> TimeType {
        self.duration_left_millisecond
    }

    fn set_duration_left_millisecond(&mut self, duration: TimeType) {
        self.duration_left_millisecond = duration;
    }

    fn get_status_info(&self) -> StatusInfo {
        self.status_info
    }

    fn get_duration_millisecond(&self) -> TimeType {
        self.duration_millisecond
    }

    fn get_owner_id(&self) -> IdType {
        self.owner_player_id
    }

    fn add_damage_contribution(&self, damage: usize) {
        if let Some(cumulative_damage) = &self.cumulative_damage {
            *cumulative_damage.borrow_mut() += damage;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn buff_status_test() {
        let buff = BuffStatus {
            id: 1,
            duration_left_millisecond: 3000,
            status_info: StatusInfo::CritHitRatePercent(10),
            duration_millisecond: 15000,
            is_raidwide: false,
            cumulative_damage: None,
            owner_player_id: 0,
        };

        assert_eq!(buff.get_id(), 1);
        assert_eq!(buff.get_duration_left_millisecond(), 3000);
        assert_eq!(buff.get_status_info(), StatusInfo::CritHitRatePercent(10));
        assert_eq!(buff.get_duration_millisecond(), 15000);
    }

    fn debuff_status_test() {
        let debuff = DebuffStatus {
            id: 1,
            duration_left_millisecond: 3000,
            status_info: StatusInfo::CritHitRatePercent(10),
            duration_millisecond: 15000,
            cumulative_damage: None,
            owner_player_id: 0,
        };

        assert_eq!(debuff.get_id(), 1);
        assert_eq!(debuff.get_duration_left_millisecond(), 3000);
        assert_eq!(debuff.get_status_info(), StatusInfo::CritHitRatePercent(10));
        assert_eq!(debuff.get_duration_millisecond(), 15000);
    }
}

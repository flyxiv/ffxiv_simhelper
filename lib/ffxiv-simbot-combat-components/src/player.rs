use crate::skill::{AttackSkill, Skill, SkillInfo};
use crate::status::{BuffStatus, DebuffStatus, Status, StatusHolder, StatusInfo, StatusTimer};
use crate::target::Target;
use crate::{IdType, TimeType};

use crate::priority_table::{FfxivPriorityTable, PriorityTable};
use crate::turn_type::{PlayerTurn, TurnType};
use ffxiv_simbot_db::job::Job;
use ffxiv_simbot_db::stat_calculator::CharacterPower;
use std::cell::RefCell;
use std::rc::Rc;

/// If the delay is over 3 * OGCD delay, then it is turn to use a GCD skill,
/// Since in FFXIV a player can use at most 2 OGCD skills between GCD skills.
/// so 1 GCD delay + 2 oGCD delay = 3 * oGCD delay.

static MAX_MANA: i32 = 10000;

/// Saves information about the player: buffs, stat multipliers, jobs.
pub trait Player: Sized + StatusHolder<BuffStatus> {
    fn get_id(&self) -> IdType;
    fn get_job(&self) -> &Job;
    fn get_player_power(&self) -> &CharacterPower;
    fn get_delay(&self) -> TimeType;
    fn get_next_skill(&self, debuff_list: Rc<RefCell<Vec<DebuffStatus>>>)
        -> SkillInfo<AttackSkill>;

    fn set_delay(&mut self, delay: TimeType);
    fn has_resources_for_skill<S: Skill>(&self, skill: S) -> bool;
    fn get_next_gcd_time_millisecond(&self) -> TimeType;
    fn set_next_gcd_time_milliseconds(&mut self, next_gcd_time_millisecond: TimeType);
    fn get_next_turn_time_milliseconds(&self) -> TimeType;
}

/// The Abstraction for an actual FFXIV Player in the combat.
pub struct FfxivPlayer {
    /// Stat/Job Data about the player
    id: IdType,
    job: Job,
    power: CharacterPower,

    priority_table: FfxivPriorityTable<AttackSkill>,
    rotation_log: Vec<AttackSkill>,

    /// Realtime Combat Data about the player
    buff_list: Rc<RefCell<Vec<BuffStatus>>>,
    current_combo: Option<IdType>,
    /// How many seconds passed after the most recent GCD. If delay is close to GCD, an oGCD will
    /// clip the player's next GCD so it becomes a GCD turn.
    total_delay: TimeType,

    /// Combat time related data
    next_gcd_time_millisecond: TimeType,
    next_turn: PlayerTurn,

    mana_available: Option<i32>,
}

impl Player for FfxivPlayer {
    fn get_id(&self) -> IdType {
        self.id
    }
    fn get_job(&self) -> &Job {
        &self.job
    }

    fn get_player_power(&self) -> &CharacterPower {
        &self.power
    }

    fn get_delay(&self) -> TimeType {
        self.total_delay
    }

    fn set_delay(&mut self, delay: TimeType) {
        self.total_delay = delay;
    }

    fn get_next_skill(
        &self,
        debuff_list: Rc<RefCell<Vec<DebuffStatus>>>,
    ) -> SkillInfo<AttackSkill> {
        self.priority_table
            .get_next_skill(self.buff_list.clone(), debuff_list, self)
    }

    fn has_resources_for_skill<S: Skill>(&self, skill: S) -> bool {
        /// TODO: Implement mana resource check for casters.
        return false;
    }

    fn get_next_gcd_time_millisecond(&self) -> TimeType {
        self.next_gcd_time_millisecond
    }

    fn set_next_gcd_time_milliseconds(&mut self, gcd_delay: TimeType) {
        self.next_gcd_time_millisecond += gcd_delay;
    }

    fn get_next_turn_time_milliseconds(&self) -> TimeType {
        self.next_turn.next_turn_combat_time_millisecond
    }
}

impl FfxivPlayer {
    /// After using a turn, calculate when the next turn will be in combat time,
    /// and also figure out if it is a GCD/oGCD turn.
    pub fn calculate_next_turn(&mut self, current_combat_time_millisecond: TimeType) -> PlayerTurn {
        let current_turn = &self.next_turn;
        current_turn.get_next_turn(self, current_combat_time_millisecond)
    }
}

impl StatusHolder<BuffStatus> for FfxivPlayer {
    fn get_status_list(&self) -> Rc<RefCell<Vec<BuffStatus>>> {
        self.buff_list.clone()
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
            id: 0,
            job: Job::default(),
            power: CharacterPower::default(),
            buff_list: RefCell::new(vec![]),
            total_delay: RefCell::new(0),
            combat_time_millisecond: 0,
            buff_scores: RefCell::new(Default::default()),
            mana_available: RefCell::new(0),
        };

        let buff1 = BuffStatus {
            id: 1,
            duration_left_millisecond: 1000,
            status_data: StatusInfo::CritHitRatePercent(10),
            duration_millisecond: 1000,
            is_raidwide: false,
            cumulative_damage: None,
            owner_player_id: 0,
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
            id: 0,
            job: Job::default(),
            power: CharacterPower::default(),
            buff_list: RefCell::new(vec![]),
            total_delay: RefCell::new(0),
            combat_time_millisecond: 0,
            buff_scores: RefCell::new(Default::default()),
            mana_available: RefCell::new(0),
        };

        let two_seconds_left_buff = BuffStatus {
            id: 1,
            duration_left_millisecond: 2000,
            status_data: StatusInfo::CritHitRatePercent(10),
            duration_millisecond: 10000,
            is_raidwide: false,
            cumulative_damage: None,
            owner_player_id: 0,
        };

        let five_seconds_left_buff = BuffStatus {
            id: 2,
            duration_left_millisecond: 5000,
            status_data: StatusInfo::CritHitRatePercent(10),
            duration_millisecond: 10000,
            is_raidwide: true,
            cumulative_damage: None,
            owner_player_id: 0,
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

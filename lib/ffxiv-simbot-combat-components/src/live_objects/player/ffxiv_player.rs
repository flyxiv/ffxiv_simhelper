use crate::id_entity::IdEntity;
use crate::live_objects::player::gcd_calculator::GcdCalculator;
use crate::live_objects::player::Player;
use crate::live_objects::turn_type::{FfxivTurnType, PlayerTurn, TurnType};
use crate::rotation::cooldown_timer::CooldownTimer;
use crate::rotation::priority_table::{PriorityTable, SkillResult};
use crate::rotation::FfxivPriorityTable;
use crate::skill::attack_skill::AttackSkill;
use crate::skill::Skill;
use crate::status::buff_status::BuffStatus;
use crate::status::debuff_status::DebuffStatus;
use crate::status::status_holder::StatusHolder;
use crate::status::status_info::StatusInfo;
use crate::status::status_timer::StatusTimer;
use crate::status::Status;
use crate::{IdType, TimeType};
use ffxiv_simbot_db::job::Job;
use ffxiv_simbot_db::stat_calculator::CharacterPower;
use ffxiv_simbot_db::MultiplierType;
use std::cell::RefCell;
use std::rc::Rc;

static TWO_MINUTES_IN_MILLISECOND: TimeType = 120000;
static BURST_START_TIME_MILLISECOND: TimeType = 7000;
static BURST_END_TIME_MILLISECOND: TimeType = 23000;

/// The Abstraction for an actual FFXIV Player in the combat.
pub struct FfxivPlayer {
    /// Stat/Job Data about the player
    pub id: IdType,
    pub job: Job,
    pub power: CharacterPower,

    pub priority_table: RefCell<FfxivPriorityTable>,

    /// Realtime Combat Data about the player
    pub buff_list: Rc<RefCell<Vec<BuffStatus>>>,
    /// How many seconds passed after the most recent GCD. If delay is close to GCD, an oGCD will
    /// clip the player's next GCD so it becomes a GCD turn.
    pub total_delay: TimeType,

    /// Combat time related data
    pub next_gcd_time_millisecond: TimeType,
    pub last_gcd_time_millisecond: TimeType,
    pub next_turn: PlayerTurn,

    pub mana_available: Option<i32>,
}

impl Player for FfxivPlayer {
    fn get_job(&self) -> &Job {
        &self.job
    }

    fn get_player_power(&self) -> &CharacterPower {
        &self.power
    }

    fn get_delay(&self) -> TimeType {
        self.total_delay
    }

    fn get_next_skill(
        &self,
        debuff_list: Rc<RefCell<Vec<DebuffStatus>>>,
    ) -> Option<SkillResult<AttackSkill>> {
        self.priority_table
            .borrow_mut()
            .get_next_skill(self.buff_list.clone(), debuff_list, self)
    }

    fn set_delay(&mut self, delay: TimeType) {
        self.total_delay = delay;
    }

    fn get_damage_inflict_time_millisecond<S: Skill>(&self, skill: &S) -> Option<TimeType> {
        let inflict_time = skill.get_charging_time_millisecond() + self.get_cast_time(skill);

        if inflict_time == 0 {
            None
        } else {
            Some(inflict_time)
        }
    }

    fn has_resources_for_skill<S: Skill>(&self, _: S) -> bool {
        /// TODO: Implement mana resource check for casters.
        return false;
    }

    fn get_last_gcd_time_millisecond(&self) -> TimeType {
        self.last_gcd_time_millisecond
    }

    fn set_last_gcd_time_millisecond(&mut self, time: TimeType) {
        self.last_gcd_time_millisecond = time;
    }

    fn get_next_gcd_time_millisecond(&self) -> TimeType {
        self.next_gcd_time_millisecond
    }

    fn set_next_gcd_time_millisecond<S: Skill>(&mut self, skill: &S) {
        let gcd_delay = self.get_gcd(skill);
        self.next_gcd_time_millisecond += gcd_delay;
    }

    fn get_next_turn_time_millisecond(&self) -> TimeType {
        self.next_turn.next_turn_combat_time_millisecond
    }

    fn get_gcd_delay_millisecond<S: Skill>(&self, skill: &S) -> TimeType {
        let gcd_cooldown_millisecond = skill.get_gcd_cooldown_millsecond();

        if skill.is_speed_buffed() {
            self.get_speed_buffed_time(gcd_cooldown_millisecond)
        } else {
            gcd_cooldown_millisecond
        }
    }

    fn get_turn(&self) -> &FfxivTurnType {
        &self.next_turn.turn_type
    }

    fn get_turn_type(&self) -> &FfxivTurnType {
        &self.next_turn.turn_type
    }
    fn get_millisecond_before_burst(&self) -> TimeType {
        let combat_time = self.next_turn.next_turn_combat_time_millisecond;
        self.get_next_burst_time(combat_time)
    }

    fn delay_turn_by(&mut self, delay: TimeType) {
        self.next_turn.next_turn_combat_time_millisecond += delay;
    }
}

impl IdEntity for FfxivPlayer {
    fn get_id(&self) -> IdType {
        self.id
    }
}

impl CooldownTimer for FfxivPlayer {
    fn update_cooldown(&mut self, time_passed: TimeType) {
        self.priority_table
            .borrow_mut()
            .update_cooldown(time_passed);
    }
}

impl FfxivPlayer {
    fn get_next_burst_time(&self, combat_time: TimeType) -> TimeType {
        let burst_number_offset = combat_time % TWO_MINUTES_IN_MILLISECOND;

        if burst_number_offset < BURST_START_TIME_MILLISECOND {
            BURST_START_TIME_MILLISECOND - burst_number_offset
        } else if burst_number_offset > BURST_END_TIME_MILLISECOND {
            TWO_MINUTES_IN_MILLISECOND + BURST_START_TIME_MILLISECOND - burst_number_offset
        } else {
            0
        }
    }

    fn get_gcd<S: Skill>(&self, skill: &S) -> TimeType {
        let mut gcd_cooldown_millisecond = skill.get_gcd_cooldown_millsecond();

        let charging_time = skill.get_charging_time_millisecond();

        if skill.is_speed_buffed() {
            gcd_cooldown_millisecond = self.get_speed_buffed_time(gcd_cooldown_millisecond)
        }

        charging_time + gcd_cooldown_millisecond
    }

    fn get_cast_time<S>(&self, skill: &S) -> TimeType
    where
        S: Skill,
    {
        let cast_time = skill.get_gcd_cast_time();

        if skill.is_speed_buffed() {
            self.get_speed_buffed_time(cast_time)
        } else {
            cast_time
        }
    }

    fn get_speed_buffed_time(&self, time_millisecond: TimeType) -> TimeType {
        let speed_buff_multiplier = self.get_gcd_buff_multiplier();

        self.calculate_gcd_millisecond(
            time_millisecond,
            self.get_player_power().speed_multiplier,
            speed_buff_multiplier,
        )
    }

    fn get_gcd_buff_multiplier(&self) -> MultiplierType {
        let mut gcd_buffs_multiplier = 1.0;
        for buff in self.buff_list.borrow().iter() {
            match buff.status_info {
                StatusInfo::SpeedPercent(buff_increase_percent) => {
                    gcd_buffs_multiplier =
                        gcd_buffs_multiplier * (buff_increase_percent as MultiplierType / 100.0);
                }
                _ => {}
            }
        }
        gcd_buffs_multiplier
    }

    /// After using a turn, calculate when the next turn will be in combat time,
    /// and also figure out if it is a GCD/oGCD turn.
    pub fn calculate_next_turn(&mut self, skill_delay: TimeType) {
        let current_turn = &self.next_turn;
        self.next_turn = current_turn.get_next_turn(
            self,
            skill_delay,
            current_turn.next_turn_combat_time_millisecond,
        )
    }

    pub fn new(
        id: IdType,
        job: Job,
        power: CharacterPower,
        priority_table: FfxivPriorityTable,
    ) -> FfxivPlayer {
        FfxivPlayer {
            id,
            job,
            power,
            priority_table: RefCell::new(priority_table),
            buff_list: Rc::new(RefCell::new(vec![])),
            total_delay: 0,
            next_gcd_time_millisecond: 0,
            last_gcd_time_millisecond: 0,
            next_turn: PlayerTurn::default(),
            mana_available: None,
        }
    }
}

impl GcdCalculator for FfxivPlayer {}

impl StatusHolder<BuffStatus> for FfxivPlayer {
    fn get_status_list(&self) -> Rc<RefCell<Vec<BuffStatus>>> {
        self.buff_list.clone()
    }
}

impl StatusTimer<BuffStatus> for FfxivPlayer {
    fn update_status_time(&mut self, elapsed_time: TimeType) {
        if elapsed_time <= 0 {
            return;
        }

        let status_list = self.get_status_list();

        for status in status_list.borrow_mut().iter_mut() {
            status.set_duration_left_millisecond(
                status.get_duration_left_millisecond() - elapsed_time,
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::skill::status::{Status, StatusInfo};

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

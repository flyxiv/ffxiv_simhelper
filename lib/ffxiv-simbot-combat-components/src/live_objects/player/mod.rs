use crate::id_entity::IdEntity;
use crate::live_objects::turn_type::FfxivTurnType;
use crate::skill::attack_skill::{AttackSkill, SkillInfo};
use crate::skill::Skill;
use crate::status::buff_status::BuffStatus;
use crate::status::debuff_status::DebuffStatus;
use crate::status::status_holder::StatusHolder;
use crate::TimeType;
use ffxiv_simbot_db::job::Job;
use ffxiv_simbot_db::stat_calculator::CharacterPower;
use std::cell::RefCell;
use std::rc::Rc;

/// If the delay is over 3 * OGCD delay, then it is turn to use a GCD skill,
/// Since in FFXIV a player can use at most 2 OGCD skills between GCD skills.
/// so 1 GCD delay + 2 oGCD delay = 3 * oGCD delay.
pub mod ffxiv_player;
pub mod gcd_calculator;

static MAX_MANA: i32 = 10000;

/// Saves information about the player: buffs, stat multipliers, rotation.
pub trait Player: Sized + StatusHolder<BuffStatus> + IdEntity {
    fn get_job(&self) -> &Job;
    fn get_player_power(&self) -> &CharacterPower;
    fn get_delay(&self) -> TimeType;
    fn get_next_skill(
        &self,
        debuff_list: Rc<RefCell<Vec<DebuffStatus>>>,
    ) -> Option<SkillInfo<AttackSkill>>;

    fn get_last_gcd_time_millisecond(&self) -> TimeType;
    fn set_delay(&mut self, delay: TimeType);

    fn get_damage_inflict_time_millisecond<S: Skill>(&self, skill: &S) -> Option<TimeType>;
    fn has_resources_for_skill<S: Skill>(&self, skill: S) -> bool;
    fn get_next_gcd_time_millisecond(&self) -> TimeType;
    fn set_next_gcd_time_millisecond<S: Skill>(&mut self, skill: &S);
    fn get_next_turn_time_millisecond(&self) -> TimeType;
    fn get_gcd_delay_millisecond<S: Skill>(&self, skill: &S) -> TimeType;
    fn get_turn(&self) -> &FfxivTurnType;
    fn get_turn_type(&self) -> &FfxivTurnType;
    fn get_millisecond_before_burst(&self) -> TimeType;
    fn set_last_gcd_time_millisecond(&mut self, time: TimeType);
    fn delay_turn_by(&mut self, delay: TimeType);
}

pub mod attack_skill;

use crate::id_entity::IdEntity;
use crate::live_objects::player::gcd_calculator::GcdCalculator;
use crate::status::status_holder::StatusHolder;
use crate::{DamageType, IdType, TimeType};
use ffxiv_simbot_db::MultiplierType;

pub static GCD_TURN_DELAY_PERCENTAGE_THRESHOLD: MultiplierType = 0.7;
/// The normal delay time for o-GCD skills.
/// After using 1 oGCD, the player cannot use another skill for 0.7 seconds.
pub static NON_GCD_DELAY_MILLISECOND: i32 = 700;

/// The resource requirements for a skill.
/// Skill might need mana, status(suiton status is needed for Trick Attack), or combo status.
#[derive(Clone)]
pub(crate) enum ResourceRequirements {
    Mana(i32),
    Status(i32),
    PreviousCombo(IdType),
}

pub trait Skill: Sized + Clone + IdEntity {
    fn get_potency(&self) -> DamageType;
    fn get_cooldown_millisecond(&self) -> TimeType;
    fn get_charging_time_millisecond(&self) -> TimeType;
    fn get_delay_millisecond(&self) -> TimeType;
    fn is_gcd(&self) -> bool;
    fn get_gcd_cast_time(&self) -> TimeType;
    fn get_gcd_time_millisecond(&self) -> TimeType;
    fn get_gcd_cooldown_millsecond(&self) -> TimeType;

    fn start_cooldown(&mut self);
    fn is_ready(&self) -> bool;
}

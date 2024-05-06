pub mod attack_skill;

use crate::id_entity::IdEntity;
use crate::{DamageType, IdType, ResourceType, StackType, TimeType};
use ffxiv_simbot_db::MultiplierType;

pub static GCD_TURN_DELAY_PERCENTAGE_THRESHOLD: MultiplierType = 0.65;
/// The normal delay time for o-GCD skills.
/// After using 1 oGCD, the player cannot use another skill for 0.7 seconds.
pub static NON_GCD_DELAY_MILLISECOND: i32 = 700;

/// The resource requirements for a skill.
/// Skill might need mana, status(suiton status is needed for Trick Attack), or combo status.
#[derive(Clone)]
pub(crate) enum ResourceRequirements {
    Mana(ResourceType),
    UseStatus(IdType),
    CheckStatus(IdType),
    StackResource1(ResourceType),
    StackResource2(ResourceType),
}

pub trait Skill: Sized + Clone + IdEntity {
    fn get_potency(&self) -> DamageType;
    fn get_cooldown_millisecond(&self) -> TimeType;
    fn get_charging_time_millisecond(&self) -> TimeType;
    fn get_delay_millisecond(&self) -> TimeType;
    fn is_gcd(&self) -> bool;
    fn get_gcd_cast_time(&self) -> TimeType;
    fn get_gcd_time_millisecond(&self) -> TimeType;
    fn get_current_cooldown_millisecond(&self) -> TimeType;
    fn get_gcd_cooldown_millsecond(&self) -> TimeType;

    fn start_cooldown(&mut self);
    fn is_ready(&self) -> bool;
    fn is_raidbuff(&self) -> bool;
    fn is_speed_buffed(&self) -> bool;
    fn stack_skill_id(&self) -> IdType;
    fn is_auto_attack(&self) -> bool;
    fn get_name(&self) -> &String;
    fn get_stacks(&self) -> StackType;

    fn get_resource1_created(&self) -> ResourceType;
    fn get_resource2_created(&self) -> ResourceType;
    fn get_combo(&self) -> Option<IdType>;
    fn get_resource_required(&self) -> &Vec<ResourceRequirements>;
}

use crate::player::Player;
use crate::skill::Skill;
use crate::target::Target;
use ffxiv_simbot_lib_db::job::Job;
use ffxiv_simbot_lib_db::stat_calculator::CharacterPower;

/// Simulate DPS for a job based on Priority System
pub trait DpsSimulator<T: Target, P: Player, S: Skill> {
    fn get_next_ability(&self) -> S;
    fn get_player(&self) -> P;
    fn get_target(&self) -> T;
    fn calculate_potency(&self, character: CharacterPower) -> i32;
    fn calculate_dps(&self, character: CharacterPower) -> f64;
}

pub struct FfxivDpsSimulator<T> {
    job: Job,
    priority_system: Vec<T>,
}

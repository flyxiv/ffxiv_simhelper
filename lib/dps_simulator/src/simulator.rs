use ffxiv_simbot_lib_db::stat_calculator::CharacterPower;

/// Simulate DPS for a job based on Priority System
pub trait DpsSimulator {
    fn get_next_ability(&self) -> Skill;
    fn calculate_potency(&self, character: CharacterPower) -> i32;
    fn calculate_dps(&self, character: CharacterPower) -> f64;
}

pub struct FfxivDpsSimulator {
    job: Job,
    priority_system: Vec<Skill>,
}

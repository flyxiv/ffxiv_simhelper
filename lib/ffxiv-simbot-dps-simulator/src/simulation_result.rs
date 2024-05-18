use ffxiv_simbot_combat_components::{DamageType, DpsType, IdType, TimeType};
use std::collections::HashMap;

/// Shows the result of the simulation.
/// Contains:
/// 1. RDPS profile of each player in the party. (party member, RDPS, contributed DPS)
/// 2. Skill usage logs of each player (time the skill was used(ms) + skill name)

pub type SimulationResponse = HashMap<IdType, SimulationResult>;

pub struct SimulationResult {
    pub job_name: String,
    pub raw_damage_total: DamageType,
    /// Rdps the job's buff skills earned.
    pub rdps_earned: DpsType,
    /// The job's Rdps contribution to the other party member's buffs
    pub rdps_contributed: DpsType,
    pub rotation_log: Vec<RotationLog>,
}

#[derive(Clone)]
pub struct RotationLog {
    pub casted_time_millisecond: TimeType,
    pub skill_id: IdType,
}

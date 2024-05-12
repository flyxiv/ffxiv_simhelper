use crate::damage_calculator::damage_rdps_profile::RaidDamageTable;
use crate::simulator::FfxivSimulationBoard;
use ffxiv_simbot_combat_components::id_entity::IdEntity;
use ffxiv_simbot_combat_components::live_objects::player::Player;
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

impl From<FfxivSimulationBoard> for SimulationResponse {
    fn from(simulation_board: FfxivSimulationBoard) -> Self {
        let player_ids = simulation_board
            .party
            .iter()
            .map(|player| player.borrow().get_id())
            .collect::<Vec<IdType>>();

        let mut result = HashMap::new();

        for player_id in player_ids {
            let job_name = simulation_board.party[player_id]
                .borrow()
                .get_job()
                .name
                .clone();

            let raw_damage_total = simulation_board.damage_profiles[player_id]
                .borrow()
                .values()
                .sum();
            let rdps_earned = simulation_board
                .rdps_table
                .borrow()
                .get_rdps_earned(player_id) as DpsType
                / simulation_board.finish_combat_time_millisecond as DpsType;

            let rotation_log = simulation_board
                .rotation_logs
                .borrow()
                .get(&player_id)
                .unwrap()
                .clone();

            let rdps_contributed = simulation_board
                .rdps_table
                .borrow()
                .get_rdps_contribution(player_id) as DpsType
                / simulation_board.finish_combat_time_millisecond as DpsType;

            result.insert(
                player_id,
                SimulationResult {
                    job_name,
                    raw_damage_total,
                    rdps_earned,
                    rdps_contributed,
                    rotation_log,
                },
            );
        }

        result
    }
}

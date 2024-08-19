use crate::response::CountType;
use ffxiv_simbot_combat_components::live_objects::player::player_power::PlayerPower;
use ffxiv_simbot_combat_components::types::{DpsType, IdType, PlayerIdType, TimeType};
use serde::Serialize;

pub(crate) const SKILL_ENTITY_STRING: &'static str = "Skill";
pub(crate) const STATUS_ENTITY_STRING: &'static str = "Status";

/// API Response Format for quicksim/advancedsim API
/// Given as a GraphQL response
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SimulationApiResponse {
    pub main_player_id: PlayerIdType,
    pub combat_time_millisecond: TimeType,
    pub simulation_data: Vec<SimulationDataResponse>,
    pub main_player_power: PlayerPower,
    pub main_player_job_abbrev: String,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SimulationDataResponse {
    pub player_id: PlayerIdType,
    pub job_abbrev: String,
    pub role: String,
    pub simulation_summary: SimulationSummaryResponse,
    pub party_contribution_table: Vec<PartyContributionResponse>,
    pub damage_profile_table: Vec<DamageProfileResponse>,
    pub rotation_log: Vec<SkillLogResponse>,
}

/// Aggregates all the needed DPS summary data
/// ## Terms described in the member equation:
/// 1) Given Contribution : Total sum of all my contribution to party member's buffs.
/// 2) Received Contribution : Total sum of all party member's contribution to my buffs.
/// 3) Raw Damage : Raw damage that would be dealt if there wasn't any buffs.
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SimulationSummaryResponse {
    /// Raw Damage + Received Contribution
    pub rdps: DpsType,
    /// Raw Damage + Given Contribution - Received Contribution
    pub adps: DpsType,
    /// Raw Damage + Given Contribution
    /// Which is equal to the actual DPS that is shown in the damage meters.
    pub pdps: DpsType,
    /// Effective Dps
    /// The best metric for comparing how useful DPS performance was overall.
    /// raw damage + Given Contribution + Received Contribution
    pub edps: DpsType,
}

/// Records the damage contribution of each skill
/// to each player's buff.
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PartyContributionResponse {
    pub skill_id: IdType,
    pub party_member_id: PlayerIdType,
    pub status_id: IdType,
    pub contributed_rdps: DpsType,
}

/// Records the rdps/pdps contribution of each skill
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DamageProfileResponse {
    pub id: IdType,
    /// Skill or Status
    pub entity: String,
    /// Sum of all raw damage
    pub rdps_contribution: DpsType,
    /// Sum of all raw damage + rdps contribution
    pub pdps_contribution: DpsType,
    pub cast_count: CountType,
}

/// Records the rotation log of each player
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SkillLogResponse {
    pub time: TimeType,
    pub skill_id: IdType,
    pub target: Option<PlayerIdType>,
    pub buffs: Vec<IdType>,
    pub debuffs: Vec<IdType>,
}

use crate::response::CountType;
use ffxiv_simhelper_combat_components::live_objects::player::player_power::PlayerPower;
use ffxiv_simhelper_combat_components::types::{DpsType, PlayerIdType, SkillIdType, TimeType};
use serde::Serialize;

pub(crate) const SKILL_ENTITY_STRING: &'static str = "Skill";
pub(crate) const STATUS_ENTITY_STRING: &'static str = "Status";

/// API Response Format for dpsanalysis API
/// Gives statistics and needed data for analyzing the result of the simulation.
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SimulationApiResponse {
    /// same as the main_player_id in the SimulationApiRequest.
    pub main_player_id: PlayerIdType,

    /// same as the main_player_name in the SimulationApiRequest.
    pub combat_time_millisecond: TimeType,

    /// contains the simulation result data of each player in the party.
    /// simulation_data[player_id] contains the simulation result of player_id.
    pub simulation_data: Vec<SimulationDataResponse>,

    /// same as the main_player_power in the SimulationApiRequest.
    pub main_player_power: PlayerPower,

    /// 3-letter abbreviation of the main player's combat job
    pub main_player_job_abbrev: String,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SimulationDataResponse {
    pub player_id: PlayerIdType,
    pub job_abbrev: String,
    pub role: String,
    pub simulation_summary: SimulationSummaryResponse,

    /// Records the total damage contribution of player to another party member's buffs.
    pub party_contribution_table: Vec<PartyContributionResponse>,

    /// Records total damage profile of each skill and status of the player.
    /// User for Damage Profiles Table in the DPS Analysis Results Tab.
    pub damage_profile_table: Vec<DamageProfileResponse>,

    /// Records the total damage contribution of player_id to a party member in a burst window.
    /// Only used in BestPartner API.
    pub party_burst_contribution_table: Vec<PartyBurstContributionResponse>,

    /// Records the rotation log of the player, showing which skill was used at what time.
    /// User for Rotation Log Table in the DPS Analysis Results Tab.
    pub rotation_log: Vec<SkillLogResponse>,
}

/// Aggregates all the needed DPS summary data
/// ## Terms described in the member equation:
/// 1) Buffs Given: Total sum of all my contribution to party member's buffs.
/// 2) Buffs Taken: Total sum of all party member's contribution to my buffs.
/// 3) Raw Damage : Raw damage that would be dealt if there wasn't any buffs.
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SimulationSummaryResponse {
    /// Raw Damage + Buffs Taken
    pub rdps: Vec<DpsType>,

    /// Not implemented yet
    pub adps: Vec<DpsType>,

    /// Raw Damage + Buffs Given
    /// Which is equal to the actual DPS that is shown in the damage meters.
    pub pdps: Vec<DpsType>,

    /// Effective Dps
    /// The best metric for comparing how useful DPS performance was overall.
    /// Raw Damage + Buffs Taken + Buffs Given
    pub edps: Vec<DpsType>,
}

impl SimulationSummaryResponse {
    pub(crate) fn extend(&mut self, other: &Self) {
        self.rdps.extend(other.rdps.iter().cloned());
        self.adps.extend(other.adps.iter().cloned());
        self.pdps.extend(other.pdps.iter().cloned());
        self.edps.extend(other.edps.iter().cloned());
    }
}

/// Records the damage contribution of each skill
/// to each player's buff.
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PartyContributionResponse {
    pub skill_id: SkillIdType,
    pub party_member_id: PlayerIdType,
    pub status_id: SkillIdType,
    pub contributed_damage: i32,
}

/// Records the total damage contribution of player_id to a party member's in a burst window.
/// Only used in BestPartner API.
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PartyBurstContributionResponse {
    /// The minute of the burst window.
    /// Since FFXIV only has 2 min raidbuffs right now, must be even number and 0 stands for opener.
    pub minute: TimeType,
    pub party_member_id: PlayerIdType,
    pub contributed_damage: i32,
}

/// Records the rdps/pdps contribution of each skill
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DamageProfileResponse {
    pub id: SkillIdType,
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
    pub skill_id: SkillIdType,
    pub target: Option<PlayerIdType>,
    pub buffs: Vec<SkillIdType>,
    pub debuffs: Vec<SkillIdType>,
}

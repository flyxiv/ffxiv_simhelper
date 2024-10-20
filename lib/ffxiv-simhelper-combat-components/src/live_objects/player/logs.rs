/// Records skill use events and damage events, stored for each player.
/// This is used as raw data to create simulation summary.
use crate::damage_calculator::DamageRdpsProfile;
use crate::live_objects::player::StatusKey;
use crate::types::{MultiplierType, PlayerIdType, SkillIdType, TimeType};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct SkillLog {
    pub time: TimeType,
    pub skill_id: SkillIdType,
    pub target_id: Option<PlayerIdType>,

    /// Records important buffs that the player had at the time of skill usage.
    pub buffs: Vec<SkillIdType>,

    /// Records important debuffs that the player had at the time of skill usage.
    pub debuffs: Vec<SkillIdType>,
}

#[derive(Debug, Clone)]
pub struct DamageLog {
    pub time: TimeType,
    pub skill_id: SkillIdType,
    pub raw_damage: MultiplierType,

    /// Records how much the skill contributed to each raidbuff
    pub rdps_contribution: Vec<RdpsContribution>,
}

#[derive(Debug, Clone)]
pub struct RdpsContribution {
    pub player_id: PlayerIdType,
    pub raid_buff_status_id: SkillIdType,
    pub contributed_damage: MultiplierType,
}

#[inline]
fn convert_to_rdps_contribution(
    table: &HashMap<StatusKey, MultiplierType>,
) -> Vec<RdpsContribution> {
    table
        .iter()
        .filter(|(_, &damage)| damage > 0.0)
        .map(|(key, damage)| RdpsContribution {
            player_id: key.player_id,
            raid_buff_status_id: key.status_id,
            contributed_damage: *damage,
        })
        .collect()
}

impl DamageLog {
    pub fn new(
        skill_id: SkillIdType,
        damage_rdps_profile: &DamageRdpsProfile,
        time: TimeType,
    ) -> DamageLog {
        DamageLog {
            time,
            skill_id,
            raw_damage: damage_rdps_profile.raw_damage,
            rdps_contribution: convert_to_rdps_contribution(&damage_rdps_profile.rdps_contribution),
        }
    }
}

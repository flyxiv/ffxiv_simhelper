use crate::damage_calculator::DamageRdpsProfile;
use crate::live_objects::player::StatusKey;
use crate::types::{IdType, MultiplierType, TimeType};
use std::collections::HashMap;

/// Records skill use events and damage events, stored for each player.
/// This is used as raw data to create simulation summary.
#[derive(Debug, Clone)]
pub struct SkillLog {
    pub time: TimeType,
    pub skill_id: IdType,
    pub target_id: Option<IdType>,
    pub buffs: Vec<IdType>,
    pub debuffs: Vec<IdType>,
}

#[derive(Debug, Clone)]
pub struct DamageLog {
    pub time: TimeType,
    pub skill_id: IdType,
    pub raw_damage: MultiplierType,
    pub rdps_contribution: Vec<RdpsContribution>,
}

#[derive(Debug, Clone)]
pub struct RdpsContribution {
    pub player_id: IdType,
    pub raid_buff_status_id: IdType,
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
        skill_id: IdType,
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

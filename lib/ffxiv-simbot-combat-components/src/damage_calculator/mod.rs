use crate::live_objects::player::StatusKey;
use crate::{DamageType, IdType};
use std::collections::HashMap;

mod contribution_share;
pub mod multiplier_calculator;
pub mod raw_damage_calculator;
pub mod rdps_calculator;

pub type FfxivRaidDamageTable = HashMap<StatusKey, DamageType>;

/// Implements FFXIV's damage calculation logic based on the
/// player's stat multiplier + skill's potency + guaranteed critical hit/direct hit buff.
pub struct DamageRdpsProfile {
    pub skill_id: IdType,
    pub raw_damage: DamageType,
    pub final_damage: DamageType,
    pub rdps_contribution: FfxivRaidDamageTable,
}

/// Stores all the needed keys to query the damage contribution of a specific status.
/// skill_id: needed to organize how much each skill contributed to the total rdps of the status.
/// player_id: needed to identify the owner of the status, to figure out which player's table the
///            skill's damage goes to
/// owner_id: needed to identify the owner of the status, to figure out which player's table the
///            status' damage goes to
#[derive(PartialEq, Eq, Hash, Clone)]
pub struct RaidDamageTableKey {
    pub(crate) status_id: IdType,
    pub(crate) owner_id: IdType,
    pub(crate) skill_id: IdType,
}

impl RaidDamageTableKey {
    pub fn new(status_id: IdType, owner_id: IdType, skill_id: IdType) -> Self {
        RaidDamageTableKey {
            status_id,
            owner_id,
            skill_id,
        }
    }
}

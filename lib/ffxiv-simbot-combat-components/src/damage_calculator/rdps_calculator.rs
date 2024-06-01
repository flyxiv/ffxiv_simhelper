use crate::damage_calculator::damage_rdps_profile::{FfxivRaidDamageTable, RaidDamageTable};
use crate::damage_calculator::multiplier_calculator::MultiplierCalculator;
use crate::damage_calculator::{DamageRdpsProfile, RaidDamageTableKey};
use crate::id_entity::IdEntity;
use crate::live_objects::player::StatusKey;
use crate::owner_tracker::OwnerTracker;
use crate::status::buff_status::BuffStatus;
use crate::status::debuff_status::DebuffStatus;
use crate::{DamageType, IdType};
use ffxiv_simbot_db::stat_calculator::CharacterPower;
use ffxiv_simbot_db::MultiplierType;
use std::collections::HashMap;

pub trait RdpsCalculator {
    /// Given the raw damage and all the list of buffs/debuffs on the player and the target,
    /// 1) Convert the buffs to a damage multiplier.
    /// 2) Calculate the RDPS contribution of each buff
    /// 3) Order each buff to update its RDPS contribution.
    fn make_damage_profile(
        &self,
        skill_id: IdType,
        snapshotted_buffs: HashMap<StatusKey, BuffStatus>,
        snapshotted_debuffs: HashMap<StatusKey, DebuffStatus>,
        skill_damage: DamageType,
        power: &CharacterPower,
        player_id: IdType,
    ) -> DamageRdpsProfile;
}

fn apply_multiplier(damage: DamageType, multiplier: MultiplierType) -> DamageType {
    f64::floor(damage as MultiplierType * multiplier) as DamageType
}

pub struct FfxivRdpsCalculator {}

impl MultiplierCalculator for FfxivRdpsCalculator {}

impl RdpsCalculator for FfxivRdpsCalculator {
    fn make_damage_profile(
        &self,
        skill_id: IdType,
        snapshotted_buffs: HashMap<StatusKey, BuffStatus>,
        snapshotted_debuffs: HashMap<StatusKey, DebuffStatus>,
        skill_damage: DamageType,
        power: &CharacterPower,
        _: IdType,
    ) -> DamageRdpsProfile
where {
        let mut damage_profile = DamageRdpsProfile {
            raw_damage: skill_damage,
            final_damage: skill_damage,
            status_rdps_contribution: FfxivRaidDamageTable {
                rdps_table: Default::default(),
            },
        };

        for buff in snapshotted_buffs.values() {
            let buff_id = buff.get_id();

            let raid_damage_profile_key = RaidDamageTableKey {
                skill_id,
                status_id: buff_id,
                owner_id: buff.get_owner_id(),
            };

            let damage_multiplier = self.calculate_multiplier(buff, power);
            let contribution = apply_multiplier(skill_damage, damage_multiplier) - skill_damage;

            damage_profile.final_damage =
                apply_multiplier(damage_profile.final_damage, damage_multiplier);
            damage_profile
                .status_rdps_contribution
                .insert(raid_damage_profile_key, contribution);
        }

        for debuff in snapshotted_debuffs.values() {
            let buff_id = debuff.get_id();

            let raid_damage_profile_key = RaidDamageTableKey {
                skill_id,
                status_id: buff_id,
                owner_id: debuff.get_owner_id(),
            };

            let damage_multiplier = self.calculate_multiplier(debuff, power);
            let contribution = apply_multiplier(skill_damage, damage_multiplier) - skill_damage;

            damage_profile.final_damage =
                apply_multiplier(damage_profile.final_damage, damage_multiplier);
            damage_profile
                .status_rdps_contribution
                .insert(raid_damage_profile_key, contribution);
        }

        damage_profile
    }
}

impl Default for FfxivRdpsCalculator {
    fn default() -> Self {
        FfxivRdpsCalculator {}
    }
}

use crate::damage_calculator::multiplier_calculator::MultiplierCalculator;
use crate::damage_calculator::DamageRdpsProfile;
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
        skill_damage: MultiplierType,
        power: &CharacterPower,
        player_id: IdType,
    ) -> DamageRdpsProfile;
}

fn apply_multiplier(damage: MultiplierType, multiplier: MultiplierType) -> MultiplierType {
    f64::floor(damage * multiplier) as MultiplierType
}

pub struct FfxivRdpsCalculator {}

impl MultiplierCalculator for FfxivRdpsCalculator {}

impl RdpsCalculator for FfxivRdpsCalculator {
    fn make_damage_profile(
        &self,
        skill_id: IdType,
        snapshotted_buffs: HashMap<StatusKey, BuffStatus>,
        snapshotted_debuffs: HashMap<StatusKey, DebuffStatus>,
        skill_damage: MultiplierType,
        power: &CharacterPower,
        _: IdType,
    ) -> DamageRdpsProfile {
        let mut damage_profile = DamageRdpsProfile {
            skill_id,
            raw_damage: skill_damage,
            final_damage: skill_damage,
            rdps_contribution: HashMap::new(),
        };

        let mut total_multiplier = 1.0;

        for buff in snapshotted_buffs.values() {
            let buff_id = buff.get_id();

            let status_key = StatusKey::new(buff_id, buff.get_owner_id());
            let damage_multiplier = self.calculate_multiplier(buff, power);

            if damage_multiplier == 1.0 {
                continue;
            }

            total_multiplier *= damage_multiplier;

            let mut entry = damage_profile
                .rdps_contribution
                .entry(status_key)
                .or_insert(1.0);
            *entry *= damage_multiplier;

            damage_profile.final_damage =
                apply_multiplier(damage_profile.final_damage, damage_multiplier);
        }

        for debuff in snapshotted_debuffs.values() {
            let status_id = debuff.get_id();
            let status_key = StatusKey::new(status_id, debuff.get_owner_id());

            let damage_multiplier = self.calculate_multiplier(debuff, power);

            if damage_multiplier == 1.0 {
                continue;
            }

            total_multiplier *= damage_multiplier;

            let mut entry = damage_profile
                .rdps_contribution
                .entry(status_key)
                .or_insert(1.0);
            *entry *= damage_multiplier;

            damage_profile.final_damage =
                apply_multiplier(damage_profile.final_damage, damage_multiplier);
        }

        let buffed_damage =
            (damage_profile.final_damage - damage_profile.raw_damage) as MultiplierType;
        let total_multiplier_log = MultiplierType::log10(total_multiplier);

        for (_, rdps) in damage_profile.rdps_contribution.iter_mut() {
            *rdps = buffed_damage * (MultiplierType::log10(*rdps) / total_multiplier_log);
        }

        damage_profile
    }
}

impl Default for FfxivRdpsCalculator {
    fn default() -> Self {
        FfxivRdpsCalculator {}
    }
}

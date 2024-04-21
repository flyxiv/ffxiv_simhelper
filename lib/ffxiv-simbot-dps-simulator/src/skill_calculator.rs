use crate::damage_rdps_profile::{FfxivRaidDamageTable, RaidDamageTable, RaidDamageTableKey};
use crate::multiplier_calculator::MultiplierCalculator;
use ffxiv_simbot_combat_components::status::{BuffStatus, DebuffStatus, Status, StatusHolder};
use ffxiv_simbot_combat_components::{DamageType, IdType};
use ffxiv_simbot_db::stat_calculator::CharacterPower;
use ffxiv_simbot_db::DamageMultiplierType;
use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::default;
use std::rc::Rc;

/// Simulates the effect of a single skill and distribute the damage contribution of each
/// buff to the rightful owner.

pub(crate) struct SkillDamageResult {
    pub(crate) raw_damage: DamageType,
    /// damage after adding all buffs/debuffs
    pub(crate) final_damage: DamageType,
    pub(crate) raid_damage_profile: FfxivRaidDamageTable,
}

pub(crate) trait SkillCalculator {
    /// Given the raw damage and all the list of buffs/debuffs on the player and the target,
    /// 1) Convert the buffs to a damage multiplier.
    /// 2) Calculate the RDPS contribution of each buff
    /// 3) Order each buff to update its RDPS contribution.
    fn make_damage_profile<SHB: StatusHolder<BuffStatus>, SHD: StatusHolder<DebuffStatus>>(
        &self,
        buff_holder: Rc<RefCell<SHB>>,
        debuff_holder: Rc<RefCell<SHD>>,
        skill_damage: DamageType,
        power: &CharacterPower,
        player_id: IdType,
    ) -> SkillDamageResult;
}

fn apply_multiplier(damage: DamageType, multiplier: DamageMultiplierType) -> DamageType {
    f64::floor(damage as DamageMultiplierType * multiplier) as DamageType
}

pub(crate) struct FfxivSkillCalculator {}

impl MultiplierCalculator for FfxivSkillCalculator {}

impl SkillCalculator for FfxivSkillCalculator {
    fn make_damage_profile<SHB, SHD>(
        &self,
        buff_holder: Rc<RefCell<SHB>>,
        debuff_holder: Rc<RefCell<SHD>>,
        skill_damage: DamageType,
        power: &CharacterPower,
        player_id: IdType,
    ) -> SkillDamageResult
    where
        SHB: StatusHolder<BuffStatus>,
        SHD: StatusHolder<DebuffStatus>,
    {
        let mut damage_profile: SkillDamageResult = SkillDamageResult {
            raw_damage: skill_damage,
            final_damage: skill_damage,
            raid_damage_profile: FfxivRaidDamageTable {
                rdps_table: Default::default(),
            },
        };

        let buff_list = buff_holder.borrow().get_status_list();
        let debuff_list = debuff_holder.borrow().get_status_list();

        for buff in buff_list.borrow().iter() {
            let buff_id = buff.get_id();

            let raid_damage_profile_key = RaidDamageTableKey {
                player_id,
                status_id: buff_id,
            };

            let damage_multiplier = self.calculate_multiplier(buff, power);
            let contribution = apply_multiplier(skill_damage, damage_multiplier) - skill_damage;

            damage_profile.final_damage =
                apply_multiplier(damage_profile.final_damage, damage_multiplier);
            damage_profile
                .raid_damage_profile
                .insert(raid_damage_profile_key, contribution);
        }

        for debuff in debuff_list.borrow().iter() {
            let buff_id = debuff.get_id();

            let raid_damage_profile_key = RaidDamageTableKey {
                player_id,
                status_id: buff_id,
            };

            let damage_multiplier = self.calculate_multiplier(debuff, power);
            let contribution = apply_multiplier(skill_damage, damage_multiplier) - skill_damage;

            damage_profile.final_damage =
                apply_multiplier(damage_profile.final_damage, damage_multiplier);
            damage_profile
                .raid_damage_profile
                .insert(raid_damage_profile_key, contribution);
        }

        damage_profile
    }
}

impl Default for FfxivSkillCalculator {
    fn default() -> Self {
        FfxivSkillCalculator {}
    }
}

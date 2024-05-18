use crate::damage_calculator::RaidDamageTableKey;
/// Reads the total buff/debuff status snapshot of the skill and calculates the
/// skill's final damage + how much each buff/debuff contributed to that number.
use crate::{DamageType, IdType};
use std::collections::HashMap;

pub type RdpsTable = HashMap<RaidDamageTableKey, DamageType>;

/// Total Damage/Rdps database.
/// This is used to calculate the total rdps of the player in all possible perspectives.
pub trait RaidDamageTable {
    /// get total RDPS of the status with the given StatusKey(status_id, owner_id)
    /// station_id and owner_id are needed to uniquely identify the status,
    /// sinch there could be multiple same jobs in the party.
    fn query_by_status(&self, status_id: IdType, owner_id: IdType) -> DamageType;
    /// get total raid damage contribution of the player
    fn query_by_key(&self, key: RaidDamageTableKey) -> DamageType;
    fn insert(&mut self, key: RaidDamageTableKey, damage: DamageType);
    fn update_table(&mut self, new_data: &Self);
}

pub struct FfxivRaidDamageTable {
    pub rdps_table: RdpsTable,
}

impl RaidDamageTable for FfxivRaidDamageTable {
    fn query_by_status(&self, status_id: IdType, owner_id: IdType) -> DamageType {
        let mut total_damage = 0;

        for (key, damage) in &self.rdps_table {
            if key.status_id == status_id && key.owner_id == owner_id {
                total_damage += damage;
            }
        }
        total_damage
    }

    fn query_by_key(&self, key: RaidDamageTableKey) -> DamageType {
        if let Some(damage) = self.rdps_table.get(&key) {
            *damage
        } else {
            0
        }
    }

    fn insert(&mut self, key: RaidDamageTableKey, damage: DamageType) {
        self.rdps_table.insert(key, damage);
    }

    fn update_table(&mut self, new_data: &Self) {
        for (key, damage) in &new_data.rdps_table {
            if let Some(current_damage) = self.rdps_table.get_mut(key) {
                *current_damage += damage;
            } else {
                self.rdps_table.insert(key.clone(), *damage);
            }
        }
    }
}

impl Default for FfxivRaidDamageTable {
    fn default() -> Self {
        FfxivRaidDamageTable {
            rdps_table: RdpsTable::new(),
        }
    }
}

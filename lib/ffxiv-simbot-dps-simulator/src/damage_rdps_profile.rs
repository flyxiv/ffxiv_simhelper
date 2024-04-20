use ffxiv_simbot_combat_components::{DamageType, IdType};
use std::collections::HashMap;

/// Saves the Raid Damage contribution by each status, by each player .
#[derive(PartialEq, Eq, Hash, Clone)]
pub(crate) struct RaidDamageTableKey {
    pub(crate) player_id: IdType,
    pub(crate) status_id: IdType,
}

pub trait RaidDamageTable {
    /// get total raid damage of the status
    fn query_by_status_id(&self, status_id: IdType) -> DamageType;
    /// get total raid damage of the player
    fn query_by_player_id(&self, player_id: IdType) -> DamageType;
    fn query_by_key(&self, key: RaidDamageTableKey) -> DamageType;
    fn insert(&mut self, key: RaidDamageTableKey, damage: DamageType);
    fn update_table(&mut self, new_data: &Self);
}

pub(crate) struct FfxivRaidDamageTable {
    pub(crate) rdps_table: HashMap<RaidDamageTableKey, DamageType>,
}

impl RaidDamageTable for FfxivRaidDamageTable {
    fn query_by_status_id(&self, status_id: IdType) -> DamageType {
        let mut total_damage = 0;

        for (key, damage) in &self.rdps_table {
            if key.status_id == status_id {
                total_damage += damage;
            }
        }
        total_damage
    }

    fn query_by_player_id(&self, player_id: IdType) -> DamageType {
        let mut total_damage = 0;

        for (key, damage) in &self.rdps_table {
            if key.player_id == player_id {
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

use crate::errors::DataError;
use crate::stat::StatType;
use std::collections::HashMap;
use std::hash::Hash;
use std::path::PathBuf;

/// Hash Table for fast searching by the object's Id.
pub type IdTable<T, U> = HashMap<T, U>;
pub type StatModifierType = f64;
pub type MultiplierType = f64;

pub(crate) mod character;
pub mod clan;
pub mod constants;
pub mod equipment;
pub mod errors;
pub mod ffxiv_context;
pub mod food;
pub mod job;
pub mod materia;
mod medicine;
pub mod stat;
pub mod stat_calculator;

/// Saves Base Constants Needed for getting Job Attributes for Stats
/// https://www.akhmorning.com/allagan-studies/modifiers/levelmods/
#[derive(PartialEq, Copy, Clone)]
pub struct StatModifier {
    pub max_level_main_stat_modifier: StatModifierType,
    pub max_level_base_vitality: StatType,
    pub max_level_base_piety: StatType,
    pub max_level_base_direct_hit: StatType,
    pub max_level_base_critical_hit: StatType,
    pub max_level_base_determination: StatType,
    pub max_level_base_skill_speed: StatType,
    pub max_level_base_spell_speed: StatType,
    pub max_level_base_tenacity: StatType,
    pub max_level_sub_stat_modifier: StatModifierType,
    pub max_level_div: StatModifierType,
    pub hp_per_vitality_non_tank: StatModifierType,
    pub hp_per_vitality_tank: StatModifierType,
    pub max_level_hp_modifier: f64,
}

pub(crate) trait JsonFileReader {
    fn read_json_file(&self, file_path: &PathBuf) -> Result<String, DataError> {
        std::fs::read_to_string(file_path).map_err(|e| format!("{:?}", e).into())
    }
}

/// Extract the Search Key from the object.
pub trait SearchKeyEntity<T>
where
    T: Hash + Sized + Eq,
{
    fn get_search_key(&self) -> Vec<T>;
}

/// convert id containing item vector to id -> item hash map for faster search.
pub(crate) fn item_vec_to_id_table<T, U>(items: Vec<U>) -> IdTable<T, U>
where
    T: Hash + Sized + Eq,
    U: SearchKeyEntity<T> + Sized + Clone,
{
    let mut table = HashMap::new();

    for item in items {
        let keys = item.get_search_key();

        for key in keys {
            table.insert(key, item.clone());
        }
    }

    table
}

pub(crate) fn item_vec_to_id_vec_table<T, U>(items: Vec<U>) -> IdTable<T, Vec<U>>
where
    T: Hash + Sized + Eq,
    U: SearchKeyEntity<T> + Sized + Clone,
{
    let mut table = HashMap::new();

    for item in items {
        let keys = item.get_search_key();

        for key in keys {
            if !table.contains_key(&key) {
                table.insert(key, vec![item.clone()]);
            } else {
                let entry = table.get_mut(&key).unwrap();
                entry.push(item.clone());
            }
        }
    }

    table
}

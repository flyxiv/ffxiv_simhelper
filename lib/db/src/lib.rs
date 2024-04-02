use crate::equipment::SlotType;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::hash::Hash;
use std::path::PathBuf;

/// Hash Table for fast searching by the object's Id.
pub type IdTable<T, U> = HashMap<T, U>;
type Result<T> = std::result::Result<T, DataError>;

pub(crate) mod character;
pub mod clan;
pub mod equipment;
pub mod food;
pub mod job;
pub mod materia;
mod medicine;
pub mod stat;

pub enum DataError {
    FileNotFoundError(String),
    JsonParseError(String),
    EquipmentParseError(String),
    JobClassParseError(String),
    RaceParseError(String),
    StatParseError(String),
    EquipError(SlotType),
    UnEquipError(SlotType),
    MateriaEquipError(SlotType),
    MateriaUnequipError(SlotType),
}

impl Debug for DataError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DataError::FileNotFoundError(s) => write!(f, "File not found: {}", s),
            DataError::JsonParseError(s) => write!(f, "Error parsing JSON: {}", s),
            DataError::EquipmentParseError(s) => write!(f, "Error parsing equipment: {}", s),
            DataError::JobClassParseError(s) => write!(f, "Error parsing job class: {}", s),
            DataError::RaceParseError(s) => write!(f, "Error parsing race: {}", s),
            DataError::StatParseError(s) => write!(f, "Error parsing stat: {}", s),
            DataError::EquipError(slot) => write!(f, "Equip to Invalid Slot: {:?}", slot),
            DataError::UnEquipError(slot) => write!(f, "Unequip Invalid Slot: {:?}", slot),
            DataError::MateriaEquipError(slot) => {
                write!(f, "Cannot Equip Materia to Slot: {:?}", slot)
            }
            DataError::MateriaUnequipError(slot) => {
                write!(f, "Cannot Unequip Materia in Slot: {:?}", slot)
            }
        }
    }
}

impl Display for DataError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DataError::FileNotFoundError(s) => write!(f, "File not found: {}", s),
            DataError::JsonParseError(s) => write!(f, "Error parsing JSON: {}", s),
            DataError::EquipmentParseError(s) => write!(f, "Error parsing equipment: {}", s),
            DataError::JobClassParseError(s) => write!(f, "Error parsing job class: {}", s),
            DataError::RaceParseError(s) => write!(f, "Error parsing race: {}", s),
            DataError::StatParseError(s) => write!(f, "Error parsing stat: {}", s),
            DataError::EquipError(slot) => write!(f, "Equip to Invalid Slot: {:?}", slot),
            DataError::UnEquipError(slot) => write!(f, "Unequip Invalid Slot: {:?}", slot),
            DataError::MateriaEquipError(slot) => write!(f, "Cannot Equip to Slot: {:?}", slot),
            DataError::MateriaUnequipError(slot) => {
                write!(f, "Cannot Unequip Materia in Slot: {:?}", slot)
            }
        }
    }
}

impl Error for DataError {}

impl From<serde_json::Error> for DataError {
    fn from(e: serde_json::Error) -> Self {
        DataError::JsonParseError(e.to_string())
    }
}

impl From<String> for DataError {
    fn from(s: String) -> Self {
        Self::FileNotFoundError(s)
    }
}

trait JsonFileReader {
    fn read_json_file(&self, file_path: &PathBuf) -> Result<String> {
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

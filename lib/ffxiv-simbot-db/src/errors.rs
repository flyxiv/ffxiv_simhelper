use crate::equipment::SlotType;
use thiserror::Error;

pub(crate) type Result<T> = std::result::Result<T, DataError>;

#[derive(Error, Debug)]
pub enum DataError {
    #[error("File not found: {0}")]
    FileNotFoundError(String),
    #[error("Error parsing JSON: {0}")]
    JsonParseError(String),
    #[error("Error parsing JSON: {0}")]
    SerdeJsonError(#[from] serde_json::Error),
    #[error("Error parsing equipment: {0}")]
    EquipmentParseError(String),
    #[error("Error parsing job class: {0}")]
    JobClassParseError(String),
    #[error("Error parsing materia: {0}")]
    RaceParseError(String),
    #[error("Error parsing stat: {0}")]
    StatParseError(String),
    #[error("Error parsing stat calculator: {0:?}")]
    EquipError(SlotType),
    #[error("Unequip Invalid Slot: {0:?}")]
    UnEquipError(SlotType),
    #[error("Cannot Equip Materia to Slot: {0:?}")]
    MateriaEquipError(SlotType),
    #[error("Cannot Unequip Materia in Slot: {0:?}")]
    MateriaUnequipError(SlotType),
}

impl From<String> for DataError {
    fn from(s: String) -> Self {
        DataError::FileNotFoundError(s)
    }
}

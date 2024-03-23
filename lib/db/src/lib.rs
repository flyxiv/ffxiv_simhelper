use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

type Result<T> = std::result::Result<T, DataError>;

pub(crate) mod character;
pub(crate) mod clan;
pub(crate) mod equipment;
pub(crate) mod food;
pub(crate) mod job;
pub(crate) mod materia;
mod medicine;
pub(crate) mod stat;

pub(crate) enum DataError {
    FileNotFoundError(String),
    JsonParseError,
    EquipmentParseError,
    JobClassParseError,
    RaceParseError,
    StatParseError,
}

impl Debug for DataError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DataError::FileNotFoundError(_) => f.fmt("File not found"),
            DataError::JsonParseError => f.fmt("Error parsing JSON"),
            DataError::EquipmentParseError => f.fmt("Error parsing equipment"),
            DataError::JobClassParseError => f.fmt("Error parsing job class"),
            DataError::RaceParseError => f.fmt("Error parsing race"),
            DataError::StatParseError => f.fmt("Error parsing stat"),
        }
    }
}

impl Display for DataError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DataError::FileNotFoundError(_) => f.fmt("File not found"),
            DataError::JsonParseError => f.fmt("Error parsing JSON"),
            DataError::EquipmentParseError => f.fmt("Error parsing equipment"),
            DataError::JobClassParseError => f.fmt("Error parsing job class"),
            DataError::RaceParseError => f.fmt("Error parsing race"),
            DataError::StatParseError => f.fmt("Error parsing stat"),
        }
    }
}

impl Error for DataError {}

impl From<serde_json::Error> for DataError {
    fn from(_: serde_json::Error) -> Self {
        DataError::JsonParseError
    }
}

impl From<String> for DataError {
    fn from(s: String) -> Self {
        Self::FileNotFoundError(s)
    }
}

trait JsonFileReader {
    fn read_json_file(&self, file_name: &str) -> Result<String> {
        std::fs::read_to_string(file_name).map_err(|e| format!("{:?}", e).into())
    }
}

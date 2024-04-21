use std::fmt::{Debug, Display, Formatter};

pub mod engine;
mod engine_config;

pub(crate) type Result<T> = std::result::Result<T, EngineError>;

pub enum EngineError {
    ConfigFileNotFoundError(String),
    ConfigParseError(String),
}

impl Display for EngineError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EngineError::ConfigFileNotFoundError(_) => write!(f, "Config File Not Found"),
            EngineError::ConfigParseError(s) => {
                write!(f, "Error parsing config: {}", s)
            }
        }
    }
}

impl Debug for EngineError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EngineError::ConfigFileNotFoundError(_) => write!(f, "Config File Not Found"),
            EngineError::ConfigParseError(s) => {
                write!(f, "Error parsing config: {}", s)
            }
        }
    }
}

impl From<std::io::Error> for EngineError {
    fn from(e: std::io::Error) -> Self {
        EngineError::ConfigFileNotFoundError(e.to_string())
    }
}

impl From<serde_yaml::Error> for EngineError {
    fn from(e: serde_yaml::Error) -> Self {
        EngineError::ConfigParseError(e.to_string())
    }
}

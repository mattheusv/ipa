use shellexpand::LookupError;
use std::{env::VarError, io::Error};

#[derive(Debug)]
pub enum IpaError {
    Io(Error),
    InvalidConfig(serde_yaml::Error),
    InvalidPath(LookupError<VarError>),
    InvalidGroup,
    EmptyCommand,
    InvalidCommand,
}

impl std::fmt::Display for IpaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IpaError::Io(e) => write!(f, "{}", e),
            IpaError::InvalidConfig(e) => write!(f, "Invalid config: {}", e),
            IpaError::InvalidPath(e) => write!(f, "Invalid path: {}", e),
            IpaError::InvalidGroup => write!(f, "Invalid group name"),
            IpaError::EmptyCommand => write!(f, "Empty command to execute"),
            IpaError::InvalidCommand => write!(f, "Invalid command"),
        }
    }
}

impl From<Error> for IpaError {
    fn from(e: Error) -> Self {
        IpaError::Io(e)
    }
}

impl From<serde_yaml::Error> for IpaError {
    fn from(e: serde_yaml::Error) -> Self {
        IpaError::InvalidConfig(e)
    }
}

impl From<LookupError<VarError>> for IpaError {
    fn from(e: LookupError<VarError>) -> Self {
        IpaError::InvalidPath(e)
    }
}

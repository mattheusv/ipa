use std::env::VarError;
use shellexpand::LookupError;
use std::io::Error;

#[derive(Debug)]
pub enum IpaError {
    IO(Error),
    InvalidConfig(serde_yaml::Error),
    InvalidPath(LookupError<VarError>),
    InvalidPackage,
}

impl From<Error> for IpaError {
    fn from(e: Error) -> Self {
        IpaError::IO(e)
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

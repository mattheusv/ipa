use std::io::Error;

#[derive(Debug)]
pub enum IpaError {
    IO(Error),
    InvalidConfig(serde_yaml::Error),
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



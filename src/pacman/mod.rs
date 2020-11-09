use std::io;
pub mod pacman;

pub trait PackageManagement {
    fn install(&self, name: &str) -> Result<(), Error>;
}

#[derive(Debug)]
pub enum Error {
    /// Error installing package
    PacmanSync(String),

    /// io error installing package.
    Io(io::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::PacmanSync(p) => write!(f, "Unable to install package {}", p),
            Error::Io(e) => write!(f, "{}", e),
        }
    }
}

impl From<io::Error> for Error {
    fn from(val: io::Error) -> Self {
        Error::Io(val)
    }
}

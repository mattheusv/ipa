use serde::{Deserialize, Serialize};
use std::io;
#[cfg(feature = "apt")]
pub mod apt;
pub mod archlinux;

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

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Package {
    #[serde(default)]
    pub name: String,
}

impl Package {
    pub fn new(name: &str) -> Self {
        Package {
            name: name.to_string(),
        }
    }
}

pub trait PackageManagement {
    fn install(&self, package: &Package) -> Result<(), Error>;
}

pub struct Pacman<T: PackageManagement> {
    target: T,
}

#[cfg(feature = "pacman")]
impl Pacman<archlinux::Pacman> {
    pub fn new() -> Self {
        Pacman {
            target: archlinux::Pacman::new(),
        }
    }
}

#[cfg(feature = "pacman")]
impl PackageManagement for Pacman<archlinux::Pacman> {
    fn install(&self, package: &Package) -> Result<(), Error> {
        self.target.install(package)
    }
}

#[cfg(feature = "apt")]
impl Pacman<apt::Apt> {
    pub fn new() -> Self {
        Pacman {
            target: apt::Apt::new(),
        }
    }
}

#[cfg(feature = "apt")]
impl PackageManagement for Pacman<apt::Apt> {
    fn install(&self, package: &Package) -> Result<(), Error> {
        self.target.install(package)
    }
}

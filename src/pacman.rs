use log::{debug, warn};
use serde::{Deserialize, Serialize};
use std::io;
use std::process::{Command, Stdio};

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

pub struct Pacman {
    bin: &'static str,
}

impl Pacman {
    pub fn new() -> Self {
        Pacman { bin: "pacman" }
    }

    pub fn is_installed(&self, package: &str) -> std::io::Result<bool> {
        Ok(Command::new(self.bin)
            .arg("-Qe")
            .arg(package)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()?
            .success())
    }
}

impl PackageManagement for Pacman {
    fn install(&self, package: &Package) -> Result<(), Error> {
        if self.is_installed(&package.name)? {
            warn!("Package {} already installed", package.name);
            return Ok(());
        }
        debug!("Installing package {}", package.name);
        let status = Command::new(self.bin)
            .arg("-S")
            .arg(&package.name)
            .arg("--noconfirm")
            .arg("--quiet")
            .stdout(Stdio::null())
            .status()?;
        if status.success() {
            return Ok(());
        }
        return Err(Error::PacmanSync(package.name.clone()));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_install_invalid_command() {
        let pacman = Pacman::new();
        assert!(pacman.install(&Package::new("-bla")).is_err());
    }
}

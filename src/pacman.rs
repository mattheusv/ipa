use log::{debug, warn};
use std::io;
use std::process::{Command, Stdio};

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
    fn install(&self, package: &str) -> Result<(), Error> {
        if self.is_installed(package)? {
            warn!("Package {} already installed", package);
            return Ok(());
        }
        debug!("Installing package {}", package);
        let status = Command::new(self.bin)
            .arg("-S")
            .arg(package)
            .arg("--noconfirm")
            .arg("--quiet")
            .stdout(Stdio::null())
            .status()?;
        if status.success() {
            return Ok(());
        }
        return Err(Error::PacmanSync(package.to_string()));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_install_invalid_command() {
        let pacman = Pacman::new();
        assert!(pacman.install("-bla").is_err());
    }
}

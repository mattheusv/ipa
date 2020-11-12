use crate::pacman::{Error, Package, PackageManagement};
use log::{debug, warn};
use std::process::{Command, Stdio};

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

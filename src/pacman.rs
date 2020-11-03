use super::{error, PackageManagement};
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
    fn install(&self, package: &str) -> Result<(), error::IpaError> {
        if self.is_installed(package)? {
            warn!("Package {} already installed", package);
            return Ok(());
        }
        debug!("Installing package {}", package);
        Command::new(self.bin)
            .arg("-S")
            .arg(package)
            .arg("--noconfirm")
            .arg("--quiet")
            .stdout(Stdio::null())
            .status()?;
        Ok(())
    }
}

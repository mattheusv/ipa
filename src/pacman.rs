use ipa::{error, PackageManagement};
use std::process::{Command, Stdio};

pub struct Pacman {
    bin: &'static str,
}

impl Pacman {
    pub fn new() -> Self {
        Pacman { bin: "pacman" }
    }

    fn is_installed(&self, package: &str) -> std::io::Result<bool> {
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
            println!("Package {} already installed", package);
            return Ok(());
        }
        println!("Installing package {}", package);
        Command::new(self.bin)
            .arg("-S")
            .arg(package)
            .arg("--noconfirm")
            .arg("--quiet")
            .status()?;
        Ok(())
    }
}

use super::{error, PackageManagement};

pub struct Pacman {
    bin: String,
    install_cmd: String,
    args: Vec<String>,
}

impl Pacman {
    pub fn new() -> Self {
        Pacman {
            bin: String::from("pacman"),
            install_cmd: String::from("-S"),
            args: vec![],
        }
    }
}

impl PackageManagement for Pacman {
    fn install(&self, name: &String) -> Result<(), error::IpaError> {
        println!("Installing package {}", name);
        Ok(())
    }
}

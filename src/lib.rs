use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
mod error;
mod pacman;


pub trait PackageManagement {
    fn install(&self, name: &str) -> Result<(), error::IpaError>;
}


#[derive(Debug, Serialize, Deserialize)]
struct SysLink {
    config: String,
    path: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Package {
    #[serde(default)]
    name: String,

    #[serde(default)]
    config: String,

    #[serde(default)]
    path: String,

    #[serde(default)]
    aur: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    #[serde(default)]
    packages: Vec<Package>,

    #[serde(default)]
    sys_links: Vec<SysLink>,
}

impl Config {
    pub fn new(config_file: &Path) -> Result<Self, error::IpaError> {
        let content = fs::read_to_string(config_file)?;

        let config: Config = serde_yaml::from_str(&content)?;
        Ok(config)
    }
}

pub struct Ipa {
    config: Config,
    pacman: Box<dyn PackageManagement>,
}

impl Ipa {
    pub fn new(config_file: &Path) -> Result<Self, error::IpaError> {
        let config = Config::new(config_file)?;
        let pacman = Box::new(pacman::Pacman::new());

        Ok(Ipa {
            config: config,
            pacman: pacman,
        })
    }

    pub fn process(&self) -> Result<(), error::IpaError> {
        for package in self.config.packages.iter() {
            self.pacman.install(&package.name)?;
        }
        Ok(())
    }
}

use serde::{Deserialize, Serialize};
use std::fs;
use std::os::unix;
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
pub struct Config {
    #[serde(default)]
    packages: Vec<Package>,

    #[serde(default)]
    sys_links: Vec<SysLink>,
}

impl Config {
    pub fn new(content: &str) -> Result<Self, error::IpaError> {
        let config: Config = serde_yaml::from_str(&content)?;
        Ok(config)
    }
}

pub struct Ipa {
    config: Config,
    pacman: Box<dyn PackageManagement>,
}

impl Ipa {
    pub fn new(config: Config, pacman: Box<dyn PackageManagement>) -> Self {
        Ipa {
            config: config,
            pacman: pacman,
        }
    }

    pub fn from_file(config_yaml: &str) -> Result<Self, error::IpaError> {
        let content = fs::read_to_string(config_yaml)?;

        let config = Config::new(&content)?;
        let pacman = Box::new(pacman::Pacman::new());

        Ok(Ipa {
            config: config,
            pacman: pacman,
        })
    }

    pub fn process(&self) -> Result<(), error::IpaError> {
        for package in self.config.packages.iter() {
            self.pacman.install(&package.name)?;
            if !package.config.is_empty() && !package.path.is_empty() {
                self.symlink(Path::new(&package.path), Path::new(&package.config))?;
            }
        }

        for sys_link in self.config.sys_links.iter() {
            self.symlink(Path::new(&sys_link.path), Path::new(&sys_link.config))?;
        }
        Ok(())
    }

    fn symlink(&self, src: &Path, dst: &Path) -> Result<(), error::IpaError> {
        if dst.exists() {
            println!("Symbolic link {:?} already exists", dst);
            return Ok(());
        }
        println!("Linking {:?} in {:?}", src, dst);
        unix::fs::symlink(src, dst)?;
        Ok(())
    }
}

use serde::{Deserialize, Serialize};
use std::fs;
use std::os::unix;
use std::path::Path;
mod error;
mod pacman;

pub trait PackageManagement {
    fn install(&self, name: &str) -> Result<(), error::IpaError>;
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct SymLink {
    config: String,

    path: String,

    #[serde(default)]
    relink: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct Package {
    #[serde(default)]
    name: String,

    #[serde(default)]
    link: SymLink,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    packages: Vec<Package>,

    #[serde(default)]
    link: Vec<SymLink>,
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

    pub fn from_file(config_yaml: &Path) -> Result<Self, error::IpaError> {
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
            if !package.link.config.is_empty() && !package.link.path.is_empty() {
                self.symlink(
                    Path::new(&package.link.path),
                    Path::new(&package.link.config),
                    package.link.relink,
                )?;
            }
        }

        for link in self.config.link.iter() {
            self.symlink(Path::new(&link.path), Path::new(&link.config), link.relink)?;
        }

        Ok(())
    }

    fn symlink_dir(&self, src: &Path, dst: &Path, relink: bool) -> Result<(), error::IpaError> {
        println!("Create symbolic link to all files into {:?}", src);
        for entry in fs::read_dir(src)? {
            let entry = entry?;

            // The first entry of iterator is src path
            if entry.path() == src {
                continue;
            }

            if entry.path().is_dir() {
                if let Some(name) = entry.path().file_name() {
                    let dst_dir = dst.join(name);
                    if !dst_dir.exists() {
                        fs::create_dir(&dst_dir)?;
                    }
                    self.symlink_dir(entry.path().as_path(), dst_dir.as_path(), relink)?;
                }
            } else {
                if let Some(name) = entry.path().file_name() {
                    self.symlink(entry.path().as_path(), dst.join(name).as_path(), relink)?;
                }
            }
        }
        Ok(())
    }

    fn symlink(&self, src: &Path, dst: &Path, relink: bool) -> Result<(), error::IpaError> {
        if relink && dst.exists() && !dst.is_dir() {
            println!("Relinking {:?}", dst);
            fs::remove_file(dst)?;
        }

        if src.is_dir() && dst.is_dir() {
            return self.symlink_dir(src, dst, relink);
        }

        if dst.exists() {
            println!("Symbolic link {:?} already exists", dst);
            return Ok(());
        }
        println!("Linking {:?} in {:?}", src, dst);
        unix::fs::symlink(src, dst)?;
        Ok(())
    }
}

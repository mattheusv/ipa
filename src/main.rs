use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
mod error;

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
struct Action {
    #[serde(default)]
    packages: Vec<Package>,

    #[serde(default)]
    sys_links: Vec<SysLink>,
}

impl Action {
    pub fn new(config_file: &Path) -> Result<Self, error::IpaError> {
        let content = fs::read_to_string(config_file)?;

        let action: Action = serde_yaml::from_str(&content)?;
        Ok(action)
    }
}

fn install_packages(packages: Vec<Package>) -> Result<(), error::IpaError> {
    for package in packages {
        println!("Installing package {}", package.name);
        if !package.config.is_empty() && !package.path.is_empty() {
            sys_link_file(Path::new(&package.config), Path::new(&package.path))?;
        }
    }
    Ok(())
}

fn sys_link(links: Vec<SysLink>) -> Result<(), error::IpaError> {
    for link in links {
        sys_link_file(Path::new(&link.config), Path::new(&link.path))?;
    }
    Ok(())
}

fn sys_link_file(target: &Path, link_name: &Path) -> Result<(), error::IpaError> {
    println!("Linking {:?} on {:?}", link_name, target);
    Ok(())
}

fn main() -> Result<(), error::IpaError> {
    let action = Action::new(Path::new("ipa.yml"))?;

    install_packages(action.packages)?;
    sys_link(action.sys_links)?;
    Ok(())
}

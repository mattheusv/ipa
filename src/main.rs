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

fn main() -> Result<(), error::IpaError> {
    let action = Action::new(Path::new("ipa.yml"))?;

    println!("{:#?}", action);
    Ok(())
}

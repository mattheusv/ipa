use super::IpaError;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct SymLink {
    pub config: String,

    pub path: String,

    #[serde(default)]
    pub relink: bool,

    #[serde(default)]
    pub group: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Package {
    #[serde(default)]
    pub name: String,

    #[serde(default)]
    pub link: SymLink,

    #[serde(default)]
    pub group: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub packages: Vec<Package>,

    #[serde(default)]
    pub link: Vec<SymLink>,
}

impl Config {
    pub fn new(content: &str) -> Result<Self, IpaError> {
        let config: Config = serde_yaml::from_str(&content)?;
        Ok(config)
    }

    pub fn load(config_file: &Path) -> Result<Self, IpaError> {
        let content = fs::read_to_string(config_file)?;
        Config::new(&content)
    }
}

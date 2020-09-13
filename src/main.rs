use serde::{Serialize, Deserialize};
use std::io::Error;
use std::path::Path;
use std::fs;

#[derive(Debug)]
enum IpaError {
    IO(Error),
    InvalidConfig(serde_yaml::Error),
}

impl From<Error> for IpaError {
    fn from(e: Error) -> Self {
        IpaError::IO(e)
    }
}

impl From<serde_yaml::Error> for IpaError {
    fn from(e: serde_yaml::Error) -> Self {
        IpaError::InvalidConfig(e)
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Link {
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
    sys_links: Vec<Link>,
}

impl Action{
    pub fn new(config_file: &Path) -> Result<Self, IpaError> {
        let content = fs::read_to_string(config_file)?;

        let action: Action = serde_yaml::from_str(&content)?;
        Ok(action)
    }
}


fn main() -> Result<(), IpaError>{
    let action = Action::new(Path::new("ipa.yml"))?;

    println!("{:#?}", action);
    Ok(())
}

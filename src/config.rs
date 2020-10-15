use super::IpaError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct SymLink {
    pub config: String,

    pub path: String,

    #[serde(default)]
    pub relink: bool,
}

impl SymLink {
    pub fn new(config: &str, path: &str, relink: bool) -> Self {
        SymLink {
            config: config.to_string(),
            path: path.to_string(),
            relink,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Package {
    #[serde(default)]
    pub name: String,
}

impl Package {
    pub fn new(name: &str) -> Self {
        Package {
            name: name.to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Values {
    pub package: Option<Package>,
    pub link: Option<SymLink>,
}

impl Values {
    pub fn new(package: Option<Package>, link: Option<SymLink>) -> Self {
        Values { package, link }
    }
}

#[derive(Debug, PartialEq)]
pub struct Config {
    pub values: HashMap<String, Vec<Values>>,
}

impl Config {
    pub fn new(content: &str) -> Result<Self, IpaError> {
        let values: HashMap<String, Vec<Values>> = serde_yaml::from_str(&content)?;
        Ok(Config { values })
    }

    pub fn load(config_file: &Path) -> Result<Self, IpaError> {
        let content = fs::read_to_string(config_file)?;
        Config::new(&content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_config() {
        let content = "
gui:
  - link:
      config: foo/bar
      path: foo/baz
    package:
      name: neovim
  - package:
      name: alacritty

dev:
  - link:
      config: bar/foo
      path: baz/foo
      relink: true
";
        let config = Config::new(&content).unwrap();
        let mut values = HashMap::new();
        values.insert(
            String::from("gui"),
            vec![
                Values::new(
                    Some(Package::new("neovim")),
                    Some(SymLink::new("foo/bar", "foo/baz", false)),
                ),
                Values::new(Some(Package::new("alacritty")), None),
            ],
        );
        values.insert(
            String::from("dev"),
            vec![Values::new(
                None,
                Some(SymLink::new("bar/foo", "baz/foo", true)),
            )],
        );

        let expected_config = Config { values };
        assert_eq!(expected_config, config);
    }
}

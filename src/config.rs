use super::IpaError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct SymLink {
    pub dst: String,

    pub src: String,

    #[serde(default)]
    pub relink: bool,
}

impl SymLink {
    pub fn new(dst: &str, src: &str, relink: bool) -> Self {
        SymLink {
            dst: dst.to_string(),
            src: src.to_string(),
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
    pub shell: Option<Vec<String>>,
}

impl Values {
    pub fn new(
        package: Option<Package>,
        link: Option<SymLink>,
        shell: Option<Vec<String>>,
    ) -> Self {
        Values {
            package,
            link,
            shell,
        }
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
      dst: foo/bar
      src: foo/baz
    shell:
      - echo
      - foo
    package:
      name: neovim
  - package:
      name: alacritty

dev:
  - link:
      dst: bar/foo
      src: baz/foo
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
                    Some(vec![String::from("echo"), String::from("foo")]),
                ),
                Values::new(Some(Package::new("alacritty")), None, None),
            ],
        );
        values.insert(
            String::from("dev"),
            vec![Values::new(
                None,
                Some(SymLink::new("bar/foo", "baz/foo", true)),
                None,
            )],
        );

        let expected_config = Config { values };
        assert_eq!(expected_config, config);
    }
}

use crate::{pacman::Package, shell::Shell, symlink::SymLink};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, io, path::Path};

#[derive(Debug)]
pub enum Error {
    /// Config file not found.
    NotFound,

    /// io error reading file.
    Io(io::Error),

    /// Not valid yaml or missing parameters.
    Yaml(serde_yaml::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::NotFound => write!(f, "Unable to locate config file"),
            Error::Io(e) => write!(f, "{}", e),
            Error::Yaml(e) => write!(f, "Problem with config: {}", e),
        }
    }
}

impl From<io::Error> for Error {
    fn from(val: io::Error) -> Self {
        if val.kind() == io::ErrorKind::NotFound {
            Error::NotFound
        } else {
            Error::Io(val)
        }
    }
}

impl From<serde_yaml::Error> for Error {
    fn from(e: serde_yaml::Error) -> Self {
        Error::Yaml(e)
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Values {
    pub package: Option<Package>,
    pub link: Option<SymLink>,
    pub shell: Option<Shell>,
}

impl Values {
    pub fn new(package: Option<Package>, link: Option<SymLink>, shell: Option<Shell>) -> Self {
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
    pub fn new(content: &str) -> Result<Self, Error> {
        let values: HashMap<String, Vec<Values>> = serde_yaml::from_str(&content)?;
        Ok(Config { values })
    }

    pub fn load(config_file: &Path) -> Result<Self, Error> {
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
      command: echo foo
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
                    Some(SymLink::new("foo/bar", "foo/baz", false, true)),
                    Some(Shell::new("echo foo")),
                ),
                Values::new(Some(Package::new("alacritty")), None, None),
            ],
        );
        values.insert(
            String::from("dev"),
            vec![Values::new(
                None,
                Some(SymLink::new("bar/foo", "baz/foo", true, true)),
                None,
            )],
        );

        let expected_config = Config { values };
        assert_eq!(expected_config, config);
    }
}

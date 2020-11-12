use crate::{
    config::{Config, Values},
    pacman, shell, symlink,
};
use log::info;

#[derive(Debug)]
pub enum Error {
    /// Group name not exists in config
    InvalidGroup,

    /// Error with instalation of package
    Pacman(pacman::Error),

    /// Error with creating symbolic link
    SymLink(symlink::Error),

    /// Error to execute shell commands
    Shell(shell::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidGroup => write!(f, "Invalid group name"),
            Error::Pacman(err) => err.fmt(f),
            Error::SymLink(err) => err.fmt(f),
            Error::Shell(err) => err.fmt(f),
        }
    }
}

impl From<shell::Error> for Error {
    fn from(val: shell::Error) -> Self {
        Error::Shell(val)
    }
}

impl From<pacman::Error> for Error {
    fn from(val: pacman::Error) -> Self {
        Error::Pacman(val)
    }
}

impl From<symlink::Error> for Error {
    fn from(val: symlink::Error) -> Self {
        Error::SymLink(val)
    }
}

pub struct Ipa<'a, P: pacman::PackageManagement> {
    config: Config,
    pacman: &'a P,
}

impl<'a, P> Ipa<'a, P>
where
    P: pacman::PackageManagement,
{
    pub fn new(config: Config, pacman: &'a P) -> Self {
        Ipa { config, pacman }
    }

    pub fn setup_except_group(&self, group: &str) -> Result<(), Error> {
        for (g, values) in self.config.values.iter() {
            if g != group {
                info!("Configuring values of group {}", group);
                self.process(values)?;
            }
        }
        Ok(())
    }

    pub fn setup_group(&self, group: &str) -> Result<(), Error> {
        if let Some(values) = self.config.values.get(group) {
            info!("Configuring values of group {}", group);
            return self.process(&values);
        }
        Err(Error::InvalidGroup)
    }

    pub fn setup(&self) -> Result<(), Error> {
        for (group, values) in self.config.values.iter() {
            info!("Configuring values of group {}", group);
            self.process(values)?;
        }
        Ok(())
    }

    fn process(&self, values: &[Values]) -> Result<(), Error> {
        for value in values.iter() {
            self.process_value(value)?;
        }
        Ok(())
    }

    fn process_value(&self, value: &Values) -> Result<(), Error> {
        if let Some(ref package) = value.package {
            self.pacman.install(&package)?;
        }

        if let Some(ref link) = value.link {
            symlink::symlink(link)?;
        }

        if let Some(ref shell) = value.shell {
            shell::execute(&shell)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pacman::Package;
    use std::cell::RefCell;
    use tempfile::{tempdir, NamedTempFile};

    struct FakePacman {
        installed_packages: RefCell<Vec<String>>,
    }

    impl FakePacman {
        fn new() -> Self {
            FakePacman {
                installed_packages: RefCell::new(vec![]),
            }
        }
    }

    impl pacman::PackageManagement for FakePacman {
        fn install(&self, package: &Package) -> Result<(), pacman::Error> {
            self.installed_packages
                .borrow_mut()
                .push(String::from(&package.name));
            Ok(())
        }
    }

    #[test]
    fn test_setup_all_packages() {
        let content = "
dev:
  - package:
      name: neovim

gui:
  - package:
      name: alacritty
            ";

        let config = Config::new(&content).unwrap();

        let pacman = FakePacman::new();
        let ipa = Ipa::new(config, &pacman);

        ipa.setup().unwrap();

        assert_eq!(2, pacman.installed_packages.borrow().len());
    }

    #[test]
    fn test_setup_group_packages() {
        let content = "
dev:
  - package:
      name: neovim

gui:
  - package:
      name: alacritty
            ";

        let config = Config::new(&content).unwrap();

        let pacman = FakePacman::new();
        let ipa = Ipa::new(config, &pacman);

        ipa.setup_group("dev").unwrap();

        assert_eq!(1, pacman.installed_packages.borrow().len());
    }

    #[test]
    fn test_setup_except_packages() {
        let content = "
dev:
  - package:
      name: neovim
  - package:
      name: tmux

gui:
  - package:
      name: alacritty
            ";

        let config = Config::new(&content).unwrap();

        let pacman = FakePacman::new();
        let ipa = Ipa::new(config, &pacman);

        ipa.setup_except_group("dev").unwrap();

        assert_eq!(1, pacman.installed_packages.borrow().len());
    }

    #[test]
    fn test_link() {
        let src_dir = tempdir().unwrap();
        let dst_dir = tempdir().unwrap();

        let src_config = NamedTempFile::new_in(&src_dir).unwrap();

        let content = format!(
            "
dev:
    - link:
        dst: {:?}
        src: {:?}
",
            dst_dir.path(),
            src_dir.path(),
        );

        let config = Config::new(&content).unwrap();

        let pacman = FakePacman::new();
        let ipa = Ipa::new(config, &pacman);

        ipa.setup().unwrap();

        let dst_config = dst_dir.path().join(src_config.path().file_name().unwrap());

        assert!(
            dst_config.as_path().exists(),
            "Assert that {:?} is created",
            dst_config.as_path()
        );
    }
}

use super::{config, error, shell, symlink, PackageManagement};

use config::{Config, Values};
use error::IpaError;
use symlink::symlink;

pub struct Ipa<'a, P: PackageManagement> {
    config: Config,
    pacman: &'a P,
}

impl<'a, P> Ipa<'a, P>
where
    P: PackageManagement,
{
    pub fn new(config: Config, pacman: &'a P) -> Self {
        Ipa { config, pacman }
    }

    pub fn setup_except_group(&self, group: &str) -> Result<(), IpaError> {
        for (g, values) in self.config.values.iter() {
            if g != group {
                println!("Configuring values of group {}", group);
                self.process(values)?;
            }
        }
        Ok(())
    }

    pub fn setup_group(&self, group: &str) -> Result<(), IpaError> {
        if let Some(values) = self.config.values.get(group) {
            return self.process(&values);
        }
        Err(IpaError::InvalidGroup)
    }

    pub fn setup(&self) -> Result<(), IpaError> {
        for (group, values) in self.config.values.iter() {
            println!("Configuring values of group {}", group);
            self.process(values)?;
        }
        Ok(())
    }

    fn process(&self, values: &Vec<Values>) -> Result<(), IpaError> {
        for value in values.iter() {
            self.process_value(value)?;
        }
        Ok(())
    }

    fn process_value(&self, value: &Values) -> Result<(), IpaError> {
        if let Some(ref package) = value.package {
            self.pacman.install(&package.name)?;
        }

        if let Some(ref link) = value.link {
            symlink(link)?;
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

    impl PackageManagement for FakePacman {
        fn install(&self, package: &str) -> Result<(), IpaError> {
            self.installed_packages
                .borrow_mut()
                .push(String::from(package));
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

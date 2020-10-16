use super::{config, error, PackageManagement};
use std::fs;
use std::os::unix;
use std::path::Path;

use config::{Config, SymLink, Values};
use error::IpaError;

pub struct Ipa<'a, P: PackageManagement> {
    config: Config,
    pacman: &'a P,
}

fn new_path<'a>(s: &str, out: &'a mut String) -> Result<&'a Path, IpaError> {
    let path = shellexpand::full(s)?;
    out.push_str(path.as_ref());
    Ok(Path::new(out))
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
            if package.name.is_empty() {
                return Err(IpaError::InvalidPackage);
            }
            println!("Installing package {}", package.name);
            self.pacman.install(&package.name)?;
        }

        if let Some(ref link) = value.link {
            self.process_link(link)?;
        }
        Ok(())
    }

    fn process_link(&self, link: &SymLink) -> Result<(), IpaError> {
        let mut src = String::new();
        let mut dst = String::new();
        self.symlink(
            new_path(&link.path, &mut src)?,
            new_path(&link.config, &mut dst)?,
            link.relink,
        )
    }
    fn symlink(&self, src: &Path, dst: &Path, relink: bool) -> Result<(), IpaError> {
        if src.is_dir() && dst.is_dir() {
            return self.symlink_dir(src, dst, relink);
        }

        if dst.exists() {
            if !relink {
                println!("Symbolic link {:?} already exists", dst);
                return Ok(());
            }
            println!("Relinking {:?}", dst);
            fs::remove_file(dst)?;
        }

        println!("Linking {:?} in {:?}", src, dst);
        unix::fs::symlink(src, dst)?;
        Ok(())
    }

    fn symlink_dir(&self, src: &Path, dst: &Path, relink: bool) -> Result<(), IpaError> {
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
    fn test_relink_links() {
        let src_dir = tempdir().unwrap();
        let dst_dir = tempdir().unwrap();

        let src_path_config = src_dir.path().join("src");
        std::fs::File::create(&src_path_config).unwrap();

        let dst_path_config = dst_dir.path().join("dst");
        std::fs::File::create(&dst_path_config).unwrap();

        let content = format!(
            "
dev:
  - link:
      config: {:?}
      path: {:?}
      relink: true
",
            dst_path_config, src_path_config,
        );

        let config = Config::new(&content).unwrap();
        let pacman = FakePacman::new();
        let ipa = Ipa::new(config, &pacman);

        ipa.setup().unwrap();

        let is_symlink = std::fs::symlink_metadata(dst_path_config.as_path())
            .unwrap()
            .file_type()
            .is_symlink();

        assert_eq!(true, is_symlink);
    }

    #[test]
    fn test_links() {
        let src_dir = tempdir().unwrap();
        let dst_dir = tempdir().unwrap();

        let src_config = NamedTempFile::new_in(&src_dir).unwrap();

        let content = format!(
            "
dev:
    - link:
        config: {:?}
        path: {:?}
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

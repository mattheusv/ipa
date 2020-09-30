use serde::{Deserialize, Serialize};
use std::fs;
use std::os::unix;
use std::path::Path;
pub mod error;
pub mod pacman;

pub trait PackageManagement {
    fn install(&self, name: &str) -> Result<(), error::IpaError>;
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct SymLink {
    config: String,

    path: String,

    #[serde(default)]
    relink: bool,

    #[serde(default)]
    group: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Package {
    #[serde(default)]
    name: String,

    #[serde(default)]
    link: SymLink,

    #[serde(default)]
    group: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    packages: Vec<Package>,

    #[serde(default)]
    link: Vec<SymLink>,
}

impl Config {
    pub fn new(content: &str) -> Result<Self, error::IpaError> {
        let config: Config = serde_yaml::from_str(&content)?;
        Ok(config)
    }
}

pub struct Ipa {
    config: Config,
    pacman: Box<dyn PackageManagement>,
}

fn new_path<'a>(s: &str, out: &'a mut String) -> Result<&'a Path, error::IpaError> {
    let path = shellexpand::full(s)?;
    out.push_str(path.as_ref());
    Ok(Path::new(out))
}

impl Ipa {
    pub fn new(config: Config, pacman: Box<dyn PackageManagement>) -> Self {
        Ipa { config, pacman }
    }

    pub fn from_file(config_yaml: &Path) -> Result<Self, error::IpaError> {
        let content = fs::read_to_string(config_yaml)?;

        let config = Config::new(&content)?;
        let pacman = Box::new(pacman::Pacman::new());

        Ok(Ipa { config, pacman })
    }

    fn filter_except_packages(&self, group: &str) -> Vec<&Package> {
        self.config
            .packages
            .iter()
            .filter(|p| p.group != group)
            .map(|p| p)
            .collect()
    }

    fn filter_packages(&self, group: &str) -> Vec<&Package> {
        self.config
            .packages
            .iter()
            .filter(|p| p.group == group)
            .map(|p| p)
            .collect()
    }

    fn filter_except_links(&self, group: &str) -> Vec<&SymLink> {
        self.config
            .link
            .iter()
            .filter(|l| l.group != group)
            .map(|l| l)
            .collect()
    }

    fn filter_links(&self, group: &str) -> Vec<&SymLink> {
        self.config
            .link
            .iter()
            .filter(|l| l.group == group)
            .map(|l| l)
            .collect()
    }

    fn process(
        &self,
        packages: &Vec<&Package>,
        links: &Vec<&SymLink>,
    ) -> Result<(), error::IpaError> {
        for package in packages.iter() {
            if package.name.is_empty() {
                return Err(error::IpaError::InvalidPackage);
            }

            self.pacman.install(&package.name)?;
            if !package.link.config.is_empty() && !package.link.path.is_empty() {
                let mut src = String::new();
                let mut dst = String::new();

                self.symlink(
                    new_path(&package.link.path, &mut src)?,
                    new_path(&package.link.config, &mut dst)?,
                    package.link.relink,
                )?;
            }
        }

        for link in links.iter() {
            let mut src = String::new();
            let mut dst = String::new();
            self.symlink(
                new_path(&link.path, &mut src)?,
                new_path(&link.config, &mut dst)?,
                link.relink,
            )?;
        }

        Ok(())
    }

    pub fn setup(&self) -> Result<(), error::IpaError> {
        self.process(
            &self.config.packages.iter().map(|p| p).collect(),
            &self.config.link.iter().map(|l| l).collect(),
        )
    }

    pub fn setup_group(&self, group: &str) -> Result<(), error::IpaError> {
        self.process(&self.filter_packages(group), &self.filter_links(group))
    }

    pub fn setup_except_group(&self, group: &str) -> Result<(), error::IpaError> {
        self.process(&self.filter_except_packages(group), &self.filter_except_links(group))
    }

    fn symlink_dir(&self, src: &Path, dst: &Path, relink: bool) -> Result<(), error::IpaError> {
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

    fn symlink(&self, src: &Path, dst: &Path, relink: bool) -> Result<(), error::IpaError> {
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::{tempdir, NamedTempFile};

    struct FakePacman {}

    impl PackageManagement for FakePacman {
        fn install(&self, _package: &str) -> Result<(), error::IpaError> {
            Ok(())
        }
    }

    #[test]
    fn filter_except_links() {
        let content = "
link:
    - config: ~/.testing/nvim
      path: ~/.testing/dotfiles/nvim
      group: dev

    - config: ~/.testing/alacritty
      path: ~/.testing/dotfiles/alacritty
      group: dev

    - config: ~/.testing/i3
      path: ~/.testing/dotfiles/i3
      group: gui
            ";
        let group = "gui";

        let ipa = Ipa::new(Config::new(content).unwrap(), Box::new(FakePacman {}));

        let packages = ipa.filter_except_links(&group);

        let alacritty = SymLink {
            config: String::from("~/.testing/alacritty"),
            path: String::from("~/.testing/dotfiles/alacritty"),
            relink: false,
            group: String::from("dev"),
        };
        let nvim = SymLink {
            config: String::from("~/.testing/nvim"),
            path: String::from("~/.testing/dotfiles/nvim"),
            relink: false,
            group: String::from("dev"),
        };

        assert_eq!(packages, vec![&nvim, &alacritty]);
    }

    #[test]
    fn filter_links() {
        let content = "
link:
    - config: ~/.testing/nvim
      path: ~/.testing/dotfiles/nvim
      group: dev

    - config: ~/.testing/alacritty
      path: ~/.testing/dotfiles/alacritty
      group: dev

    - config: ~/.testing/i3
      path: ~/.testing/dotfiles/i3
      group: gui
            ";
        let group = "gui";

        let ipa = Ipa::new(Config::new(content).unwrap(), Box::new(FakePacman {}));

        let packages = ipa.filter_links(&group);
        let i3 = SymLink {
            config: String::from("~/.testing/i3"),
            path: String::from("~/.testing/dotfiles/i3"),
            relink: false,
            group: String::from("gui"),
        };

        assert_eq!(packages, vec![&i3]);
    }

    #[test]
    fn filter_except_packages() {
        let content = "
packages:
    - name: nvim
      group: dev

    - name: alacritty
      group: dev

    - name: firefox
      group: gui
            ";
        let group = "gui";

        let ipa = Ipa::new(Config::new(content).unwrap(), Box::new(FakePacman {}));

        let packages = ipa.filter_packages(&group);
        let firefox = Package {
            name: String::from("firefox"),
            link: SymLink {
                config: String::new(),
                path: String::new(),
                relink: false,
                group: String::new(),
            },
            group: String::from("gui"),
        };

        assert_eq!(packages, vec![&firefox]);
    }


    #[test]
    fn filter_packages() {
        let content = "
packages:
    - name: nvim
      group: dev

    - name: alacritty
      group: dev

    - name: firefox
      group: gui
            ";
        let group = "dev";

        let ipa = Ipa::new(Config::new(content).unwrap(), Box::new(FakePacman {}));

        let packages = ipa.filter_packages(&group);
        let nvim = Package {
            name: String::from("nvim"),
            link: SymLink {
                config: String::new(),
                path: String::new(),
                relink: false,
                group: String::new(),
            },
            group: String::from("dev"),
        };

        let alacritty = Package {
            name: String::from("alacritty"),
            link: SymLink {
                config: String::new(),
                path: String::new(),
                relink: false,
                group: String::new(),
            },
            group: String::from("dev"),
        };

        assert_eq!(packages, vec![&nvim, &alacritty]);
    }

    #[test]
    fn install_and_setup_packages() {
        let src_dir = tempdir().unwrap();
        let dst_dir = tempdir().unwrap();

        let src_config = NamedTempFile::new_in(&src_dir).unwrap();

        let content = format!(
            "
packages:
    - name: foobar
      link:
        config: {:?}
        path: {:?}
",
            dst_dir.path(),
            src_dir.path(),
        );

        let result = Config::new(&content);
        assert!(result.is_ok());
        let config = result.unwrap();

        let ipa = Ipa::new(config, Box::new(FakePacman {}));

        assert!(ipa.setup().is_ok());

        let dst_config = dst_dir.path().join(src_config.path().file_name().unwrap());

        assert!(
            dst_config.as_path().exists(),
            "Assert that {:?} is created",
            dst_config.as_path()
        );
    }

    #[test]
    fn install_and_relink_packages() {
        let src_dir = tempdir().unwrap();
        let dst_dir = tempdir().unwrap();

        let src_path_config = src_dir.path().join("src");
        std::fs::File::create(&src_path_config).unwrap();

        let dst_path_config = dst_dir.path().join("dst");
        std::fs::File::create(&dst_path_config).unwrap();

        let content = format!(
            "
packages:
    - name: foobar
      link:
        config: {:?}
        path: {:?}
        relink: true
",
            dst_path_config, src_path_config,
        );

        let result = Config::new(&content);
        assert!(result.is_ok());

        let config = result.unwrap();
        let ipa = Ipa::new(config, Box::new(FakePacman {}));

        assert!(ipa.setup().is_ok());

        let is_symlink = std::fs::symlink_metadata(dst_path_config.as_path())
            .unwrap()
            .file_type()
            .is_symlink();

        assert_eq!(true, is_symlink);
    }
}

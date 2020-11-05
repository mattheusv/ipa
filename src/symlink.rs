use crate::error::IpaError;
use log::{debug, warn};
use serde::{Deserialize, Serialize};
use std::{fs, io, os::unix, path::Path};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct SymLink {
    pub dst: String,

    pub src: String,

    #[serde(default)]
    pub relink: bool,

    #[serde(default = "default_create")]
    pub create: bool,
}

impl SymLink {
    pub fn new(dst: &str, src: &str, relink: bool, create: bool) -> Self {
        SymLink {
            dst: dst.to_string(),
            src: src.to_string(),
            relink,
            create,
        }
    }
}

pub fn symlink(link: &SymLink) -> Result<(), IpaError> {
    let mut src = String::new();
    let mut dst = String::new();
    symlink_path(
        expand_path(&link.src, &mut src)?,
        expand_path(&link.dst, &mut dst)?,
        link.relink,
        link.create,
    )
}

fn symlink_path(src: &Path, dst: &Path, relink: bool, create: bool) -> Result<(), IpaError> {
    if !src.exists() {
        return Err(IpaError::Io(io::Error::new(
            io::ErrorKind::NotFound,
            format!("source file of link does not exists: {:?}", src),
        )));
    }

    if src.is_dir() && dst.is_dir() {
        return symlink_dir(src, dst, relink, create);
    }

    if dst.exists() {
        if !relink {
            warn!("Symbolic link {:?} already exists", dst);
            return Ok(());
        }
        warn!("Relinking {:?}", dst);
        fs::remove_file(dst)?;
    }

    if create {
        if let Some(parent) = dst.parent() {
            if !parent.exists() {
                debug!("Create destination sub directory {:?}", parent);
                fs::create_dir_all(parent)?;
            }
        }
    }

    debug!("Linking {:?} in {:?}", src, dst);
    unix::fs::symlink(src, dst)?;
    Ok(())
}

fn symlink_dir(src: &Path, dst: &Path, relink: bool, create: bool) -> Result<(), IpaError> {
    debug!("Create symbolic link to all files into {:?}", src);
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
                symlink_dir(entry.path().as_path(), dst_dir.as_path(), relink, create)?;
            }
        } else if let Some(name) = entry.path().file_name() {
            symlink_path(
                entry.path().as_path(),
                dst.join(name).as_path(),
                relink,
                create,
            )?;
        }
    }
    Ok(())
}

// Convert a path like `~/some/path/in/home` to `/home/user/some/path/in/home`
fn expand_path<'a>(s: &str, out: &'a mut String) -> Result<&'a Path, IpaError> {
    let path = shellexpand::full(s)?;
    out.push_str(path.as_ref());
    Ok(Path::new(out))
}

fn default_create() -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_create_dst_subdirectory() {
        let src_dir = tempdir().unwrap();
        let dst_dir = tempdir().unwrap();

        let src_path_config = src_dir.path().join("src");
        std::fs::File::create(&src_path_config).unwrap();

        let dst_path_config = dst_dir.path().join("dst/").join("file");

        let link = SymLink::new(
            &dst_path_config.to_str().unwrap(),
            &src_path_config.to_str().unwrap(),
            true,
            true,
        );

        assert!(symlink(&link).is_ok());
    }

    #[test]
    fn test_src_file_not_exist() {
        let link = SymLink::new("/tmp/src-invalid", "/tmp/dst-invalid", false, false);
        assert!(symlink(&link).is_err());
    }

    #[test]
    fn test_link_dir() {
        let src_dir = tempdir().unwrap();
        let dst_dir = tempdir().unwrap();

        let src_path_config = src_dir.path().join("src");
        std::fs::File::create(&src_path_config).unwrap();

        let link = SymLink::new(
            &dst_dir.path().to_str().unwrap(),
            &src_dir.path().to_str().unwrap(),
            true,
            false,
        );

        symlink(&link).unwrap();

        let dst_config = dst_dir.path().join(src_path_config.file_name().unwrap());

        assert!(
            dst_config.as_path().exists(),
            "Assert that {:?} is created",
            dst_config.as_path()
        );
    }

    #[test]
    fn test_relink_links() {
        let src_dir = tempdir().unwrap();
        let dst_dir = tempdir().unwrap();

        let src_path_config = src_dir.path().join("src");
        std::fs::File::create(&src_path_config).unwrap();

        let dst_path_config = dst_dir.path().join("dst");
        std::fs::File::create(&dst_path_config).unwrap();

        let link = SymLink::new(
            &dst_path_config.to_str().unwrap(),
            &src_path_config.to_str().unwrap(),
            true,
            false,
        );

        symlink(&link).unwrap();

        let is_symlink = std::fs::symlink_metadata(dst_path_config.as_path())
            .unwrap()
            .file_type()
            .is_symlink();

        assert_eq!(true, is_symlink);
    }

    #[test]
    fn test_not_relink_links() {
        let src_dir = tempdir().unwrap();
        let dst_dir = tempdir().unwrap();

        let src_path_config = src_dir.path().join("src");
        std::fs::File::create(&src_path_config).unwrap();

        let dst_path_config = dst_dir.path().join("dst");
        std::fs::File::create(&dst_path_config).unwrap();

        let link = SymLink::new(
            &dst_path_config.to_str().unwrap(),
            &src_path_config.to_str().unwrap(),
            false,
            false,
        );

        symlink(&link).unwrap();

        let is_symlink = std::fs::symlink_metadata(dst_path_config.as_path())
            .unwrap()
            .file_type()
            .is_symlink();

        assert_eq!(false, is_symlink);
    }
}

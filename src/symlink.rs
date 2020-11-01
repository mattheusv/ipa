use super::error::IpaError;
use serde::{Deserialize, Serialize};
use std::fs;
use std::os::unix;
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

pub fn symlink(link: &SymLink) -> Result<(), IpaError> {
    let mut src = String::new();
    let mut dst = String::new();
    symlink_path(
        expand_path(&link.src, &mut src)?,
        expand_path(&link.dst, &mut dst)?,
        link.relink,
    )
}

fn symlink_path(src: &Path, dst: &Path, relink: bool) -> Result<(), IpaError> {
    if src.is_dir() && dst.is_dir() {
        return symlink_dir(src, dst, relink);
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

fn symlink_dir(src: &Path, dst: &Path, relink: bool) -> Result<(), IpaError> {
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
                symlink_dir(entry.path().as_path(), dst_dir.as_path(), relink)?;
            }
        } else {
            if let Some(name) = entry.path().file_name() {
                symlink_path(entry.path().as_path(), dst.join(name).as_path(), relink)?;
            }
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

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

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
        );

        symlink(&link).unwrap();

        let is_symlink = std::fs::symlink_metadata(dst_path_config.as_path())
            .unwrap()
            .file_type()
            .is_symlink();

        assert_eq!(false, is_symlink);
    }
}

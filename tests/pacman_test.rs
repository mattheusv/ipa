use ipa::{config, pacman, runner};
use std::env::current_dir;
use std::fs::{remove_dir_all, remove_file, File};
use std::io::prelude::*;
use std::path::Path;

#[test]
fn test_setup_pacman() {
    let mut file = File::create("/tmp/src.txt").unwrap();
    file.write_all(b"Hello, ipa!").unwrap();

    let config = config::Config::load(
        current_dir()
            .unwrap()
            .join("tests/config.test.yml")
            .as_path(),
    )
    .unwrap();
    let pacman = pacman::archlinux::Pacman::new();
    let ipa = runner::Ipa::new(config, &pacman);

    assert!(ipa.setup().is_ok());
    assert!(pacman.is_installed("fzf").unwrap());
    assert!(pacman.is_installed("tmux").unwrap());
    assert!(Path::new("/tmp/dst.txt").exists());
    assert!(Path::new("/tmp/tmux-plugins/").exists());
    assert!(Path::new("/tmp/i3blocks/").exists());

    // Clean files
    remove_file("/tmp/dst.txt").unwrap();
    remove_dir_all("/tmp/tmux-plugins/").unwrap();
    remove_dir_all("/tmp/i3blocks/").unwrap();
}

use ipa::{config, pacman, runner};
use std::env::current_dir;
use std::path::Path;

#[test]
fn test_setup_pacman() {
    let config = config::Config::load(
        current_dir()
            .unwrap()
            .join("tests/config.test.yml")
            .as_path(),
    )
    .unwrap();
    let pacman = pacman::Pacman::new();
    let ipa = runner::Ipa::new(config, &pacman);

    assert!(ipa.setup().is_ok());
    assert!(pacman.is_installed("fzf").unwrap());
    assert!(pacman.is_installed("tmux").unwrap());
    assert!(Path::new("/tmp/bashrc.tmp").exists());
    assert!(Path::new("/tmp/tmux-plugins/").exists());
    assert!(Path::new("/tmp/i3blocks/").exists());
}

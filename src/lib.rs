pub mod cli;
pub mod config;
pub mod error;
pub mod pacman;
pub mod runner;
pub mod shell;
pub mod symlink;

use error::IpaError;

pub trait PackageManagement {
    fn install(&self, name: &str) -> Result<(), IpaError>;
}

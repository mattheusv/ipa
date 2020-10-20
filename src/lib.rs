pub mod config;
pub mod error;
pub mod runner;
pub mod symlink;

use error::IpaError;

pub trait PackageManagement {
    fn install(&self, name: &str) -> Result<(), IpaError>;
}

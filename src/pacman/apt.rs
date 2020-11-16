use crate::pacman::{Error, Package, PackageManagement};

pub struct Apt;

impl Apt {
    pub fn new() -> Self {
        Apt
    }
}

impl PackageManagement for Apt {
    fn install(&self, _package: &Package) -> Result<(), Error> {
        Ok(())
    }
}

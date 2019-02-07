use crate::os;
use crate::property::Property;
use crate::PrResult;
use std::fmt;
use std::io;
use std::process;

/// packages are installed by pacman
#[derive(Clone)]
pub struct PacmanInstalled {
    pub packages: Vec<&'static str>,
}

/// packages are not installed by pacman
#[derive(Clone)]
pub struct PacmanRemoved {
    pub packages: Vec<&'static str>,
}

impl fmt::Display for PacmanInstalled {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.packages.len() == 1 {
            write!(f, "package {} is installed by pacman", self.packages[0])?;
        } else {
            write!(f, "packages {:?} are installed by pacman", self.packages)?;
        }

        Ok(())
    }
}

impl Property<os::ArchLinux> for PacmanInstalled {
    fn check(&self) -> PrResult<bool> {
        for package in &self.packages {
            let s = process::Command::new("pacman")
                // This option retrieves a list of the files installed by a package
                .arg("-Ql")
                .arg(package)
                .status()?;
            if !s.success() {
                return Ok(false);
            }
        }

        Ok(true)
    }

    fn apply(&self) -> PrResult<()> {
        let s = process::Command::new("pacman")
            .arg("-S")
            .arg("--needed")
            .arg("--noconfirm")
            .args(&self.packages)
            .status()?;
        if !s.success() {
            Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("Packages {:?} not installed successfully", self.packages),
            ))
        } else {
            Ok(())
        }
    }
}

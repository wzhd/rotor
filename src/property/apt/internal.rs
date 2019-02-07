use crate::os;
use crate::property::Property;
use crate::PrResult;
use std::fmt;
use std::io;
use std::process;

/// packages are installed by pacman
#[derive(Clone)]
pub struct AptInstalled {
    pub packages: Vec<&'static str>,
}

#[derive(Clone)]
pub struct AptRemoved {
    pub packages: Vec<&'static str>,
}

impl fmt::Display for AptInstalled {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.packages.len() == 1 {
            write!(f, "package {} is installed by apt", self.packages[0])?;
        } else {
            write!(f, "packages {:?} are installed by apt", self.packages)?;
        }

        Ok(())
    }
}

impl Property<os::DebianLike> for AptInstalled {
    fn check(&self) -> PrResult<bool> {
        let ps = &self.packages;
        let out = process::Command::new("apt-cache")
            .env("LANG", "C")
            .arg("policy")
            .args(ps)
            .output()?
            .stdout;
        let out =
            String::from_utf8(out).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        let is: Vec<bool> = out
            .lines()
            .filter_map(|l| {
                if l.contains("Installed: (none)") {
                    Some(false)
                } else if l.contains("Installed: ") {
                    Some(true)
                } else {
                    None
                }
            })
            .collect();
        if is.iter().all(|&i| i) && is.len() == ps.len() {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn apply(&self) -> PrResult<()> {
        let s = process::Command::new("apt-get")
            .arg("--assume-yes")
            .arg("install")
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

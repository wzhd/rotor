use crate::os;
use crate::property::Property;
use crate::PrResult;
use std::fmt;
use std::io;
use std::process;

pub struct GitGlobalConfigKey {
    pub key: &'static str,
}

#[derive(Clone)]
pub struct GitGlobalConfig {
    key: &'static str,
    value: &'static str,
}

impl GitGlobalConfigKey {
    pub fn value(&self, value: &'static str) -> GitGlobalConfig {
        GitGlobalConfig {
            key: self.key,
            value,
        }
    }
}

impl fmt::Display for GitGlobalConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "git global config {}={}", self.key, self.value)?;
        Ok(())
    }
}

impl Property<os::Any> for GitGlobalConfig {
    fn check(&self) -> PrResult<bool> {
        let mut out = process::Command::new("git")
            .arg("config")
            .arg("--null")
            .arg("--global")
            .arg(self.key)
            .output()?
            .stdout;
        let null_end = out.last() == Some(&0);
        if null_end {
            out.truncate(out.len() - 1);
        }
        let s =
            String::from_utf8(out).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        if s == self.value {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn apply(&self) -> PrResult<()> {
        let s = process::Command::new("git")
            .arg("config")
            .arg("--global")
            .arg(self.key)
            .arg(self.value)
            .status()?;
        if !s.success() {
            Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!(
                    "git global config {}={} not set successfully",
                    self.key, self.value
                ),
            ))
        } else {
            Ok(())
        }
    }
}

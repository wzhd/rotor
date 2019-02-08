use dirs::home_dir;
use std::fmt;
use std::io;
use std::path::PathBuf;

pub mod cmd;

/// Relative paths are considered relative to user's home directory
#[derive(Clone)]
pub enum UserPathBuf {
    Home(PathBuf),
    Absolute(PathBuf),
}

impl fmt::Debug for UserPathBuf {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            UserPathBuf::Home(p) => {
                write!(f, "~/{:?}", p)?;
            }
            UserPathBuf::Absolute(p) => write!(f, "{:?}", p)?,
        }
        Ok(())
    }
}

impl<P: Into<PathBuf>> From<P> for UserPathBuf {
    fn from(p: P) -> Self {
        let p = p.into();
        if p.is_relative() {
            UserPathBuf::Home(p)
        } else {
            UserPathBuf::Absolute(p)
        }
    }
}

impl UserPathBuf {
    pub fn expand_user(&self) -> io::Result<PathBuf> {
        match self {
            UserPathBuf::Absolute(p) => Ok(p.clone()),
            UserPathBuf::Home(p) => {
                let home = home_dir().ok_or_else(|| {
                    io::Error::new(io::ErrorKind::NotFound, "User home not found.")
                })?;
                Ok(home.join(p))
            }
        }
    }
}

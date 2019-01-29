use std::io;
use dirs::home_dir;
use std::path::{PathBuf, Path};

pub fn home<P: AsRef<Path>>(rel: P) -> io::Result<PathBuf> {
    let home = home_dir().ok_or_else(|| {
        io::Error::new(io::ErrorKind::NotFound, "User home not found.")
    })?;
    let rel = rel.as_ref();
    if rel.is_absolute() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput,
                                  "Not a relative path in home"));
    }
    let path = home.join(rel);
    Ok(path)
}

mod internal;

use self::internal::{PacmanInstalled, PacmanRemoved};

/// package is installed by pacman
pub fn installed(package: &'static str) -> PacmanInstalled {
    installed_all(&[package])
}

/// packages are installed by pacman
pub fn installed_all(packages: &[&'static str]) -> PacmanInstalled {
    let packages = packages.to_vec();
    PacmanInstalled { packages }
}

/// Package is not installed by pacman.
pub fn removed(package: &'static str) -> PacmanRemoved {
    removed_all(&[package])
}

/// Packages are not installed by pacman.
pub fn removed_all(packages: &[&'static str]) -> PacmanRemoved {
    let packages = packages.to_vec();
    PacmanRemoved { packages }
}

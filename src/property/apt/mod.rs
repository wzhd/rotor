//! Manage packages on Debian and derivatives
//! apt-get and dpkg may be invoked

mod internal;

use self::internal::{AptInstalled, AptRemoved};

pub fn installed(package: &'static str) -> AptInstalled {
    installed_all(&[package])
}

pub fn installed_all(packages: &[&'static str]) -> AptInstalled {
    let packages = packages.to_vec();
    AptInstalled { packages }
}

pub fn removed(package: &'static str) -> AptRemoved {
    removed_all(&[package])
}

pub fn removed_all(packages: &[&'static str]) -> AptRemoved {
    let packages = packages.to_vec();
    AptRemoved { packages }
}

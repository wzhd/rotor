//! Properties related to a directory

use crate::property::dir::internal::ManagedDir;
use crate::util::UserPathBuf;
use std::sync::Arc;

mod internal;

use self::internal::PackageLinked;

#[allow(dead_code)]
pub fn path<P: Into<UserPathBuf>>(path: P) -> ManagedDir {
    let path = path.into();
    let path = Arc::new(path);
    ManagedDir { path }
}

impl ManagedDir {
    /// Treat the subdirectories as packages, which are structured collections of files, which can be mirrored
    /// to a target directory using symlinks, preserving the structure.
    /// It can be used to manage the installation of multiple software packages in the same run-time directory tree.
    /// It can be used to manage dotfiles in the home directory.
    /// Similar to GNU Stow to some extent.
    #[allow(dead_code)]
    pub fn as_package_source(&self) -> PackageLinked {
        PackageLinked::new(self.path.clone())
    }
}

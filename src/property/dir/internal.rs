use crate::os::Any;
use crate::property::Property;
use crate::util::UserPathBuf;
use crate::PrResult;
use std::fmt;
use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;

use pathdiff::diff_paths;
use same_file::is_same_file;
use walkdir::WalkDir;

/// A dir with some properties managed
pub struct ManagedDir {
    pub path: Arc<UserPathBuf>,
}

/// Treat the subdirectories as packages, which are structured
/// collections of files, which can be mirrored to a target
/// directory using symlinks, preserving the structure.
#[derive(Clone)]
pub struct PackageLinked {
    /// Directory containing packages, which contain
    /// directories and regular files.
    pub repo: Arc<UserPathBuf>,
    /// Directory where directories and symlinks should
    /// be created.
    install: Option<UserPathBuf>,
    linked: Vec<&'static str>,
    unlinked: Vec<&'static str>,
}

impl PackageLinked {
    pub fn new(path: Arc<UserPathBuf>) -> PackageLinked {
        // The default target directory is the parent directory
        let target = path.parent();
        PackageLinked {
            repo: path,
            install: target,
            linked: vec![],
            unlinked: vec![],
        }
    }

    /// Set the directory where directories and symlinks
    /// should be created. The default is the parent of
    /// the repo directory.
    #[allow(dead_code)]
    pub fn install_to<P: Into<UserPathBuf>>(mut self, target: P) -> PackageLinked {
        let target = target.into();
        self.install = Some(target);
        self
    }

    #[allow(dead_code)]
    /// Add a package to be linked.
    pub fn linked(self, package: &'static str) -> PackageLinked {
        self.linked_all(&[package])
    }

    /// Add a package to be unlinked.
    #[allow(dead_code)]
    pub fn unlinked(self, package: &'static str) -> PackageLinked {
        self.unlinked_all(&[package])
    }

    /// Add packages to be linked.
    pub fn linked_all(mut self, packages: &[&'static str]) -> PackageLinked {
        self.linked.extend_from_slice(packages);
        self
    }

    /// Add packages to be unlinked.
    pub fn unlinked_all(mut self, packages: &[&'static str]) -> PackageLinked {
        self.unlinked.extend_from_slice(packages);
        self
    }

    fn get_repo_dir(&self) -> io::Result<PathBuf> {
        self.repo.expand_user()?.canonicalize()
    }

    fn get_install_dir(&self) -> io::Result<PathBuf> {
        if let Some(p) = &self.install {
            p.expand_user()
        } else {
            Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Target not defined.",
            ))
        }
    }
}

impl Property<Any> for PackageLinked {
    fn check(&self) -> PrResult<bool> {
        let source = self.get_repo_dir()?;
        let target = self.get_install_dir()?;
        for &package in &self.linked {
            if !check_linked(&source, &target, package)? {
                return Ok(false);
            }
        }
        for &package in &self.unlinked {
            if !check_unlinked(&source, &target, package)? {
                return Ok(false);
            }
        }
        Ok(true)
    }

    fn apply(&self) -> PrResult<()> {
        let source = self.get_repo_dir()?;
        let target = self.get_install_dir()?;
        for &package in &self.linked {
            if !check_linked(&source, &target, package)? {
                println!("Linking package {}", package);
                link_package(&source, &target, package)?;
            }
        }
        for &package in &self.unlinked {
            if !check_unlinked(&source, &target, package)? {
                println!("Unlinking package {}", package);
                unlink_package(&source, &target, package)?
            }
        }
        Ok(())
    }
}

/// For every regular file in the package, check whether there is a
/// symlink pointing to it correctly. Directories must exist with the
/// same structure.
/// Paths must be both absolute.
fn check_linked(repo: &Path, linked: &Path, package: &str) -> io::Result<bool> {
    let pkg_dir = repo.join(package);
    for entry in WalkDir::new(&pkg_dir) {
        let entry = entry?;
        let pkg_path = entry.path();
        // Relative path. Should be the same for the file relative
        // to the package directory and the link relative to the
        // installation directory.
        let rel_path = diff_paths(pkg_path, &pkg_dir)
            .expect("Can't calculate relative path; arguments are not both absolute paths");
        let link_path = linked.join(&rel_path);
        if !link_path.exists() {
            return Ok(false);
        }
        let file_meta = entry.metadata()?;
        if file_meta.is_file() {
            let is_link = link_path.symlink_metadata()?.file_type().is_symlink();
            let is_correct = is_link && is_same_file(pkg_path, link_path)?;
            if !is_correct {
                return Ok(false);
            }
        } else if file_meta.is_dir() {
            if !link_path.is_dir() {
                return Ok(false);
            }
        } else {
            // To simplify
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Package must contain only files and directories; symlink is not supported.",
            ));
        }
    }
    Ok(true)
}

/// For every regular file in the package, make sure there are no
/// symlinks pointing to it. Directories can exist within the
/// installation path because they may be shared.
/// Paths must be both absolute.
fn check_unlinked(repo: &Path, install: &Path, package: &str) -> io::Result<bool> {
    let pkg_dir = repo.join(package);
    for entry in WalkDir::new(&pkg_dir) {
        let entry = entry?;
        let pkg_path = entry.path();
        // Relative path. Should be the same for the file relative
        // to the package directory and the link relative to the
        // installation directory.
        let rel_path = diff_paths(pkg_path, &pkg_dir)
            .expect("Can't calculate relative path; arguments are not both absolute paths");
        let link_path = install.join(&rel_path);
        if !link_path.exists() {
            continue;
        }
        let file_meta = entry.metadata()?;
        if file_meta.is_file() {
            if is_same_file(pkg_path, link_path)? {
                return Ok(false);
            }
        } else if !file_meta.is_dir() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Package must contain only files and directories; symlink is not supported.",
            ));
        }
    }
    Ok(true)
}

fn link_package(repo: &Path, install: &Path, package: &str) -> io::Result<()> {
    let pkg_dir = repo.join(package);
    // By default directories are yielded before their contents.
    for entry in WalkDir::new(&pkg_dir) {
        let entry = entry?;
        let pkg_path = entry.path();
        // Relative path to bases.
        let rel_path = diff_paths(pkg_path, &pkg_dir)
            .expect("Can't calculate relative path; arguments are not both absolute paths");
        let link_path = install.join(&rel_path);

        let file_meta = entry.metadata()?;
        if file_meta.is_file() {
            match link_path.symlink_metadata() {
                Ok(meta) => {
                    if meta.file_type().is_symlink() {
                        if link_path.exists() {
                            // Non-broken symlink
                            if is_same_file(&link_path, &pkg_path)? {
                                continue;
                            }
                        // It's okay if it doesn't point to the file in this package.
                        // The symlink may belong to another package.
                        } else {
                            return Err(io::Error::new(
                                io::ErrorKind::AlreadyExists,
                                format!("{:?} is a broken symlink", link_path),
                            ));
                        }
                    } else {
                        return Err(io::Error::new(
                            io::ErrorKind::AlreadyExists,
                            format!(
                                "{:?} exists, can't create symlink for {}",
                                link_path, package
                            ),
                        ));
                    }
                }
                Err(e) => {
                    if e.kind() == io::ErrorKind::NotFound {
                        let link_dir = link_path
                            .parent()
                            .expect("No directory where symlink should be created");
                        let rel_link = diff_paths(pkg_path, &link_dir)
                            .expect("Can't calculate relative path for symlink creation");
                        symlink_file(rel_link, link_path)?;
                    } else {
                        return Err(e);
                    }
                }
            }
        } else if file_meta.is_dir() {
            if !link_path.is_dir() {
                println!("Creating directory {:?} in package {}", rel_path, package);
                fs::create_dir(&link_path)?;
            }
        } else {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Package must contain only files and directories; symlink is not supported.",
            ));
        }
    }
    Ok(())
}

fn unlink_package(repo: &Path, install: &Path, package: &str) -> io::Result<()> {
    let pkg_dir = repo.join(package);
    for entry in WalkDir::new(&pkg_dir) {
        let entry = entry?;
        let pkg_path = entry.path();
        let rel_path = diff_paths(pkg_path, &pkg_dir)
            .expect("Can't calculate relative path; arguments are not both absolute paths");
        let link_path = install.join(&rel_path);

        let file_meta = entry.metadata()?;
        if file_meta.is_file() {
            match link_path.symlink_metadata() {
                Ok(meta) => {
                    if meta.file_type().is_symlink() && is_same_file(&link_path, &pkg_path)? {
                        // Remove link only when it points to the corresponding file in the package
                        fs::remove_file(&link_path)?;
                    }
                }
                Err(e) => {
                    if e.kind() != io::ErrorKind::NotFound {
                        return Err(e);
                    }
                }
            }
        } else if !file_meta.is_dir() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Package must contain only files and directories.",
            ));
        }
    }
    Ok(())
}

#[cfg(unix)]
/// The dst path will be a symbolic link pointing to the src path
fn symlink_file<P: AsRef<Path>, Q: AsRef<Path>>(src: P, dst: Q) -> io::Result<()> {
    use std::os::unix::fs::symlink;
    symlink(src, dst)
}

#[cfg(windows)]
/// The dst path will be a symbolic link pointing to the src path
fn symlink_file<P: AsRef<Path>, Q: AsRef<Path>>(src: P, dst: Q) {
    use std::os::windows::fs::symlink_file;
    symlink_file(src, dst)
}

impl fmt::Display for PackageLinked {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let install_dir = if let Some(ref p) = &self.install {
            format!("{:?}", p)
        } else {
            "<undefined>".to_string()
        };
        write!(
            f,
            "Packages {:?} are linked from {:?} ",
            self.linked, self.repo
        )?;
        write!(f, "to {}; ", install_dir)?;
        write!(f, "packages {:?} are not linked", self.unlinked)?;
        Ok(())
    }
}

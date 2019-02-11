use std::fmt::Display;

pub mod apt;
pub mod conf_file;
mod core;
pub mod dir;
mod file;
pub mod git;
pub mod pacman;

pub use self::core::{prop, PropertyList};
pub use self::file::file;

use self::core::PropertyClone;
use super::types::os::{self, OS};
use crate::PrResult;

pub trait Property<O: OS>: Display + PropertyClone<O> {
    fn check(&self) -> PrResult<bool>;
    fn apply(&self) -> PrResult<()>;
}

// Properties that apply to any OS also apply to Linux
impl<T> Property<os::Linux> for T
where
    T: Property<os::Any> + Clone + 'static,
{
    fn check(&self) -> PrResult<bool> {
        T::check(self)
    }
    fn apply(&self) -> PrResult<()> {
        T::apply(self)
    }
}

// Properties that apply to any Linux also apply to ArchLinux
impl<T> Property<os::ArchLinux> for T
where
    T: Property<os::Linux> + Clone + 'static,
{
    fn check(&self) -> PrResult<bool> {
        T::check(self)
    }
    fn apply(&self) -> PrResult<()> {
        T::apply(self)
    }
}

// Properties that apply to any Linux also apply to Debian derivatives
impl<T> Property<os::DebianLike> for T
where
    T: Property<os::Linux> + Clone + 'static,
{
    fn check(&self) -> PrResult<bool> {
        T::check(self)
    }
    fn apply(&self) -> PrResult<()> {
        T::apply(self)
    }
}

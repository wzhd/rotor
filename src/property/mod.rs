use std::fmt::Display;
use std::io;

mod conf_file;
pub mod file;

pub use self::conf_file::conf_file;
pub use super::types::os::{self, OS};

pub type PrResult<T> = io::Result<T>;

pub trait Property<O: OS>: Display + PropertyClone<O> {
    fn check(&self) -> PrResult<bool>;
    fn apply(&self) -> PrResult<()>;
}


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

/// Workaround
pub trait PropertyClone<O> {
    fn clone_box(&self) -> Box<Property<O>>;
}

impl<P: 'static + Property<O> + Clone, O: OS> PropertyClone<O> for P {
    fn clone_box(&self) -> Box<Property<O>> {
        Box::new(self.clone())
    }
}

impl<O: OS> Clone for Box<Property<O>> {
    fn clone(&self) -> Box<Property<O>> {
        self.clone_box()
    }
}

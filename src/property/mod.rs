use std::fmt::Display;
use std::io;

mod conf_file;
pub mod file;

pub use self::conf_file::conf_file;

pub type PrResult<T> = io::Result<T>;

pub trait Property: Display {
    fn check(&self) -> PrResult<bool>;
    fn apply(&self) -> PrResult<()>;
}

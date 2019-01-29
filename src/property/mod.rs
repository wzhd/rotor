use std::io;
use std::fmt::Display;

pub mod file;

pub type PrResult<T> = io::Result<T>;

pub trait Property: Display {
    fn check(&self) -> PrResult<bool>;
    fn apply(&self) -> PrResult<()>;
}
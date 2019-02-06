//! Combine properties

mod clone;
mod list;

pub use self::clone::PropertyClone;
pub use self::list::PropertyList;
use crate::os::OS;

pub fn prop<O: OS>() -> PropertyList<O> {
    PropertyList::new()
}

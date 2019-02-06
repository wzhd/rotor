//! Combine properties

use crate::property::OS;

mod list;

pub use self::list::PropertyList;

pub fn prop<O: OS>() -> PropertyList<O> {
    PropertyList::new()
}

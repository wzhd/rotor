//! A list of properties to be applied sequentially

use crate::os::OS;
use crate::property::Property;
use std::ops::Add;

#[derive(Default)]
pub struct PropertyList<O: OS> {
    pub properties: Vec<Box<dyn Property<O>>>,
}

impl<O: OS, P: Property<O> + 'static> Add<P> for PropertyList<O> {
    type Output = PropertyList<O>;

    fn add(mut self, rhs: P) -> Self::Output {
        self.properties.push(Box::new(rhs));
        self
    }
}

//! A list of properties to be applied sequentially

use crate::property::Property;
use crate::property::OS;
use std::ops::Add;

pub struct PropertyList<O: OS> {
    pub properties: Vec<Box<dyn Property<O>>>,
}

impl<O: OS> PropertyList<O> {
    pub fn new() -> PropertyList<O> {
        PropertyList { properties: vec![] }
    }
}

impl<O: OS, P: Property<O> + 'static> Add<P> for PropertyList<O> {
    type Output = PropertyList<O>;

    fn add(mut self, rhs: P) -> Self::Output {
        self.properties.push(Box::new(rhs));
        self
    }
}

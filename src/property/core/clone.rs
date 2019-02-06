//! Workaround to make cloning boxed property possible

use crate::os::OS;
use crate::property::Property;

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

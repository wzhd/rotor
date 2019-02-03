use super::effect::Task;
use crate::property::Property;
use crate::types::os::OS;
use std::fmt;
use std::marker::PhantomData;

#[allow(dead_code)]
pub struct UserHost<T: OS> {
    user: String,
    host: String,
    phantom: PhantomData<T>,
}

impl<T: OS> fmt::Display for UserHost<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}@{}", self.user, self.host)
    }
}

impl<T: OS> UserHost<T> {
    #[allow(dead_code)]
    pub fn new<S: Into<String>>(user: S, host: S) -> UserHost<T> {
        let user = user.into();
        let host = host.into();

        UserHost {
            user,
            host,
            phantom: PhantomData,
        }
    }

    pub fn properties(self, properties: &[Box<dyn Property<T>>]) -> Task<T> {
        Task::new(self, properties)
    }
}

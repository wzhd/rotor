use super::effect::Task;
use crate::property::Property;
use std::fmt;

#[allow(dead_code)]
pub struct UserHost {
    user: String,
    host: String,
}

impl fmt::Display for UserHost {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}@{}", self.user, self.host)
    }
}

impl UserHost {
    #[allow(dead_code)]
    pub fn new<S: Into<String>>(user: S, host: S) -> UserHost {
        let user = user.into();
        let host = host.into();
        UserHost { user, host }
    }

    #[allow(dead_code)]
    pub fn apply(self, properties: &[Box<Property>]) -> Task {
        Task::new(self, properties)
    }
}

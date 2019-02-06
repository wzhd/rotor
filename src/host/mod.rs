use super::effect::Task;
use crate::property::Property;
use crate::types::os::OS;
use std::fmt;
use std::io;
use std::marker::PhantomData;

mod user;
pub use self::user::user;
pub use self::user::HostUsersConf;
use crate::effect::Runnable;
use crate::property::PrResult;

pub trait ConfigureUser {
    fn list_users(&self) -> Vec<&str>;
    fn configure(&self, user_name: &str) -> PrResult<()>;
}

impl<O: OS> ConfigureUser for HostUsersConf<O> {
    fn list_users(&self) -> Vec<&str> {
        self.users.iter().map(|u| u.name.as_ref()).collect()
    }

    fn configure(&self, user_name: &str) -> PrResult<()> {
        for user in &self.users {
            if user.name == user_name {
                return user.properties.run();
            }
        }
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("User {} not configured", user_name),
        ));
    }
}
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

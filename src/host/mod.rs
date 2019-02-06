use crate::types::os::OS;
use std::io;

mod user;
pub use self::user::user;
pub use self::user::HostUsersConf;
use crate::effect::Runnable;
use crate::PrResult;

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
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("User {} not configured", user_name),
        ))
    }
}

use crate::types::os::OS;
use std::io;

mod user;
pub use self::user::user;
pub use self::user::HostUsersConf;
use crate::effect::Runnable;
use crate::PrResult;
use std::fmt;
use std::str::FromStr;

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

pub struct UserAtHost {
    pub user: String,
    pub host: String,
}

impl fmt::Display for UserAtHost {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}@{}", self.user, self.host)
    }
}

impl fmt::Debug for UserAtHost {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}@{}", self.user, self.host)
    }
}

impl FromStr for UserAtHost {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('@').collect();
        if parts.len() != 2 {
            return Err("Need username@hostname");
        }
        let user = parts[0].to_string();
        if user.is_empty() {
            return Err("Need username before @");
        }
        let host = parts[1].to_string();
        if host.is_empty() {
            return Err("Need hostname after @");
        }
        Ok(UserAtHost { user, host })
    }
}

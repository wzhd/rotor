mod effect;
mod host;
pub mod property;
mod types;
mod util;

pub use self::property::prop;
pub use self::types::os;

pub use self::host::user;
use self::host::ConfigureUser;
use self::host::HostUsersConf;
use std::collections::HashMap;
use std::io;

pub type PrResult<T> = io::Result<T>;

pub struct RotorBuilder {
    hosts: HashMap<String, Box<dyn ConfigureUser>>,
}

impl RotorBuilder {
    pub fn new() -> RotorBuilder {
        RotorBuilder {
            hosts: HashMap::new(),
        }
    }

    pub fn host<O: os::OS + 'static>(
        mut self,
        hostname: &'static str,
        users_conf: HostUsersConf<O>,
    ) -> RotorBuilder {
        let host = Box::new(users_conf);
        self.hosts.insert(hostname.to_string(), host);
        self
    }

    pub fn configure_user(&self, username: &str, hostname: &str) -> PrResult<()> {
        let host = self
            .hosts
            .get(hostname)
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "host not configured"))?;
        host.configure(username)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

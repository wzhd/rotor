mod effect;
mod host;
pub mod property;
mod types;
mod util;

pub use self::host::UserHost;
pub use self::property::prop;
pub use self::types::os::*;

pub use self::effect::Runnable;
pub use self::host::user;
use self::host::ConfigureUser;
use self::host::HostUsersConf;
use self::property::PrResult;
use std::collections::HashMap;
use std::io;

pub fn default_main(tasks: Vec<Box<dyn Runnable>>) -> PrResult<()> {
    for task in tasks.iter() {
        if let Err(_e) = task.run() {}
    }
    Ok(())
}

pub struct RotorBuilder {
    hosts: HashMap<String, Box<dyn ConfigureUser>>,
}

impl RotorBuilder {
    pub fn new() -> RotorBuilder {
        RotorBuilder {
            hosts: HashMap::new(),
        }
    }

    pub fn host<O: OS + 'static>(
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

#[macro_export]
macro_rules! vec_box {
    ($($x:expr),*) => (
        <[_]>::into_vec(Box::new([$(Box::new($x)),*]))
    );
    ($($x:expr,)*) => (vec_box![$($x),*])
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

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
use self::util::cmd::{RotorMain, RotorSub};
use std::collections::HashMap;
use std::io;
use structopt::StructOpt;

pub type PrResult<T> = io::Result<T>;

#[derive(Default)]
pub struct RotorBuilder {
    hosts: HashMap<String, Box<dyn ConfigureUser>>,
}

impl RotorBuilder {
    pub fn new() -> RotorBuilder {
        Default::default()
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

    /// Parse command-line arguments and run
    pub fn run(&self) {
        let opt = RotorMain::from_args();
        match opt.cmd {
            RotorSub::List => {
                println!("host  user\n");
                for (host, user_confs) in self.hosts.iter() {
                    println!("{}", host);
                    for user in user_confs.list_users() {
                        println!("      {}", user);
                    }
                }
            }
            RotorSub::Apply { ref user } => {
                println!("Configuring as {}", user);
                if let Err(e) = self.configure_user(&user.user, &user.host) {
                    eprintln!("{} not configured correctly: {:?}", user, e);
                }
            }
            RotorSub::Push { ref targets } => {
                println!("Pushing configurations to {:?}", targets);
                eprintln!("This option is under construction.");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

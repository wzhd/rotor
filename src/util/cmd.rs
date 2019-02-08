use super::super::host::UserAtHost;
use std::str::FromStr;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "rotor", about = "Deploying properties to hosts.")]
pub struct RotorMain {
    #[structopt(subcommand)]
    pub cmd: RotorSub,
}

#[derive(Debug, StructOpt)]
pub enum RotorSub {
    #[structopt(name = "list")]
    /// List configured users at hosts
    List,
    /// Apply configurations for username@hostname locally
    #[structopt(name = "apply")]
    Apply {
        #[structopt(parse(try_from_str))]
        user: UserAtHost,
    },
    /// Apply configurations to remote users or hosts via ssh
    #[structopt(name = "push")]
    Push { targets: Vec<PushTarget> },
}

#[derive(Debug)]
pub enum PushTarget {
    /// A single user on a single host
    User(UserAtHost),
    /// All users on a host
    Host(String),
}

impl FromStr for PushTarget {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.contains('@') {
            Ok(PushTarget::Host(s.to_string()))
        } else {
            let u = FromStr::from_str(s)?;
            Ok(PushTarget::User(u))
        }
    }
}

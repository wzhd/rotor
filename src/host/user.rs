use crate::property::OS;

use super::super::property::PropertyList;

pub struct HostUsersConf<O: OS> {
    pub users: Vec<UserConf<O>>,
}

/// Add configuration for a user
pub fn user<O: OS>(name: &'static str, properties: PropertyList<O>) -> HostUsersConf<O> {
    let mut users = vec![];
    let user_conf = UserConf::new(name.to_string(), properties);
    users.push(user_conf);
    HostUsersConf { users }
}

/// Add another user to the host
impl<O: OS> HostUsersConf<O> {
    pub fn user(mut self, name: &'static str, properties: PropertyList<O>) -> HostUsersConf<O> {
        let user_conf = UserConf::new(name.to_string(), properties);
        self.users.push(user_conf);
        self
    }
}

pub struct UserConf<O: OS> {
    pub name: String,
    pub properties: PropertyList<O>,
}

impl<O: OS> UserConf<O> {
    pub fn new(name: String, properties: PropertyList<O>) -> UserConf<O> {
        UserConf { name, properties }
    }
}

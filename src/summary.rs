use std::{collections::BTreeMap, fmt::Display};

use file_owner::{FileOwnerError, Group, Owner};
use serde::Serialize;

/// Lists of unique [User]s and [Group]s.
#[derive(Debug, Default, Serialize)]
pub struct Summary {
    users: BTreeMap<u32, Option<String>>,
    groups: BTreeMap<u32, Option<String>>,
}

impl Summary {
    /// Add a new [User] to the [Summary].
    pub fn add_user(&mut self, uid: u32) {
        self.users.entry(uid).or_default();
    }

    /// Add a new [Group] to the [Summary].
    pub fn add_group(&mut self, gid: u32) {
        self.groups.entry(gid).or_default();
    }

    /// Look up the names of all users and groups.
    pub fn lookup_names(&mut self) -> (Vec<(u32, FileOwnerError)>, Vec<(u32, FileOwnerError)>) {
        let mut user_failures = vec![];
        for (uid, name) in &mut self.users {
            if name.is_some() {
                continue;
            }

            match Owner::from_uid(*uid).name() {
                Ok(n) => *name = n,
                Err(e) => user_failures.push((*uid, e)),
            }
        }

        let mut group_failures = vec![];
        for (gid, name) in &mut self.groups {
            if name.is_some() {
                continue;
            }

            match Group::from_gid(*gid).name() {
                Ok(n) => *name = n,
                Err(e) => group_failures.push((*gid, e)),
            }
        }

        (user_failures, group_failures)
    }
}

impl Display for Summary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self { users, groups } = self;

        writeln!(f, "users:")?;
        for (uid, name) in users {
            match name {
                None => writeln!(f, "    {uid}")?,
                Some(name) => writeln!(f, "    {name} ({uid})")?,
            }
        }

        writeln!(f)?;

        writeln!(f, "groups:")?;
        for (gid, name) in groups {
            match name {
                None => writeln!(f, "    {gid}")?,
                Some(name) => writeln!(f, "    {name} ({gid})")?,
            }
        }

        Ok(())
    }
}

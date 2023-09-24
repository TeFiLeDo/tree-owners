use std::{collections::BTreeSet, fmt::Display};

use serde::Serialize;

/// Unique `uid`s and `gid`s.
#[derive(Debug, Default, Serialize)]
pub struct Ids {
    pub users: BTreeSet<u32>,
    pub groups: BTreeSet<u32>,
}

impl Display for Ids {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "users:")?;
        for user in &self.users {
            writeln!(f, "    {user}")?;
        }

        writeln!(f)?;

        writeln!(f, "groups:")?;
        for group in &self.groups {
            writeln!(f, "    {group}")?;
        }

        Ok(())
    }
}

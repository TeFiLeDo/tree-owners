use std::{collections::BTreeSet, fmt::Display};

use anyhow::{anyhow, Error, Result};
use file_owner::{Group, Owner};
use serde::Serialize;

use crate::id::Ids;

/// Unique user and group names.
#[derive(Debug, Default, Serialize)]
pub struct Names {
    pub users: BTreeSet<String>,
    pub groups: BTreeSet<String>,
}

impl Display for Names {
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

impl TryFrom<Ids> for Names {
    type Error = Error;

    fn try_from(value: Ids) -> Result<Self> {
        Ok(Self {
            users: value
                .users
                .into_iter()
                .map(|u| match Owner::from_uid(u).name() {
                    Ok(Some(name)) => Ok(name),
                    Ok(None) => Err(anyhow!("no name for user {u}")),
                    Err(_) => Err(anyhow!("failed to get name for user {u}")),
                })
                .collect::<Result<_>>()?,
            groups: value
                .groups
                .into_iter()
                .map(|g| match Group::from_gid(g).name() {
                    Ok(Some(name)) => Ok(name),
                    Ok(None) => Err(anyhow!("no name for group {g}")),
                    Err(_) => Err(anyhow!("failed to get name for group {g}")),
                })
                .collect::<Result<_>>()?,
        })
    }
}

use std::{cmp::Ordering, fmt::Debug};

use crate::{types::ScString, Env};

#[derive(Clone)]
pub struct String {
    env: Env,
    obj: ScString,
}

impl Debug for String {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "String()")?;
        Ok(())
    }
}

impl Eq for String {}

impl PartialEq for String {
    fn eq(&self, other: &Self) -> bool {
        self.obj.eq(&other.obj)
    }
}

impl Ord for String {
    fn cmp(&self, other: &Self) -> Ordering {
        self.obj.cmp(&other.obj)
    }
}

impl PartialOrd for String {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

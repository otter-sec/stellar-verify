use std::cmp::Ordering;

use crate::soroban_env_common::val::Val;
use crate::types::ScAddress;

#[cfg(any(kani, feature = "kani"))]
use crate::types::Hash;

#[derive(Debug, Clone)]
pub struct Address {
    pub obj: ScAddress,
}

impl Eq for Address {}

impl PartialEq for Address {
    fn eq(&self, other: &Self) -> bool {
        self.obj.eq(&other.obj)
    }
}

impl Ord for Address {
    fn cmp(&self, other: &Self) -> Ordering {
        self.obj.cmp(&other.obj)
    }
}

impl PartialOrd for Address {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.obj.partial_cmp(&other.obj)
    }
}

impl Address {
    pub fn require_auth_for_args(&self, _args: Vec<Val>) {
        todo!()
    }
}

// Derive kani::Arbitrary for Address
#[cfg(any(kani, feature = "kani"))]
impl kani::Arbitrary for Address {
    fn any() -> Self {
        let hash: Hash = kani::any();
        Address {
            obj: ScAddress::Contract(hash),
        }
    }
}

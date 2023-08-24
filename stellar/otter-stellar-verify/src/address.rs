use std::cmp::Ordering;

use crate::{
    env::internal,
    random::random,
    types::{Hash, ScAddress},
    Env, IntoVal,
};

#[allow(clippy::derived_hash_with_manual_eq)]
#[derive(Debug, Hash, Clone)]
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
        Some(self.cmp(other))
    }
}

impl Address {
    pub fn require_auth_for_args(&self, _args: (Address, Address, i128, i128)) {}
    pub fn require_auth(&self) {}
}

#[cfg(not(any(kani, feature = "kani")))]
impl Address {
    pub fn random(_env: &Env) -> Self {
        let result: [u8; 32] = random();
        let hash: Hash = Hash(result);
        Address {
            obj: ScAddress::Contract(hash),
        }
    }
}

impl<E: internal::Env> IntoVal<E, (Address, Address, i128, i128)>
    for (Address, Address, i128, i128)
{
    fn into_val(self, _e: &E) -> (Address, Address, i128, i128) {
        self.clone()
    }
}

#[cfg(any(kani, feature = "kani"))]
impl Address {
    pub fn random(_env: &Env) -> Self {
        let hash: Hash = kani::any();
        Address {
            obj: ScAddress::Contract(hash),
        }
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

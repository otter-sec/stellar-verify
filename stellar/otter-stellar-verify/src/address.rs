use crate::{env::Env, types::ScAddress};

#[cfg(any(kani, feature = "kani"))]
use crate::types::Hash;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Address {
    env: Env,
    // AddressObject is a type of ScAddress
    obj: ScAddress,
}

// Derive kani::Arbitrary for Address
#[cfg(any(kani, feature = "kani"))]
impl kani::Arbitrary for Address {
    fn any() -> Self {
        let hash: Hash = kani::any();
        Address {
            env: Env::default(),
            obj: ScAddress::Contract(hash),
        }
    }
}

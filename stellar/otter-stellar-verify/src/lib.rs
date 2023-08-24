pub mod address;
pub mod bytes;
pub mod crypto;
pub mod env;
pub mod events;
pub mod ledger;
pub mod random;
pub mod soroban_env_common;
pub mod soroban_env_host;
pub mod soroban_ledger_snapshot;
pub mod storage;
pub mod string;
pub mod token;
pub mod types;

pub use {
    address::Address,
    env::{Env, IntoVal},
    soroban_env_common::val::Val,
    stellar_sdk_macros::{contract, contractimpl},
    string::String,
};

#[cfg(any(kani, feature = "kani"))]
mod verification {
    use types::Uint256;

    use super::*;

    #[kani::proof]
    pub fn check_address() {
        let _address: address::Address = kani::any();
        let _address2: address::Address = kani::any();
        let test = Uint256::from_u128(12);
        assert_eq!(test, Uint256::from_u32(12));
    }
}

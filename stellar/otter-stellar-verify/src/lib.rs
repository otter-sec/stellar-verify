pub mod address;
pub mod env;
pub mod random;
pub mod storage;
pub mod testutils;
pub mod token;
pub mod types;

pub use {
    address::Address,
    env::{Env, IntoVal},
    stellar_sdk_macros::{contract, contractimpl},
};

#[cfg(any(kani, feature = "kani"))]
mod verification {
    use super::*;

    #[kani::proof]
    pub fn check_address() {
        let _address: address::Address = kani::any();
        let _address2: address::Address = kani::any();
    }
}

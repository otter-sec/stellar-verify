pub mod address;
pub mod env;
pub mod storage;
pub mod testutils;
pub mod token;

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
        let env = env::Env::default();
        let a: address::Address = Address::new(&env);
        let b: address::Address = Address::new(&env);
        assert!(a != b);
    }
}

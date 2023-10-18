pub mod address;
pub mod env;
pub mod storage;
pub mod testutils;
pub mod token;

pub use {
    address::Address,
    env::{Env, IntoVal},
    soroban_env_common::{val::Symbol, FromValEnum, ToValEnum, ValEnum},
    stellar_sdk_macros::{contract, contractimpl, symbol_short, verify},
};

#[macro_export]
macro_rules! log {
    ($env:expr, $fmt:literal $(,)?) => {
        // Do nothing.
    };
    ($env:expr, $fmt:literal, $($args:expr),* $(,)?) => {
        // Do nothing.
    };
}

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

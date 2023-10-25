pub mod address;
pub mod env;
pub mod ledger;
pub mod storage;
pub mod testutils;
pub mod token;

pub use {
    address::Address,
    env::{Env, IntoVal},
    soroban_env_common::{
        symbol::Symbol, ConversionError, FromValEnum, String, ToValEnum, Val, Vec,
    },
    stellar_sdk_macros::{
        contract, contractimpl, contractmeta, contracttype, symbol_short, verify,
    },
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

// #[macro_export]
// macro_rules! vec {
//     ($env:expr $(,)?) => {
//         $crate::Vec::new($env)
//     };
//     ($env:expr, $($x:expr),+ $(,)?) => {
//         $crate::Vec::from_array($env, [$($x),+])
//     };
// }

#[macro_export]
macro_rules! vec {
    () => (
        ($crate::Vec::new())
    );
    ($elem:expr; $n:expr) => (
        ($crate::Vec::from([$elem; $n]))
    );
    ($($x:expr),+ $(,)?) => (
        $crate::Vec::from([$($x),+])
    );
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

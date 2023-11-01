pub mod address;
pub mod auth;
pub mod bytes;
pub mod crypto;
pub mod deploy;
pub mod env;
pub mod events;
pub mod ledger;
pub mod storage;
pub mod symbol;
pub mod testutils;
pub mod token;

pub use {
    address::Address,
    auth::Context,
    bytes::{Bytes, BytesN},
    deploy::Deployer,
    env::{Env, IntoVal},
    events::Events,
    soroban_env_common::{
        symbol::Symbol, BytesObject, ConversionError, FromValEnum, String, Timepoint, ToValEnum,
        Val, Vec,
    },
    stellar_sdk_macros::{
        contract, contracterror, contractimpl, contractmeta, contracttype, symbol_short, verify,
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

#[macro_export]
macro_rules! contractimport {
    (file = $path:expr) => {
        pub struct Client {}

        impl Client {
            pub fn new(_env: &crate::Env, _contract: &crate::Address) -> Self {
                Self {}
            }
        }
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

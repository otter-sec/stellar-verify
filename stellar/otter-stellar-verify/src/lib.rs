pub mod auth;
pub mod bytes;
pub mod crypto;
pub mod deploy;
pub mod env;
pub mod events;
pub mod ledger;
pub mod map;
pub mod prng;
pub mod symbol;
pub mod testutils;
pub mod unwrap;
pub mod xdr;

pub use {
    auth::*,
    bytes::{Bytes, BytesN},
    crypto::Crypto,
    deploy::Deployer,
    env::EnvTrait,
    events::Events,
    map::Map,
    prng::Prng,
    soroban_env_common::{
        address::Address,
        env::{Env, IntoVal, TryFromVal},
        symbol::Symbol,
        symbol_short,
        token::{self, AdminClient, Client, Interface, MockToken},
        BytesObject, ConversionError, EnumType, FromValEnum, String, Timepoint, ToValEnum, Val,
        Vec,
    },
    stellar_sdk_macros::{
        contract, contractclient, contracterror, contractimpl, contractimport, contractmeta,
        contracttype, verifiable, verify,
    },
    xdr::{FromXdr, ToXdr},
};

#[cfg(any(kani, feature = "kani"))]
pub use kani::{self, Arbitrary};

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
    ($env:expr $(,)?) => {
        $crate::Vec::new($env)
    };
    ($env:expr, $($x:expr),+ $(,)?) => {
        $crate::Vec::from_array($env, [$($x),+])
    };
}

#[macro_export]
macro_rules! panic_with_error {
    ($env:expr, $error:expr) => {{
        panic!("{}", $error);
    }};
}

#[cfg(any(kani, feature = "kani"))]
mod verification {
    use super::*;
    use soroban_env_common::{Address, Env};

    #[kani::proof]
    pub fn check_address() {
        let env = Env::default();
        let a: Address = Address::new(&env);
        let b: Address = Address::new(&env);
        assert!(a != b);
    }
}

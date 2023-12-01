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
pub mod xdr;

pub use {
    auth::Context,
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
        contract, contracterror, contractimpl, contractimport, contractmeta, contracttype,
        verifiable, verify,
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
    () => (
        ($crate::Vec::new(Env::default()))
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
    use soroban_env_common::{Address, Env};

    #[kani::proof]
    pub fn check_address() {
        let env = Env::default();
        let a: Address = Address::new(&env);
        let b: Address = Address::new(&env);
        assert!(a != b);
    }
}

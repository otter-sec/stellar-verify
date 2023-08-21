use std::rc::Rc;

use crate::{
    address::Address,
    crypto::Crypto,
    events::Events,
    ledger::Ledger,
    soroban_ledger_snapshot::LedgerSnapshot,
    storage::Storage,
    types::{Hash, ScAddress},
};

pub mod internal {
    pub use crate::soroban_env_host::*;

    pub type EnvImpl = Host;
    pub type MaybeEnvImpl = Option<Host>;

    pub trait Env {}
}

#[derive(Clone)]
pub struct Env {
    env_impl: internal::EnvImpl,
    _snapshot: Option<Rc<LedgerSnapshot>>,
}
impl Default for Env {
    fn default() -> Self {
        Self::default_with_testutils()
    }
}
impl Env {
    pub fn host(&self) -> &internal::Host {
        &self.env_impl
    }

    fn default_with_testutils() -> Env {
        Env {
            env_impl: internal::EnvImpl::default(),
            _snapshot: None,
        }
    }

    pub fn current_contract_address(&self) -> Address {
        Address {
            obj: ScAddress::Contract(Hash::default()),
        }
    }

    #[inline(always)]
    pub fn storage(&self) -> Storage {
        Storage::new(self)
    }

    #[inline(always)]
    pub fn events(&self) -> Events {
        Events::new(self)
    }

    #[inline(always)]
    pub fn ledger(&self) -> Ledger {
        Ledger::new(self)
    }

    #[inline(always)]
    pub fn crypto(&self) -> Crypto {
        Crypto::new(self)
    }

    pub fn register_stellar_asset_contract(&self, admin: Address) -> Address {
        todo!()
    }
}

pub trait IntoVal<E: internal::Env, T> {
    fn into_val(&self, e: &E) -> T;
}

#[derive(Clone, Default)]
pub struct MaybeEnv {
    _maybe_env_impl: internal::MaybeEnvImpl,
    _snapshot: Option<Rc<LedgerSnapshot>>,
}

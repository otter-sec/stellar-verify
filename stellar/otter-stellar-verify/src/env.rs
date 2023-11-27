use crate::{events::Events, ledger::Ledger, Crypto, Deployer, Prng};
use soroban_env_common::Env;

pub trait EnvTrait {
    fn events(&self) -> Events;
    fn ledger(&self) -> Ledger;
    fn crypto(&self) -> Crypto;
    fn deployer(&self) -> Deployer;
    fn prng(&self) -> Prng;
}

impl EnvTrait for Env {
    fn events(&self) -> Events {
        Events::new(self)
    }

    fn ledger(&self) -> Ledger {
        Ledger::default()
    }

    fn crypto(&self) -> Crypto {
        Crypto::new(self)
    }

    fn deployer(&self) -> Deployer {
        Deployer::new(self)
    }

    fn prng(&self) -> Prng {
        Prng::new(self)
    }
}

use crate::ledger::Ledger;
use crate::Crypto;
use crate::Deployer;
use crate::Prng;
use crate::Val;
use crate::{address::Address, events::Events, storage::Storage, token::MockToken};
use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

pub mod internal {

    pub trait Env {}
}

#[derive(Debug)]
pub struct Env {
    pub storage: Rc<RefCell<Storage>>,
}

impl Clone for Env {
    fn clone(&self) -> Self {
        Env {
            storage: self.storage.clone(), // Cloning the Rc
        }
    }
}

impl Default for Env {
    fn default() -> Self {
        Self::default_with_testutils()
    }
}

impl internal::Env for Env {}

pub static mut CURRENT_CONTRACT: u8 = 0;

impl Env {
    fn default_with_testutils() -> Env {
        Env {
            storage: Rc::new(RefCell::new(Storage::default())),
        }
    }

    pub fn events(&self) -> Events {
        Events::new(self)
    }

    pub fn ledger(&self) -> Ledger {
        Ledger::default()
    }

    pub fn storage(&self) -> Ref<Storage> {
        self.storage.borrow()
    }

    pub fn current_contract_address(&self) -> Address {
        unsafe {
            Address {
                val: CURRENT_CONTRACT,
            }
        }
    }

    pub fn crypto(&self) -> Crypto {
        Crypto::new(self)
    }

    pub fn deployer(&self) -> Deployer {
        Deployer::new(self)
    }

    pub fn prng(&self) -> Prng {
        Prng::new(self)
    }

    pub fn invoke_contract<T>(
        &self,
        _contract_address: &Address,
        _func: &crate::Symbol,
        _args: Vec<Val>,
    ) -> T {
        todo!("Not yet implemented");
    }

    pub fn mock_all_auths(&self) {}

    pub fn register_stellar_asset_contract(&self, admin: Address) -> Address {
        let contract_address = Address::new(self);
        let token = MockToken::new(
            contract_address,
            "Stellar Lumens".to_string(),
            "XLM".to_string(),
            7,
            100_000_000_000,
            admin,
        );
        self.storage.borrow_mut().insert_token(token);
        contract_address
    }

    pub fn register_contract(&self, _contract_id: Option<Address>) -> Address {
        let contract_address = Address::new(self);
        unsafe {
            CURRENT_CONTRACT = contract_address.val;
        }
        contract_address
    }
}

pub trait IntoVal<E: internal::Env, T> {
    fn into_val(self, e: &E) -> T;
}

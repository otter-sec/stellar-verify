use crate::ledger::Ledger;
use crate::{address::Address, storage::Storage, token::MockToken};
use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

pub mod internal {

    pub trait Env {}
}

#[derive(Clone, Debug)]
pub struct Env {
    pub storage: Rc<RefCell<Storage>>,
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

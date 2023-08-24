use std::{cell::RefCell, rc::Rc};

use crate::{
    address::Address,
    storage::Storage,
    token::MockToken,
    types::{Hash, ScAddress},
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

impl Env {
    fn default_with_testutils() -> Env {
        Env {
            storage: Rc::new(RefCell::new(Storage::default())),
        }
    }

    pub fn current_contract_address(&self) -> Address {
        Address {
            obj: ScAddress::Contract(Hash::default()),
        }
    }

    pub fn register_stellar_asset_contract(&self, admin: Address) -> Address {
        let contract_address = Address::random(self);
        let token = MockToken::new(
            contract_address.clone(),
            "Stellar Lumens".to_string(),
            "XLM".to_string(),
            7,
            100_000_000_000,
            admin,
        );
        self.storage.borrow_mut().set_token(token);
        contract_address
    }
}

pub trait IntoVal<E: internal::Env, T> {
    fn into_val(self, e: &E) -> T;
}

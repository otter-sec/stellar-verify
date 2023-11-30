use crate::{Address, MockToken, Storage, String, Val};
use std::fmt::Debug;
use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

pub static mut CURRENT_CONTRACT: u8 = 0;

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

pub mod internal {

    pub trait Env {}
}

impl internal::Env for Env {}

impl Env {
    fn default_with_testutils() -> Env {
        Env {
            storage: Rc::new(RefCell::new(Storage::default())),
        }
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
            "Stellar Lumens".into(),
            "XLM".into(),
            7,
            100_000_000_000,
            admin,
        );
        self.storage.borrow_mut().insert_token(token);
        contract_address
    }

    pub fn register_contract<T>(&self, _contract_id: Option<Address>, _contract: T) -> Address {
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

pub trait TryFromVal<E: internal::Env, V: ?Sized>: Sized {
    type Error: Debug + Into<crate::ConversionError>;
    fn try_from_val(env: &E, v: &V) -> Result<Self, Self::Error>;
}

impl<E: internal::Env> IntoVal<E, String> for &'static str {
    fn into_val(self, _e: &E) -> String {
        self.into()
    }
}

use crate::{Address, FromValEnum, MockToken, Storage, Val};
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

    #[cfg(any(kani, feature = "kani"))]
    pub fn invoke_contract<T: kani::Arbitrary>(
        &self,
        _contract_address: &Address,
        _func: &crate::Symbol,
        _args: crate::Vec<Val>,
    ) -> T {
        kani::any()
    }

    #[cfg(not(any(kani, feature = "kani")))]
    pub fn invoke_contract<T>(
        &self,
        _contract_address: &Address,
        _func: &crate::Symbol,
        _args: crate::Vec<Val>,
    ) -> T {
        unimplemented!("Cross-contract calls not supported");
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

    pub fn authorize_as_current_contract<T>(&self, _auth_entries: Vec<T>) {
        // nop
    }
}

pub trait IntoVal<E: internal::Env, T> {
    fn into_val(self, e: &E) -> T;
}

impl<E: internal::Env, T> IntoVal<E, T> for T {
    fn into_val(self, _env: &E) -> T {
        self
    }
}

pub trait TryFromVal<E: internal::Env, V: ?Sized>: Sized {
    type Error: Debug + Into<crate::ConversionError>;
    fn try_from_val(env: &E, v: &Val) -> Result<Self, Self::Error>;
}

impl<E: internal::Env, T, U> TryFromVal<E, T> for U
where
    U: FromValEnum,
{
    type Error = crate::ConversionError;
    fn try_from_val(_e: &E, v: &Val) -> Result<Self, Self::Error> {
        Ok(U::from_val(v.clone()).unwrap())
    }
}

pub trait TryIntoVal<E, V>
where
    E: internal::Env,
    V: FromValEnum,
{
    // Remove the default type assignment for the Error associated type
    type Error;

    // Required method
    fn try_into_val(&self, env: &E) -> Result<V, Self::Error>;
}

// Implement TryIntoVal for Val
impl<E, V> TryIntoVal<E, V> for Val
where
    E: internal::Env,
    V: FromValEnum,
{
    // Replace this with your actual error type
    type Error = crate::ConversionError;

    fn try_into_val(&self, _env: &E) -> Result<V, Self::Error> {
        // Call V::from_val to convert Val to V
        V::from_val(self.clone()).ok_or(crate::ConversionError)
    }
}

#[cfg(any(kani, feature = "kani"))]
impl kani::Arbitrary for Env {
    fn any() -> Env {
        Env::default()
    }
}

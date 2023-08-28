use crate::{env::internal, Env, IntoVal};

#[derive(Debug, Hash, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub struct Address {
    pub val: u8,
}

impl Address {
    pub fn require_auth_for_args(&self, _args: (Address, Address, i128, i128)) {}
    pub fn require_auth(&self) {}
}

impl<E: internal::Env> IntoVal<E, (Address, Address, i128, i128)>
    for (Address, Address, i128, i128)
{
    fn into_val(self, _e: &E) -> (Address, Address, i128, i128) {
        self
    }
}
const MAX_KEYS: u8 = 100;
pub static mut KEYS: u8 = 0;

impl Address {
    pub fn new(_env: &Env) -> Self {
        unsafe {
            assert!(KEYS < MAX_KEYS, "Ran out of keys during context creation.",);
            KEYS += 1;
            Address { val: KEYS - 1 }
        }
    }
}

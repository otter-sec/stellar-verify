use std::fmt::Display;

use crate::{env::internal, Env, FromValEnum, IntoVal, ToValEnum};

#[derive(Debug, Hash, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Default)]
pub struct Address {
    pub val: u8,
}

impl From<crate::Val> for Address {
    fn from(val: crate::Val) -> Self {
        match val {
            crate::Val::AddressObj(address) => Address { val: address as u8 },
            _ => panic!("Error"),
        }
    }
}

impl Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(Address {})", self.val)
    }
}

impl Address {
    pub fn require_auth_for_args(&self, _args: (Address, Address, i128, i128)) {}
    pub fn require_auth(&self) {}

    pub fn to_le_bytes(&self) -> [u8; 1] {
        self.val.to_le_bytes()
    }

    pub fn from_le_bytes(bytes: [u8; 1]) -> Self {
        Self {
            val: u8::from_le_bytes(bytes),
        }
    }
}

impl<E: internal::Env> IntoVal<E, (Address, Address, i128, i128)>
    for (Address, Address, i128, i128)
{
    fn into_val(self, _e: &E) -> (Address, Address, i128, i128) {
        self
    }
}

impl ToValEnum for Address {
    fn into_val(&self) -> crate::Val {
        crate::Val::AddressObj(self.val as u32)
    }
}

impl FromValEnum for Address {
    fn from_val(val: crate::Val) -> Option<Self> {
        if let crate::Val::AddressObj(u) = val {
            Some(Address { val: u as u8 })
        } else {
            None
        }
    }
}

// For Kani
const MAX_KEYS: u8 = 100;
pub static mut KEYS: u8 = 1;

impl Address {
    pub fn new(_env: &Env) -> Self {
        unsafe {
            assert!(KEYS < MAX_KEYS, "Ran out of keys during context creation.",);
            KEYS += 1;
            Address { val: KEYS - 1 }
        }
    }
}

#[cfg(any(kani, feature = "kani"))]
impl kani::Arbitrary for Address {
    fn any() -> Self {
        Address::new(&Env::default())
    }
}

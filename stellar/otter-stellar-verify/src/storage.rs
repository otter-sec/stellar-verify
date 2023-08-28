use std::collections::BTreeMap;
use std::fmt::Debug;

use crate::token::MockToken;
use crate::Address;

#[derive(Clone, Default)]
pub struct Storage {
    tokens: BTreeMap<Address, MockToken>,
}

impl Debug for Storage {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Storage")
    }
}

impl Storage {
    pub fn get_token(&self, address: &Address) -> Option<MockToken> {
        self.tokens.get(address).cloned()
    }

    pub fn set_token(&mut self, token: MockToken) {
        self.tokens.insert(token.address.clone(), token);
    }
}

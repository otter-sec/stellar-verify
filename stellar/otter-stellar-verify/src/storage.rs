use std::collections::BTreeMap;
use std::fmt::Debug;

use crate::Address;

use crate::env::Env;
use crate::token::MockToken;

#[derive(Clone)]
pub struct Storage {
    env: Env,
    tokens: BTreeMap<Address, MockToken>,
}

impl Debug for Storage {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Storage")
    }
}

impl Storage {
    #[inline(always)]
    pub(crate) fn new(env: &Env) -> Storage {
        Storage {
            env: env.clone(),
            tokens: BTreeMap::new(),
        }
    }

    pub fn get_token(&self, address: &Address) -> Option<MockToken> {
        self.tokens.get(address).cloned()
    }

    pub fn set_token(&mut self, token: MockToken) {
        self.tokens.insert(token.address.clone(), token);
    }
}

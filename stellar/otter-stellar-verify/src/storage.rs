use std::fmt::Debug;

use crate::token::MockToken;
use crate::Address;

#[derive(Clone, Default)]
pub struct Storage {
    tokens: Vec<MockToken>,
}

impl Debug for Storage {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Storage")
    }
}

impl Storage {
    pub fn get_token(&self, address: &Address) -> Option<MockToken> {
        self.tokens.iter().find(|t| t.address == *address).cloned()
    }

    pub fn insert_token(&mut self, token: MockToken) {
        self.tokens.push(token);
    }

    pub fn update_token(&mut self, token: MockToken) {
        //get the index of the token
        let index = self
            .tokens
            .iter()
            .position(|t| t.address == token.address)
            .unwrap();
        //replace the token
        self.tokens[index] = token;
    }
}

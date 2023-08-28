use std::collections::BTreeMap;

use crate::{address::Address, env::Env};

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct AddressPair {
    from: Address,
    to: Address,
}

#[derive(Clone, Debug)]
pub struct MockToken {
    pub address: Address,
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: i128,
    pub balances: BTreeMap<Address, i128>,
    pub admin: Address,
}

impl MockToken {
    pub fn new(
        address: Address,
        name: String,
        symbol: String,
        decimals: u8,
        total_supply: i128,
        admin: Address,
    ) -> Self {
        Self {
            address,
            name,
            symbol,
            decimals,
            total_supply,
            balances: BTreeMap::new(),
            admin,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Client {
    pub env: Env,
    pub address: Address,
}

impl Client {
    pub fn new(env: &Env, address: &Address) -> Self {
        Self {
            env: env.clone(),
            address: address.clone(),
        }
    }

    pub fn get_self_token(&self) -> Option<MockToken> {
        self.env.storage.borrow().get_token(&self.address)
    }

    pub fn balance(&self, address: &Address) -> i128 {
        let token = self.get_self_token().expect("Asset not found");
        *token.balances.get(address).unwrap_or(&0)
    }

    pub fn transfer(&self, from: &Address, to: &Address, amount: &i128) {
        let mut token = self.get_self_token().expect("Asset not found");
        let prev_bal_from = *token.balances.get(from).unwrap_or(&0);
        assert!(prev_bal_from >= *amount);
        let prev_bal_to = *token.balances.get(to).unwrap_or(&0);
        token.balances.insert(from.clone(), prev_bal_from - amount);
        token.balances.insert(to.clone(), prev_bal_to + amount);
        self.env.storage.borrow_mut().set_token(token.clone());
    }
}

#[derive(Clone, Debug)]
pub struct AdminClient {
    pub env: Env,
    pub address: Address,
}

impl AdminClient {
    pub fn new(env: &Env, address: &Address) -> Self {
        Self {
            env: env.clone(),
            address: address.clone(),
        }
    }

    pub fn get_self_token(&self) -> Option<MockToken> {
        self.env.storage.borrow().get_token(&self.address)
    }

    pub fn update_self_token(&self, token: &MockToken) {
        self.env.storage.borrow_mut().set_token(token.clone());
    }

    pub fn balance(&self, address: &Address) -> i128 {
        let token = self.get_self_token().expect("Asset not found");
        *token.balances.get(address).unwrap_or(&0)
    }

    pub fn mint(&self, to: &Address, amount: &i128) {
        let mut token = self.get_self_token().expect("Asset not found");
        let prev_bal = token.balances.get(to).unwrap_or(&0);
        token.balances.insert(to.clone(), prev_bal + amount);
        self.update_self_token(&token);
    }

    pub fn admin(&self) -> Address {
        let token = self.get_self_token().expect("Asset not found");
        token.admin.clone()
    }

    pub fn set_admin(&self, new_admin: &Address) {
        let mut token = self.get_self_token().expect("Asset not found");
        token.admin = new_admin.clone();
        self.env.storage.borrow_mut().set_token(token.clone());
    }
}

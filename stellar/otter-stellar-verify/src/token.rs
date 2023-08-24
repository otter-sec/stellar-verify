use std::collections::HashMap;

use crate::{address::Address, env::Env, storage};

#[derive(PartialEq, Eq, Hash)]
struct AddressPair {
    from: Address,
    to: Address,
}

pub struct MockToken {
    pub address: Address,
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: i128,
    pub allowances: HashMap<AddressPair, i128>,
    pub balances: HashMap<Address, i128>,
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
            allowances: HashMap::new(),
            balances: HashMap::new(),
            admin,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Client {
    pub address: Address,
    pub token: MockToken,
}

impl Client {
    pub fn new(env: &Env, address: &Address) -> Self {
        Self {
            address: address.clone(),
            token: storage::get_token(address),
        }
    }

    pub fn approve(&self, spender: &Address, amount: &i128) {
        let pair = AddressPair {
            from: self.address.clone(),
            to: spender.clone(),
        };
        let prev_allowance = self.token.allowances.get(&pair).unwrap_or(&0);
        self.token.allowances.insert(pair, prev_allowance + amount);
        storage.set_token(self.token.clone());
    }

    pub fn allowance(&self, spender: &Address) -> i128 {
        let pair = AddressPair {
            from: self.address.clone(),
            to: spender.clone(),
        };
        *self.token.allowances.get(&pair).unwrap_or(&0)
    }

    pub fn balance(&self) -> i128 {
        *self.token.balances.get(&self.address).unwrap_or(&0)
    }

    pub fn spendable_balance(&self) -> i128 {
        self.balance()
    }

    pub fn transfer(&self, from: Address, to: &Address, amount: &i128) {
        let prev_bal_to = self.token.balances.get(to).unwrap_or(&0);
        let prev_bal_from = self.token.balances.get(&self.address).unwrap_or(&0);
        assert!(prev_bal_from >= amount);
        self.token.balances.insert(to, prev_bal + amount);
        self.token.balances.insert(from, prev_bal_from - amount);
        storage.set_token(self.token);
    }
}

#[derive(Clone, Debug)]
pub struct AdminClient {
    pub address: Address,
    pub token: MockToken,
}

impl AdminClient {
    pub fn new(env: &Env, address: &Address) -> Self {
        Self {
            address: address.clone(),
            token: storage::get_token(address),
        }
    }

    pub fn mint(&self, to: &Address, amount: &i128) {
        let prev_bal = self.token.balances.get(to).unwrap_or(&0);
        self.token.balances.insert(to.clone(), prev_bal + amount);
        storage.set_token(self.token);
    }

    pub fn admin(&self) -> Address {
        self.token.admin.clone()
    }

    pub fn set_admin(&mut self, new_admin: &Address) {
        self.token.admin = new_admin;
        storage.set_token(self.token);
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct BytesObject {}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct StringObject {}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct SymbolObject {}

impl ToString for SymbolObject {
    fn to_string(&self) -> String {
        self.to_string()
    }
}

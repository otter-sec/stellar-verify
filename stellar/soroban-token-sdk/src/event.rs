use soroban_sdk::{Address, Env};

pub struct Events {
    _env: Env,
}

impl Events {
    pub fn new(env: &Env) -> Events {
        Events { _env: env.clone() }
    }

    pub fn approve(&self, _from: Address, _to: Address, _amount: i128, _expiration_ledger: u32) {}

    pub fn transfer(&self, _from: Address, _to: Address, _amount: i128) {}

    pub fn mint(&self, _admin: Address, _to: Address, _amount: i128) {}

    pub fn clawback(&self, _admin: Address, _from: Address, _amount: i128) {}

    pub fn set_authorized(&self, _admin: Address, _id: Address, _authorize: bool) {}

    pub fn set_admin(&self, _admin: Address, _new_admin: Address) {}

    pub fn burn(&self, _from: Address, _amount: i128) {}
}

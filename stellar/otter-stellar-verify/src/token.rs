use crate::{address::Address, env::Env};

#[derive(Clone, Debug)]
pub struct Client {}

impl Client {
    pub fn new(env: &Env, address: &Address) -> Self {
        Self {}
    }

    pub fn transfer(&self, _from: &Address, _contract_address: &Address, _max_spend_amount: &i128) {
        todo!()
    }
}

#[derive(Clone, Debug)]
pub struct AdminClient {}

impl AdminClient {
    pub fn mint(&self, _to: &Address, _amount: &i128) {
        todo!()
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

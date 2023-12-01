use soroban_sdk::{contracttype, symbol_short, Env, String, Symbol};

const METADATA_KEY: Symbol = symbol_short!("METADATA_");

extern crate alloc;
#[derive(Clone, Default)]
#[contracttype]
pub struct TokenMetadata {
    pub decimal: u32,
    pub name: String,
    pub symbol: String,
}

pub struct Metadata {
    env: Env,
}

impl Metadata {
    pub fn new(env: &Env) -> Metadata {
        Metadata { env: env.clone() }
    }

    pub fn set_metadata(&self, metadata: &TokenMetadata) {
        self.env.storage().instance().set(&METADATA_KEY, metadata);
    }

    pub fn get_metadata(&self) -> TokenMetadata {
        self.env
            .storage()
            .instance()
            .get(&METADATA_KEY)
            .unwrap_or_default()
    }
}

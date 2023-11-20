use soroban_sdk::{contracttype, symbol_short, Env, String, Symbol};

const _METADATA_KEY: Symbol = symbol_short!("METADATA");

#[derive(Clone)]
#[contracttype]
pub struct TokenMetadata {
    pub decimal: u32,
    pub name: String,
    pub symbol: String,
}

pub struct Metadata {
    _env: Env,
}

impl Metadata {
    pub fn new(env: &Env) -> Metadata {
        Metadata { _env: env.clone() }
    }

    #[inline(always)]
    pub fn set_metadata(&self, _metadata: &TokenMetadata) {}

    #[inline(always)]
    pub fn get_metadata(&self) -> TokenMetadata {
        //self.env.storage().instance().get(&METADATA_KEY).unwrap()
        TokenMetadata {
            decimal: 0,
            name: String::from("Test"),
            symbol: String::from("TST"),
        }
    }
}

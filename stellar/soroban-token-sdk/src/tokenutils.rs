use crate::event::Events;

use crate::metadata::Metadata;
use soroban_sdk::Env;

#[derive(Clone)]
pub struct TokenUtils(Env);

impl TokenUtils {
    pub fn new(env: &Env) -> TokenUtils {
        TokenUtils(env.clone())
    }

    pub fn metadata(&self) -> Metadata {
        Metadata::new(&self.0)
    }

    pub fn events(&self) -> Events {
        Events::new(&self.0)
    }
}

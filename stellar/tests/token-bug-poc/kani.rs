#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use core::prelude::rust_2021::*;
#[macro_use]
extern crate core;
extern crate compiler_builtins as _;
use soroban_sdk::{contract, contractimpl, token, Address, Env};
use soroban_sdk::{
    token::AdminClient as TokenAdminClient_, token::Client as TokenClient_, verify,
    EnvTrait,
};
#[cfg(any(kani, feature = "kani"))]
use soroban_sdk::kani;
pub struct Poc;
pub struct PocClient<'a> {
    pub env: soroban_sdk::Env,
    pub address: soroban_sdk::Address,
    _phantom: core::marker::PhantomData<&'a ()>,
}
impl<'a> PocClient<'a> {
    pub fn new(env: &soroban_sdk::Env, address: &soroban_sdk::Address) -> Self {
        Self {
            env: env.clone(),
            address: address.clone(),
            _phantom: core::marker::PhantomData,
        }
    }
}
impl Poc {
    fn create_token_contract<'a>(
        e: &Env,
        admin: &Address,
    ) -> (TokenClient_, TokenAdminClient_) {
        let contract_address = e.register_stellar_asset_contract(admin.clone());
        (
            TokenClient_::new(e, &contract_address),
            TokenAdminClient_::new(e, &contract_address),
        )
    }
}
impl Poc {
    pub fn failing_example(env: Env, token: Address) {
        if !(token::Client::new(&env, &token).balance(&env.current_contract_address())
            > 0)
        {
            ::core::panicking::panic(
                "assertion failed: token::Client::new(&env, &token).balance(&env.current_contract_address()) > 0",
            )
        }
    }
    #[allow(dead_code)]
    pub fn verify_failing_example() {
        let env = kani::any::<Env>();
        let _ = env.register_contract(None, 0);
        let (token_client, token_admin) = Self::create_token_contract(
            &env,
            &Address::new(&env),
        );
        token_admin.mint(&env.current_contract_address(), &1);
        let token = token_client.address;
        let env_clone = env.clone();
        kani::assume(true);
        let result = Self::failing_example(env_clone, token);
        if !(true) {
            ::core::panicking::panic("assertion failed: (true)")
        }
    }
    pub fn succeeding_example(env: Env, token: Address) {
        token::AdminClient::new(&env, &token).mint(&env.current_contract_address(), &1);
        if !(token::Client::new(&env, &token).balance(&env.current_contract_address())
            > 0)
        {
            ::core::panicking::panic(
                "assertion failed: token::Client::new(&env, &token).balance(&env.current_contract_address()) > 0",
            )
        }
    }
    #[allow(dead_code)]
    pub fn verify_succeeding_example() {
        let env = kani::any::<Env>();
        let _ = env.register_contract(None, 0);
        let (token_client, _) = Self::create_token_contract(&env, &Address::new(&env));
        let token = token_client.address;
        let env_clone = env.clone();
        kani::assume(true);
        let result = Self::succeeding_example(env_clone, token);
        if !(true) {
            ::core::panicking::panic("assertion failed: (true)")
        }
    }
}
#[cfg(any(kani, feature = "kani"))]
impl<'a> PocClient<'a> {
    pub fn failing_example(&self, token: &Address) {}
    pub fn succeeding_example(&self, token: &Address) {}
}

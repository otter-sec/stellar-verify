#![no_std]

use soroban_sdk::{contract, contractimpl, token, Address, Env};

#[contract]
pub struct Poc;

#[contractimpl]
impl Poc {
    #[cfg_attr(any(kani, feature = "kani"),
        verify,
        init({
            let (token_client, token_admin) = Self::create_token_contract(&env, &Address::new(&env));
            token_admin.mint(&env.current_contract_address(), &1);
            let token = token_client.address;
        })
    )]
    pub fn my_balance(env: Env, token: Address) {
        assert!(token::Client::new(&env, &token).balance(&env.current_contract_address()) > 0)
    }
}

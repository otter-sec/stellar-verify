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
    pub fn failing_example(env: Env, token: Address) {
        assert!(token::Client::new(&env, &token).balance(&env.current_contract_address()) > 0)
    }

    #[cfg_attr(any(kani, feature = "kani"),
        verify,
        init({
            let (token_client, _) = Self::create_token_contract(&env, &Address::new(&env));
            let token = token_client.address;
        })
    )]
    pub fn succeeding_example(env: Env, token: Address) {
        token::AdminClient::new(&env, &token).mint(&env.current_contract_address(), &1);
        assert!(token::Client::new(&env, &token).balance(&env.current_contract_address()) > 0)
    }
}

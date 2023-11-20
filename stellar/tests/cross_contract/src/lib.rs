#![no_std]

use soroban_sdk::{contract, contractimpl, Address, Env};

mod contract_a {
    soroban_sdk::contractimport!(file = "./src/soroban_cross_contract_a_contract.wasm");
}

#[contract]
pub struct ContractB;

#[contractimpl]
impl ContractB {
    #[cfg_attr(any(kani, feature = "kani"), verify)]
    pub fn add_with(env: Env, contract: Address, x: u32, y: u32) -> u32 {
        let client = contract_a::Client::new(&env, &contract);

        client.add(&x, &y)
    }
}

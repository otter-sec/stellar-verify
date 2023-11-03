#![no_std]

use soroban_sdk::{contract, contractimpl, verify, Address, Env};

mod contract_a {
    soroban_sdk::contractimport!(file = "./soroban_cross_contract_a_contract.wasm");
}

#[contract]
pub struct ContractB;

#[contractimpl]
impl ContractB {
    #[cfg_attr(any(kani, feature = "kani"), 
        verify,
        init({
            let env = Env::default();
            let contract = kani::any();
            let x = kani::any();
            let y = kani::any();
        }),
        succeeds_if({
            true
        }),
        post_condition({
            true
        })
    )]
    pub fn add_with(env: Env, contract: Address, x: u32, y: u32) -> u32 {
        let client = contract_a::Client::new(&env, &contract);

        #[cfg(not(any(kani, feature = "kani")))]
        {
            client.add(&x, &y)
        }
        #[cfg(any(kani, feature = "kani"))]
        {
            kani::any::<u32>()
        }
    }
}
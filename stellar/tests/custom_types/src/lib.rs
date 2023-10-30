#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, verify, Address, Env, Symbol,
};

extern crate alloc;
use alloc::vec::Vec;

use soroban_sdk::{FromValEnum, ToValEnum, Val};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct State {
    pub count: u32,
    pub last_incr: u32,
}

const STATE: Symbol = symbol_short!("STATE");

#[contract]
pub struct IncrementContract;

#[contractimpl]
impl IncrementContract {
    /// Increment increments an internal counter, and returns the value.
    #[cfg_attr(any(kani, feature = "kani"), 
        verify,
        init({
            let env = Env::default();
            let incr = kani::any();
            env.storage().instance().set(&STATE, 
                &State {
                    count: kani::any(),
                    last_incr: kani::any()
                }
            );
        }),
        succeeds_if({
            Self::get_state(env.clone()).count <= u32::MAX - incr
        }),
        post_condition({
            true
        })
    )]
    pub fn increment(env: Env, incr: u32) -> u32 {
        // Get the current count.
        let mut state = Self::get_state(env.clone());

        // Increment the count.
        state.count += incr;
        state.last_incr = incr;

        // Save the count.
        env.storage().instance().set(&STATE, &state);

        // Return the count to the caller.
        state.count
    }
    /// Return the current state.
    pub fn get_state(env: Env) -> State {
        env.storage().instance().get(&STATE).unwrap_or(State {
            count: 0,
            last_incr: 0,
        }) // If no value set, assume 0.
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_increment() {
        let env = Env::default();
        let _ = IncrementContract::increment(env.clone(), 101);
        let state = IncrementContract::get_state(env.clone());
        assert!(state.count == 101);
        assert!(state.last_incr == 101);
    }
}

#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Env, Symbol};
extern crate alloc;

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct State {
    pub count: u32,
    pub last_incr: u32,
}

#[contracttype]
#[derive(Clone)]
pub struct Pair(Address, Address);

const STATE: Symbol = symbol_short!("STATE");

#[contract]
pub struct IncrementContract;

#[contractimpl]
impl IncrementContract {
    /// Increment increments an internal counter, and returns the value.
    #[cfg_attr(any(kani, feature = "kani"), 
        verify,
        init({
            env.storage().instance().set(&STATE, &kani::any::<State>());
        }),
        succeeds_if({
            Self::get_state(env).count <= u32::MAX - incr
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
        let a1 = Address::new(&env);
        let a2 = Address::new(&env);
        let p1 = Pair(a1, a2);
        let serialized_bytes = Pair::serialize(&p1);
        let deserialized_pair = Pair::deserialize(&serialized_bytes);
        assert!(deserialized_pair.0 == a1);
        assert!(deserialized_pair.1 == a2);
    }
}

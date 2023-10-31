#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, verify, Address, Env, Symbol};

const COUNTER: Symbol = symbol_short!("COUNTER");

#[contract]
pub struct IncrementContract;

#[contractimpl]
impl IncrementContract {
    /// Increment increments an internal counter, and returns the value.
    #[cfg_attr(any(kani, feature = "kani"),
        verify,
        init({
            let env = Env::default();
            env.storage().instance().set(&COUNTER, &kani::any::<u32>());
        }),
        succeeds_if({
            env.storage().instance().get(&COUNTER).unwrap_or(0) < u32::MAX
        }),
        post_condition({
            true
        })
    )]
    pub fn increment(env: Env) -> u32 {
        // Get the current count.
        let mut count: u32 = env.storage().instance().get(&COUNTER).unwrap_or(0); // If no value set, assume 0.

        // Increment the count.
        count += 1;

        // Save the count.
        env.storage().instance().set(&COUNTER, &count);

        // Publish an event about the increment occuring.
        // The event has two topics:
        //   - The "COUNTER" symbol.
        //   - The "increment" symbol.
        // The event data is the count.
        env.events()
            .publish((COUNTER, symbol_short!("increment")), count);

        // Return the count to the caller.
        count
    }
}
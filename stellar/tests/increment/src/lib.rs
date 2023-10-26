// #![no_std]
use soroban_sdk::{contract, contractimpl, log, symbol_short, verify, Address, Env, Symbol};

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
        }),
        succeeds_if({
            true
        }),
        post_condition({
            env.storage().instance().get::<_, u32>(&COUNTER).unwrap_or(0) == 1
        }),
    )]
    pub fn increment(env: Env) -> u32 {
        // Get the current count.
        let mut count: u32 = env.storage().instance().get(&COUNTER).unwrap_or(0); // If no value set, assume 0.
        log!(&env, "count: {}", count);

        // Increment the count.
        count += 1;

        // Save the count.
        env.storage().instance().set(&COUNTER, &count);
        count = env.storage().instance().get(&COUNTER).unwrap_or(0);
        // The contract instance will be bumped to have a lifetime of at least 100 ledgers if the current expiration lifetime at most 50.
        // If the lifetime is already more than 100 ledgers, this is a no-op. Otherwise,
        // the lifetime is extended to 100 ledgers. This lifetime bump includes the contract
        // instance itself and all entries in storage().instance(), i.e, COUNTER.
        env.storage().instance().bump(50, 100);

        // Return the count to the caller.
        count
    }
}

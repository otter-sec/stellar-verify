#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, BytesN, Env};

extern crate alloc;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[contracttype]
pub enum DataKey {
    SignerCnt,
    ZeroVal,
    Counter(Address),
    Data(BytesN<32>),
}

#[contract]
pub struct IncrementContract;

#[contractimpl]
impl IncrementContract {
    /// Increment increments a counter for the user, and returns the value.
    #[cfg_attr(any(kani, feature = "kani"),
        verify,
        init({
            let env = Env::default();
            let user = Address::new(&env);
            let byte_data: BytesN<32> = BytesN::from_array(&[
                1u8, 2, 3, 4, 5, 6, 7, 8, 1, 2, 3, 4, 5, 6, 7, 8, 1, 2, 3, 4, 5, 6, 7, 8, 1, 2, 3, 4, 5, 6, 7, 8,
            ]);
        }),
        succeeds_if({
            true
        }),
        post_condition({
            env.storage().persistent().get::<_, DataKey>(&DataKey::Counter(user)).unwrap_or(DataKey::SignerCnt) == DataKey::Data(BytesN::from_array(&[
                1u8, 2, 3, 4, 5, 6, 7, 8, 1, 2, 3, 4, 5, 6, 7, 8, 1, 2, 3, 4, 5, 6, 7, 8, 1, 2, 3, 4, 5, 6, 7, 8,
            ]))
    }))]
    pub fn increment(env: Env, user: Address, byte_data: BytesN<32>) -> DataKey {
        // Requires `user` to have authorized call of the `increment` of this
        // contract with all the arguments passed to `increment`, i.e. `user`
        // and `value`. This will panic if auth fails for any reason.
        // When this is called, Soroban host performs the necessary
        // authentication, manages replay prevention and enforces the user's
        // authorization policies.
        // The contracts normally shouldn't worry about these details and just
        // write code in generic fashion using `Address` and `require_auth` (or
        // `require_auth_for_args`).
        user.require_auth();

        // This call is equilvalent to the above:
        // user.require_auth_for_args((&user, value).into_val(&env));

        // The following has less arguments but is equivalent in authorization
        // scope to the above calls (the user address doesn't have to be
        // included in args as it's guaranteed to be authenticated).
        // user.require_auth_for_args((value,).into_val(&env));

        // Construct a key for the data being stored. Use an enum to set the
        // contract up well for adding other types of data to be stored.
        let key = DataKey::Counter(user);
        let data = DataKey::Data(byte_data.clone());

        // Save the count.
        env.storage().persistent().set(&key, &data);

        let return_value = env
            .storage()
            .persistent()
            .get(&key)
            .unwrap_or(DataKey::SignerCnt);

        return_value
    }
}

#[cfg(test)]
mod test {
    use core::panic;

    use super::*;

    #[test]
    fn test_increment() {
        let env = Env::default();
        let user = Address::new(&env);
        let bydata: BytesN<32> = BytesN::from_array(&[
            1u8, 2, 3, 4, 5, 6, 7, 8, 1, 2, 3, 4, 5, 6, 7, 8, 1, 2, 3, 4, 5, 6, 7, 8, 1, 2, 3, 4,
            5, 6, 7, 8,
        ]);
        let hello = IncrementContract::increment(env.clone(), user, bydata.clone());

        match hello {
            DataKey::Data(data) => {
                assert!(data == bydata)
            }
            _ => panic!("Failed"),
        }
    }
}

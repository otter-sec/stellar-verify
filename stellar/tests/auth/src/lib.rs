#![no_std]
use alloc::string::ToString;
use alloc::vec::Vec;
use soroban_sdk::{contract, contractimpl, contracttype, verify, Address, Env};
use soroban_sdk::{symbol_short, FromValEnum, Symbol, ToValEnum, Val};

#[macro_use]
extern crate alloc;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
// #[contracttype]
pub enum DataKey {
    SignerCnt,
    ZeroVal,
    Counter(Address),
}

impl ToValEnum for DataKey {
    fn to_val(&self) -> Val {
        match self {
            DataKey::SignerCnt => {
                Val::VecVal(vec![Val::SymbolVal(Symbol::new_from_str("SignerCnt"))])
            }
            DataKey::ZeroVal => Val::VecVal(vec![Val::SymbolVal(Symbol::new_from_str("ZeroVal"))]),
            DataKey::Counter(data) => Val::VecVal(vec![
                Val::SymbolVal(Symbol::new_from_str("Counter")),
                data.to_val(),
            ]),
        }
    }
}
impl FromValEnum for DataKey {
    fn from_val(val: Val) -> Option<Self> {
        match val {
            Val::VecVal(vec) => match &vec[0] {
                Val::SymbolVal(sym) => match sym.to_string().as_str() {
                    "SignerCnt" => Some(DataKey::SignerCnt),
                    _ => None,
                    "ZeroVal" => Some(DataKey::ZeroVal),
                    _ => None,
                    "Counter" => Some(DataKey::Counter(vec[1].clone().into())),
                    _ => None,
                },
                _ => None,
            },
            _ => None,
        }
    }
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
            let value = 100;
        }),
        succeeds_if({
            true
        }),
        post_condition({
            // env.storage().instance().get::<_, u32>(&COUNTER).unwrap_or(0) == 0
            true
        }),
    )]
    pub fn increment(env: Env, user: Address, value: u32) -> u32 {
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
        let key = DataKey::SignerCnt;
        let value = DataKey::ZeroVal;

        // Save the count.
        env.storage().persistent().set(&key, &value);

        let return_value = env
            .storage()
            .persistent()
            .get(&key)
            .unwrap_or(Val::VecVal(vec![Val::SymbolVal(Symbol::new_from_str(
                "SignerCnt",
            ))]));

        let val = DataKey::from_val(return_value).unwrap();
        assert!(val == DataKey::ZeroVal);

        // Return the count to the caller.
        0u32
    }
}

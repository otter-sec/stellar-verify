#![no_std]

#[contract]
struct SimpleAccount;

use soroban_sdk::{
    auth::Context, contract, contractimpl, symbol_short, Address, BytesN, Env, EnvTrait, Symbol,
    Vec,
};

const ADMIN: Symbol = symbol_short!("ADMIN");

#[contractimpl]
impl SimpleAccount {
    // Initialize the contract with an owner's ed25519 public key.
    #[cfg_attr(any(kani, feature = "kani"), 
        verify,
        init({
            env.storage().instance().set(&ADMIN, &kani::any::<BytesN<32>>());
        }),
        succeeds_if({
            !env.storage().instance().has(&ADMIN)
        })
    )]
    pub fn init(env: Env, public_key: BytesN<32>) {
        if env.storage().instance().has(&ADMIN) {
            panic!("owner is already set");
        }
        env.storage().instance().set(&ADMIN, &public_key);
    }

    // This is the 'entry point' of the account contract and every account
    // contract has to implement it. `require_auth` calls for the Address of
    // this contract will result in calling this `__check_auth` function with
    // the appropriate arguments.
    //
    // This should return `()` if authentication and authorization checks have
    // been passed and return an error (or panic) otherwise.
    //
    // `__check_auth` takes the payload that needed to be signed, arbitrarily
    // typed signatures (`BytesN<64>` type here) and authorization
    // context that contains all the invocations that this call tries to verify
    // (not used in this example).
    //
    // In this example `__check_auth` only verifies the signature.
    //
    // Note, that `__check_auth` function shouldn't call `require_auth` on the
    // contract's own address in order to avoid infinite recursion.
    #[allow(non_snake_case)]
    pub fn __check_auth(
        env: Env,
        signature_payload: BytesN<32>,
        signature_args: Vec<BytesN<64>>,
        _auth_context: Vec<Context>,
    ) {
        if signature_args.len() != 1 {
            panic!("incorrect number of signature args");
        }
        let public_key: BytesN<32> = env
            .storage()
            .instance()
            .get::<_, BytesN<32>>(&ADMIN)
            .unwrap();
        env.crypto().ed25519_verify(
            &public_key,
            &signature_payload.into(),
            &signature_args.get(0).unwrap(),
        );
    }
}

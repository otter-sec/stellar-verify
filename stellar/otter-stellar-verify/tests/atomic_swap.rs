//! This contract performs an atomic token swap between two parties.
//! Parties don't need to know each other and their signatures may be matched
//! off-chain.
//! This example demonstrates how multi-party authorization can be implemented.
#![no_std]
use otter_stellar_verify as soroban_sdk;
use soroban_sdk::{contract, contractimpl, token, Address, Env, IntoVal};

#[contract]
pub struct AtomicSwapContract;

#[contractimpl]
impl AtomicSwapContract {
    // Swap token A for token B atomically. Settle for the minimum requested price
    // for each party (this is an arbitrary choice; both parties could have
    // received the full amount as well).
    #[allow(clippy::too_many_arguments)]
    pub fn swap(
        env: Env,
        a: Address,
        b: Address,
        token_a: Address,
        token_b: Address,
        amount_a: i128,
        min_b_for_a: i128,
        amount_b: i128,
        min_a_for_b: i128,
    ) {
        // Verify preconditions on the minimum price for both parties.
        if amount_b < min_b_for_a {
            panic!("not enough token B for token A");
        }
        if amount_a < min_a_for_b {
            panic!("not enough token A for token B");
        }
        // Require authorization for a subset of arguments specific to a party.
        // Notice, that arguments are symmetric - there is no difference between
        // `a` and `b` in the call and hence their signatures can be used
        // either for `a` or for `b` role.
        a.require_auth_for_args((token_a, token_b, amount_a, min_b_for_a).into_val(&env));
        b.require_auth_for_args((token_b, token_a, amount_b, min_a_for_b).into_val(&env));

        // Perform the swap by moving tokens from a to b and from b to a.
        move_token(&env, &token_a, &a, &b, amount_a, min_a_for_b);
        move_token(&env, &token_b, &b, &a, amount_b, min_b_for_a);
    }
}

fn move_token(
    env: &Env,
    token: &Address,
    from: &Address,
    to: &Address,
    max_spend_amount: i128,
    transfer_amount: i128,
) {
    let token = token::Client::new(env, token);
    let contract_address = env.current_contract_address();
    // This call needs to be authorized by `from` address. It transfers the
    // maximum spend amount to the swap contract's address in order to decouple
    // the signature from `to` address (so that parties don't need to know each
    // other).
    token.transfer(from, &contract_address, &max_spend_amount);
    // Transfer the necessary amount to `to`.
    token.transfer(&contract_address, to, &transfer_amount);
    // Refund the remaining balance to `from`.
    token.transfer(
        &contract_address,
        from,
        &(max_spend_amount - transfer_amount),
    );
}

#[cfg(test)]
mod test {
    #![cfg(test)]
    extern crate std;

    use super::*;
    use otter_stellar_verify::{token, Address, Env};
    use token::AdminClient as TokenAdminClient;
    use token::Client as TokenClient;

    fn create_token_contract(e: &Env, admin: Address) -> (TokenClient, TokenAdminClient) {
        let contract_address = e.register_stellar_asset_contract(admin);
        (
            TokenClient::new(e, &contract_address),
            TokenAdminClient::new(e, &contract_address),
        )
    }

    #[test]
    fn test_atomic_swap() {
        let env = Env::default();
        env.mock_all_auths();

        let a = Address::new(&env);
        let b = Address::new(&env);

        let token_admin = Address::new(&env);

        let (token_a, token_a_admin) = create_token_contract(&env, token_admin);
        let (token_b, token_b_admin) = create_token_contract(&env, token_admin);

        token_a_admin.mint(&a, &1000);
        token_b_admin.mint(&b, &5000);

        let _ = env.register_contract(None, AtomicSwapContract);
        AtomicSwapContract::swap(
            env.clone(),
            a,
            b,
            token_a.address,
            token_b.address,
            1000,
            4500,
            5000,
            950,
        );

        assert_eq!(token_a.balance(&a), 50);
        assert_eq!(token_a.balance(&b), 950);

        assert_eq!(token_b.balance(&a), 4500);
        assert_eq!(token_b.balance(&b), 500);
    }
}

#[cfg(kani)]
mod verification {

    use super::*;
    use otter_stellar_verify::{Address, Env};
    use token::AdminClient as TokenAdminClient;
    use token::Client as TokenClient;

    fn create_token_contract<'a>(e: &Env, admin: &Address) -> (TokenClient, TokenAdminClient) {
        let contract_address = e.register_stellar_asset_contract(admin.clone());
        (
            TokenClient::new(e, &contract_address),
            TokenAdminClient::new(e, &contract_address),
        )
    }

    #[kani::proof]
    #[kani::unwind(10)]
    #[kani::solver(cadical)]
    fn verify() {
        let env = Env::default();

        let a = Address::new(&env);
        let b = Address::new(&env);
        let token_admin = Address::new(&env);

        let (token_a, token_a_admin) = create_token_contract(&env, &token_admin);
        let amount_a = kani::any::<i128>();
        kani::assume(amount_a > 0);
        token_a_admin.mint(&a, &amount_a);

        let (token_b, token_b_admin) = create_token_contract(&env, &token_admin);
        let amount_b = kani::any::<i128>();
        kani::assume(amount_b > 0);
        token_b_admin.mint(&b, &amount_b);

        let min_b_for_a = kani::any::<i128>();
        let min_a_for_b = kani::any::<i128>();

        kani::assume(amount_b > min_b_for_a);
        kani::assume(amount_a > min_a_for_b);

        let _ = env.register_contract(None, AtomicSwapContract);
        // Call the contract.
        AtomicSwapContract::swap(
            env.clone(),
            a.clone(),
            b.clone(),
            token_a.address.clone(),
            token_b.address.clone(),
            amount_a,
            min_b_for_a,
            amount_b,
            min_a_for_b,
        );

        // Verify the postconditions.
        assert_eq!(token_a.balance(&a), (amount_a - min_a_for_b));
        assert_eq!(token_a.balance(&b), min_a_for_b);

        assert_eq!(token_b.balance(&a), min_b_for_a);
        assert_eq!(token_b.balance(&b), (amount_b - min_b_for_a));
    }
}

# stellar-verify

- We have developed a Formal Verification framework for the Soroban contracts. This framework is based on the Soroban SDK framework.
  - The framework is based on the Kani tool, which is a model checker for Rust programs.
  - The framework provides a `verify` macro that can be integrated into the entrypoint function of our contracts. This macro grants access to several other macros, such as `init`, `succeeds_if`, and `post-conditions`.
  - The `init` macro is responsible for initialising all the variables that the contract call relies on.
  - The `succeeds_if` macro is executed before the contract call, with the assumption that the pre-conditions hold true prior to the contract call.
  - The `post_condition` macro allows us to specify post-conditions. It runs after the contract call, enabling us to assert that the post-conditions following the contract execution.
  - We have completed the implementation of the required macros for contract and verification.
- We have successfully implemented the Token and its essential functions for mocking mint, transfer and other token operations.
- We have developed some MockStorage for testing purposes.

## Usage

- To use the formal verification we need to mark the function with the `#[verify]` attribute.

```rust
#[verify]
pub fn swap(...){
    ...
}
```

- Generally all the input parameters are initialised using `kani::any` macro.
- But we can also use the `init` macro to initialise the input parameters manually.

```rust
    #[init({
        let token_admin = Address::new(&env);

        let (token_a_client, token_a_admin) = Self::create_token_contract(&env, &token_admin);
        let token_a = token_a_client.address;
        kani::assume(amount_a > 0);
        token_a_admin.mint(&a, &amount_a);

        let (token_b_client, token_b_admin) = Self::create_token_contract(&env, &token_admin);
        let token_b = token_b_client.address;
        kani::assume(amount_b > 0);
        token_b_admin.mint(&b, &amount_b);
    })]
```

- To assume the pre-conditions we can use the `succeeds_if` macro.

```rust
    #[succeeds_if(
        min_b_for_a < amount_b 
        && min_a_for_b < amount_a
        && min_a_for_b > 0
        && min_b_for_a > 0
    )]
```

- To assert the post-conditions we can use the `post_condition` macro.

```rust
    #[post_condition(
        token_a_client.balance(&a) == (amount_a - min_a_for_b) &&
        token_a_client.balance(&b) == min_a_for_b &&
        token_b_client.balance(&a) == min_b_for_a &&
        token_b_client.balance(&b) == (amount_b - min_b_for_a)
    )]
```

- Note that without the `#[verify]` attribute, the contract will not be verified and init, succeeds_if and post_condition macros will not be availabe.

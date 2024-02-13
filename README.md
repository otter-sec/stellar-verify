# Stellar Verify

## Introduction

We've developed a Formal Verification framework for [Soroban contracts](https://soroban.stellar.org/) using Kani. This framework will substitute built-in SDK functions and structures with less expensive ones to accelerate the verification process.

## Macros

This framework provides a **`verify`** macro, which can be added to a function to indicate it for the verification process. This macro grants access to several other macros, such as **`init`**, **`succeeds_if`**, and **`post_conditions`**. 

At a high level:

1. The **`init`** macro is responsible for initializing all variables that the contract call relies on.
2. The **`succeeds_if`** macro executes before the contract call, assuming that the pre-conditions hold true. 
3. The **`post_condition`** macro enables specifying post-conditions, running after the contract call to assert the post-conditions following the contract execution.

## Usage

To integrate formal verification using the Soroban SDK into your project, you'll need to follow a few steps outlined below.

Firstly, ensure you've cloned the **`stellar-verify`** repository to your local machine.

Next, navigate to the **`Cargo.toml`** file of the program you intend to verify. In this file, locate the **`dependencies`** and **`dev_dependencies`** sections, and update them to reference the Soroban SDK dependency as follows:

```toml
[dependencies]
soroban-sdk = { package="otter-stellar-verify", path="/path/to/stellar-verify/stellar/otter-stellar-verify"}

[features]
kani = ["soroban-sdk/kani"]

```

After updating the dependencies, import the required modules from the Soroban SDK into your target contract. Optionally, you can use a **`cfg`** directive to conditionally import these modules only when running with Kani:

```rust
#[cfg(kani)]
use soroban_sdk::{verify, Address, FromValEnum, ToValEnum, Val, Vec};

```

Next, annotate the desired function with the **`#[verify]`** attribute. This signals the framework to include the function in the verification process. For instance:

```rust
#[verify]
pub fn swap(...){
    ...
}

```

In the typical scenario, input parameters are initialized using the **`kani::any`** macro. However, manual initialization of input parameters is also possible through the **`init`** macro. This allows for more granular control over the initialization process, as demonstrated below:

```rust
#[verify]
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

Pre-conditions are assumed using the **`succeeds_if`** macro. This macro allows developers to specify conditions that must hold true before the contract call executes. An example of pre-condition usage is outlined below:

```rust
#[verify]
#[init({
	...
)}]
#[succeeds_if(
    min_b_for_a < amount_b
    && min_a_for_b < amount_a
    && min_a_for_b > 0
    && min_b_for_a > 0
)]

```

Following the execution of the contract call, post-conditions can be asserted using the **`post_condition`** macro. This enables developers to verify specific outcomes or states resulting from the contract's execution, as illustrated in the example below:

```rust
#[verify]
#[init({
	...
)}]
#[succeeds_if(
	...
)}]
#[post_condition(
    token_a_client.balance(&a) == (amount_a - min_a_for_b) &&
    token_a_client.balance(&b) == min_a_for_b &&
    token_b_client.balance(&a) == min_b_for_a &&
    token_b_client.balance(&b) == (amount_b - min_b_for_a)
)]

```

Once you've annotated your contract functions with the necessary macros provided by the Framework, you're ready to commence the verification process. You can initiate verification using the **`cargo kani`** command, which will analyze your entire contract for verification.

Alternatively, if you prefer to verify a specific function within your contract, you can use the **`cargo kani --harness <function_name>`** command. This command allows you to focus the verification efforts on a particular function, ensuring thorough scrutiny of its behavior and adherence to specifications.

It's important to note that without the **`#[verify]`** attribute, the contract will not undergo the verification process, and the **`init`**, **`succeeds_if`**, and **`post_condition`** macros will not be accessible.

## Example Verification

For a practical demonstration of formal verification in action, interested parties can explore the following example project: [Atomic Swap](https://github.com/otter-sec/stellar-verify/tree/main/stellar/tests/atomic-swap). This project showcases how formal verification techniques were applied to verify the correctness of a smart contract implementation within the Stellar blockchain ecosystem.

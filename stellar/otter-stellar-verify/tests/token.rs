use otter_stellar_verify::{token, Address, Env};
use token::AdminClient as TokenAdminClient;
use token::TokenClient;

fn create_token_contract(e: &Env, admin: Address) -> (TokenClient, TokenAdminClient) {
    let contract_address = e.register_stellar_asset_contract(admin);
    (
        TokenClient::new(e, &contract_address),
        TokenAdminClient::new(e, &contract_address),
    )
}

#[test]
fn test_mint_and_transfer() {
    let env = Env::default();

    let a = Address::new(&env);
    let b = Address::new(&env);
    assert!(a != b);
    let token_admin = Address::new(&env);

    let (token_a, token_a_admin) = create_token_contract(&env, token_admin);
    let (token_b, token_b_admin) = create_token_contract(&env, token_admin);

    token_a_admin.mint(&a, &1000);
    assert_eq!(token_a.balance(&a), 1000);

    token_b_admin.mint(&b, &1000);
    assert_eq!(token_b.balance(&b), 1000);

    token_a.transfer(&a, &b, &90);
    assert_eq!(token_a.balance(&a), 910);
    assert_eq!(token_a.balance(&b), 90);
    token_b.transfer(&b, &a, &10);
    assert_eq!(token_b.balance(&a), 10);
    assert_eq!(token_b.balance(&b), 990);
}

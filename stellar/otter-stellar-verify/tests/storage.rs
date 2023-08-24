use otter_stellar_verify::Env;

#[test]
fn test_storage() {
    let env = Env::default();
    let value = env.storage().instance().get(&"key".to_string());
}

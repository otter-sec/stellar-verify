use soroban_sdk::{contracttype, Address, BytesN, Symbol};
extern crate alloc;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
#[contracttype]
pub struct Counter {
    pub count: u64,
    pub increments: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
#[contracttype]
pub struct User {
    pub address: Address,
    pub name: Symbol,
    pub age: u64,
    pub data: BytesN<32>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[contracttype]
pub enum DataKey {
    SignerCnt,
    ZeroVal,
    Counter(Address),
}

#[cfg(test)]
mod test {

    use soroban_sdk::Env;

    use super::*;

    #[test]
    fn test_counter() {
        let counter = Counter {
            count: 108,
            increments: 10,
        };
        let serialized = counter.serialize();
        let deserialized = Counter::deserialize(&serialized);
        assert!(deserialized == counter);
    }

    #[test]
    fn test_user() {
        let env = Env::default();
        let user = User {
            address: Address::new(&env),
            name: Symbol::from("test"),
            age: 18,
            data: BytesN::from_array(&[
                1u8, 2, 3, 4, 5, 6, 7, 8, 1, 2, 3, 4, 5, 6, 7, 8, 1, 2, 3, 4, 5, 6, 7, 8, 1, 2, 3,
                4, 5, 6, 7, 8,
            ]),
        };
        let serialized = user.serialize();
        let deserialized = User::deserialize(&serialized);
        assert!(deserialized == user);
    }

    #[test]
    fn test_storage() {
        let env = Env::default();
        let key = DataKey::SignerCnt;
        let value = 10;
        env.storage().persistent().set(&key, &value);
        assert!(env.storage().persistent().has(&key));
    }
}

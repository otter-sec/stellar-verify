use soroban_sdk::contracttype;
extern crate alloc;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
#[contracttype]
pub struct Counter {
    pub count: u64,
    pub increments: u64,
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_increment() {
        let counter = Counter {
            count: 108,
            increments: 10,
        };
        let serialized = counter.serialize();
        let deserialized = Counter::deserialize(&serialized);
        assert!(deserialized == counter);
    }
}

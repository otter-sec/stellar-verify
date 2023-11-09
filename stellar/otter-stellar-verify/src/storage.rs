use std::cell::{RefCell, RefMut};
use std::fmt::Debug;
use std::rc::Rc;

use soroban_env_common::{FromValEnum, ToValEnum, Val, Vec};

use crate::token::MockToken;
use crate::Address;

#[derive(Clone, Default)]
pub struct Storage {
    tokens: Vec<MockToken>,
    instance: Rc<RefCell<InstanceStorage>>,
    temporary: Rc<RefCell<TemporaryStorage>>,
    persistent: Rc<RefCell<PersistentStorage>>,
}

#[derive(Clone, Default, Debug)]
pub struct InstanceStorage {
    storage: Vec<(Val, Val)>,
}

#[derive(Clone, Default, Debug)]
pub struct TemporaryStorage {
    storage: Vec<(Val, Val)>,
}

#[derive(Clone, Default, Debug)]
pub struct PersistentStorage {
    storage: Vec<(Val, Val)>,
}

impl PersistentStorage {
    pub fn get<K, V>(&self, key: &K) -> Option<V>
    where
        K: ToValEnum,
        V: FromValEnum,
    {
        let matched = self.storage.iter().find(|(k, _)| *k == key.to_val());
        if let Some((_, v)) = matched {
            V::from_val(v.clone())
        } else {
            None
        }
    }

    pub fn set<K, V>(&mut self, key: &K, val: &V)
    where
        K: ToValEnum,
        V: ToValEnum,
    {
        let exists = self.storage.iter().position(|(k, _)| *k == key.to_val());

        match exists {
            Some(index) => {
                self.storage[index].1 = val.to_val();
            }
            None => {
                self.storage.push((key.to_val(), val.to_val()));
            }
        }
    }

    pub fn has<K>(&self, key: &K) -> bool
    where
        K: ToValEnum,
    {
        self.storage.iter().any(|(k, _)| *k == key.to_val())
    }

    pub fn bump<K>(&self, _: K, _: u32, _: u32) {}
}

impl TemporaryStorage {
    pub fn get<K, V>(&self, key: &K) -> Option<V>
    where
        K: ToValEnum,
        V: FromValEnum,
    {
        let matched = self.storage.iter().find(|(k, _)| *k == key.to_val());
        if let Some((_, v)) = matched {
            V::from_val(v.clone())
        } else {
            None
        }
    }

    pub fn set<K, V>(&mut self, key: &K, val: &V)
    where
        K: ToValEnum,
        V: ToValEnum,
    {
        let exists = self.storage.iter().position(|(k, _)| *k == key.to_val());

        match exists {
            Some(index) => {
                self.storage[index].1 = val.to_val();
            }
            None => {
                self.storage.push((key.to_val(), val.to_val()));
            }
        }
    }

    pub fn has<K>(&self, key: &K) -> bool
    where
        K: ToValEnum,
    {
        self.storage.iter().any(|(k, _)| *k == key.to_val())
    }

    pub fn bump<K>(&self, _: K, _: u32, _: u32) {}
}

impl InstanceStorage {
    pub fn get<K, V>(&self, key: &K) -> Option<V>
    where
        K: ToValEnum,
        V: FromValEnum,
    {
        let matched = self.storage.iter().find(|(k, _)| *k == key.to_val());
        if let Some((_, v)) = matched {
            V::from_val(v.clone())
        } else {
            None
        }
    }

    pub fn set<K, V>(&mut self, key: &K, val: &V)
    where
        K: ToValEnum,
        V: ToValEnum,
    {
        let exists = self.storage.iter().position(|(k, _)| *k == key.to_val());

        match exists {
            Some(index) => {
                self.storage[index].1 = val.to_val();
            }
            None => {
                self.storage.push((key.to_val(), val.to_val()));
            }
        }
    }

    pub fn has<K>(&self, key: &K) -> bool
    where
        K: ToValEnum,
    {
        self.storage.iter().any(|(k, _)| *k == key.to_val())
    }

    pub fn bump(&self, _: u32, _: u32) {}
}

impl Debug for Storage {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Storage")
            .field("tokens", &self.tokens)
            .field("instance", &self.instance)
            .field("temporary", &self.temporary)
            .field("persistent", &self.persistent)
            .finish()
    }
}

impl Storage {
    pub fn get_token(&self, address: &Address) -> Option<MockToken> {
        self.tokens.iter().find(|t| t.address == *address).cloned()
    }

    pub fn instance(&self) -> RefMut<InstanceStorage> {
        self.instance.borrow_mut()
    }

    pub fn temporary(&self) -> RefMut<TemporaryStorage> {
        self.temporary.borrow_mut()
    }

    pub fn persistent(&self) -> RefMut<PersistentStorage> {
        self.persistent.borrow_mut()
    }

    pub fn insert_token(&mut self, token: MockToken) {
        self.tokens.push(token);
    }

    pub fn update_token(&mut self, token: MockToken) {
        //get the index of the token
        let index = self
            .tokens
            .iter()
            .position(|t| t.address == token.address)
            .unwrap();
        //replace the token
        self.tokens[index] = token;
    }
}

#[cfg(test)]
mod test {
    use soroban_env_common::Symbol;
    use stellar_sdk_macros::symbol_short;

    use crate::Env;

    #[test]
    pub fn test_set_get_instance() {
        let env = Env::default();
        let counter: Symbol = symbol_short!("COUNTER");
        let count = 1;

        env.storage().instance().set(&counter, &count);

        let result = env.storage().instance().get(&counter).unwrap_or(0);
        assert_eq!(result, count);
    }

    #[test]
    pub fn test_clone() {
        let env = Env::default();
        let env_clone = env.clone();
        let counter: Symbol = symbol_short!("COUNTER");
        increment(env, counter, 101);
        let result = env_clone.storage().instance().get(&counter).unwrap_or(0);
        assert_eq!(result, 101);
    }

    pub fn increment(env: Env, counter: Symbol, count: i32) {
        env.storage().instance().set(&counter, &count);
    }
}

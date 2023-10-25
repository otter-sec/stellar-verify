use std::fmt::Debug;

use soroban_env_common::{FromValEnum, ToValEnum, Val};

use crate::token::MockToken;
use crate::Address;

#[derive(Clone, Default)]
pub struct Storage {
    tokens: Vec<MockToken>,
    instance: InstanceStorage,
    temporary: TemporaryStorage,
    persistent: PersistentStorage,
}

#[derive(Clone, Default)]
pub struct InstanceStorage {
    storage: Vec<(Val, Val)>,
}

#[derive(Clone, Default)]
pub struct TemporaryStorage {
    storage: Vec<(Val, Val)>,
}

#[derive(Clone, Default)]
pub struct PersistentStorage {
    storage: Vec<(Val, Val)>,
}

impl PersistentStorage {
    pub fn get<K, V>(&self, key: &K) -> Option<V>
    where
        K: ToValEnum,
        V: FromValEnum,
    {
        let matched = self.storage.iter().find(|(k, _)| k == &key.to_val());
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
        self.storage.push((key.to_val(), val.to_val())); // Convert key and val to ValEnum
    }

    pub fn has<K>(&self, key: &K) -> bool
    where
        K: ToValEnum,
    {
        self.storage.iter().any(|(k, _)| k == &key.to_val())
    }

    pub fn bump<K>(&self, _: K, _: u32, _: u32) {}
}

impl TemporaryStorage {
    pub fn get<K, V>(&self, key: &K) -> Option<V>
    where
        K: ToValEnum,
        V: FromValEnum,
    {
        let matched = self.storage.iter().find(|(k, _)| k == &key.to_val());
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
        self.storage.push((key.to_val(), val.to_val())); // Convert key and val to ValEnum
    }

    pub fn has<K>(&self, key: &K) -> bool
    where
        K: ToValEnum,
    {
        self.storage.iter().any(|(k, _)| k == &key.to_val())
    }

    pub fn bump<K>(&self, _: K, _: u32, _: u32) {}
}

impl InstanceStorage {
    pub fn get<K, V>(&self, key: &K) -> Option<V>
    where
        K: ToValEnum,
        V: FromValEnum,
    {
        let matched = self.storage.iter().find(|(k, _)| k == &key.to_val());
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
        self.storage.push((key.to_val(), val.to_val())); // Convert key and val to ValEnum
    }

    pub fn has<K>(&self, key: &K) -> bool
    where
        K: ToValEnum,
    {
        self.storage.iter().any(|(k, _)| k == &key.to_val())
    }

    pub fn bump(&self, _: u32, _: u32) {}
}

impl Debug for Storage {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Storage")
    }
}

impl Storage {
    pub fn get_token(&self, address: &Address) -> Option<MockToken> {
        self.tokens.iter().find(|t| t.address == *address).cloned()
    }

    pub fn instance(&self) -> InstanceStorage {
        self.instance.clone()
    }

    pub fn temporary(&self) -> TemporaryStorage {
        self.temporary.clone()
    }

    pub fn persistent(&self) -> PersistentStorage {
        self.persistent.clone()
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

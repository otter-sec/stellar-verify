use std::cell::{Ref, RefCell};
use std::fmt::Debug;
use std::rc::Rc;

use crate::{token::MockToken, Address, FromValEnum, ToValEnum, Val, Vec};

#[derive(Clone, Default)]
pub struct Storage {
    tokens: Vec<MockToken>,
    instance: Rc<RefCell<InstanceStorage>>,
    temporary: Rc<RefCell<TemporaryStorage>>,
    persistent: Rc<RefCell<PersistentStorage>>,
}

#[derive(Clone, Default, Debug)]
pub struct InstanceStorage {
    storage: Rc<RefCell<Vec<(Val, Val)>>>,
}

#[derive(Clone, Default, Debug)]
pub struct TemporaryStorage {
    storage: Rc<RefCell<Vec<(Val, Val)>>>,
}

#[derive(Clone, Default, Debug)]
pub struct PersistentStorage {
    storage: Rc<RefCell<Vec<(Val, Val)>>>,
}

impl PersistentStorage {
    pub fn get<K, V>(&self, key: &K) -> Option<V>
    where
        K: ToValEnum,
        V: FromValEnum,
    {
        let storage = self.storage.borrow();
        let matched = storage.iter().find(|(k, _)| *k == key.to_val());
        if let Some((_, v)) = matched {
            V::from_val(v.clone())
        } else {
            None
        }
    }

    pub fn set<K, V>(&self, key: &K, val: &V)
    where
        K: ToValEnum,
        V: ToValEnum,
    {
        let exists = self
            .storage
            .borrow_mut()
            .iter()
            .position(|(k, _)| *k == key.to_val());

        match exists {
            Some(index) => {
                self.storage.borrow_mut()[index].1 = val.to_val();
            }
            None => {
                self.storage.borrow_mut().push((key.to_val(), val.to_val()));
            }
        }
    }

    pub fn has<K>(&self, key: &K) -> bool
    where
        K: ToValEnum,
    {
        self.storage
            .borrow()
            .iter()
            .any(|(k, _)| *k == key.to_val())
    }

    pub fn extend_ttl<K>(&self, _: K, _: u32, _: u32) {}
}

impl TemporaryStorage {
    pub fn get<K, V>(&self, key: &K) -> Option<V>
    where
        K: ToValEnum,
        V: FromValEnum,
    {
        let storage = self.storage.borrow();
        let matched = storage.iter().find(|(k, _)| *k == key.to_val());
        if let Some((_, v)) = matched {
            V::from_val(v.clone())
        } else {
            None
        }
    }

    pub fn set<K, V>(&self, key: &K, val: &V)
    where
        K: ToValEnum,
        V: ToValEnum,
    {
        let exists = self
            .storage
            .borrow_mut()
            .iter()
            .position(|(k, _)| *k == key.to_val());

        match exists {
            Some(index) => {
                self.storage.borrow_mut()[index].1 = val.to_val();
            }
            None => {
                self.storage.borrow_mut().push((key.to_val(), val.to_val()));
            }
        }
    }

    pub fn has<K>(&self, key: &K) -> bool
    where
        K: ToValEnum,
    {
        self.storage
            .borrow()
            .iter()
            .any(|(k, _)| *k == key.to_val())
    }

    pub fn extend_ttl<K>(&self, _: K, _: u32, _: u32) {}
}

impl InstanceStorage {
    pub fn get<K, V>(&self, key: &K) -> Option<V>
    where
        K: ToValEnum,
        V: FromValEnum,
    {
        let storage = self.storage.borrow();
        let matched = storage.iter().find(|(k, _)| *k == key.to_val());
        if let Some((_, v)) = matched {
            V::from_val(v.clone())
        } else {
            None
        }
    }

    pub fn set<K, V>(&self, key: &K, val: &V)
    where
        K: ToValEnum,
        V: ToValEnum,
    {
        let exists = self
            .storage
            .borrow_mut()
            .iter()
            .position(|(k, _)| *k == key.to_val());

        match exists {
            Some(index) => {
                self.storage.borrow_mut()[index].1 = val.to_val();
            }
            None => {
                self.storage.borrow_mut().push((key.to_val(), val.to_val()));
            }
        }
    }

    pub fn has<K>(&self, key: &K) -> bool
    where
        K: ToValEnum,
    {
        self.storage
            .borrow()
            .iter()
            .any(|(k, _)| *k == key.to_val())
    }

    pub fn extend_ttl(&self, _: u32, _: u32) {}
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

    pub fn instance(&self) -> Ref<InstanceStorage> {
        self.instance.borrow()
    }

    pub fn temporary(&self) -> Ref<TemporaryStorage> {
        self.temporary.borrow()
    }

    pub fn persistent(&self) -> Ref<PersistentStorage> {
        self.persistent.borrow()
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
    use crate::Symbol;

    use super::*;
    #[test]
    fn test_has_storage() {
        let storage = Storage::default();
        let instance = storage.instance();
        let symb = Symbol::from("test");
        let symb2 = Symbol::from("test1");
        let value = 10;
        instance.set(&symb, &value);
        assert!(instance.has(&symb));
        assert!(!instance.has(&symb2));
    }

    #[test]
    fn test_has_storage_with_enum() {
        let storage = Storage::default();
        let instance = storage.instance();
        let symb = Symbol::from("test");
        let symb2 = Symbol::from("test1");
        let value = 10;
        instance.set(&symb, &value);
        assert!(instance.has(&symb));
        assert!(!instance.has(&symb2));
    }
}

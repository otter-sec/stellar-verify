use std::fmt::Debug;

use crate::soroban_env_common::StorageType;

use crate::env::Env;

#[derive(Clone)]
pub struct Storage {
    env: Env,
}

impl Debug for Storage {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Storage")
    }
}

impl Storage {
    #[inline(always)]
    pub(crate) fn new(env: &Env) -> Storage {
        Storage { env: env.clone() }
    }

    pub fn persistent(&self) -> Persistent {
        Persistent {
            storage: self.clone(),
        }
    }

    pub fn temporary(&self) -> Temporary {
        Temporary {
            storage: self.clone(),
        }
    }

    pub fn instance(&self) -> Instance {
        Instance {
            storage: self.clone(),
        }
    }

    fn has_internal<K>(&self, key: &K, storage_type: StorageType) -> bool {
        todo!()
    }

    #[inline(always)]
    pub(crate) fn has<K>(&self, key: &K, storage_type: StorageType) -> bool {
        self.has_internal(key, storage_type)
    }

    #[inline(always)]
    pub(crate) fn get<K>(&self, key: &K, storage_type: StorageType) -> Option<V> {
        todo!()
    }

    fn get_internal<K>(&self, key: &K, storage_type: StorageType) {
        todo!()
    }

    #[inline(always)]
    pub(crate) fn set<K, V>(&self, key: &K, val: &V, storage_type: StorageType) -> bool {
        todo!()
    }

    pub(crate) fn bump<K>(&self, key: &K, storage_type: StorageType, min_ledgers_to_live: u32) {}

    #[inline(always)]
    pub(crate) fn remove<K>(&self, key: &K, storage_type: StorageType) -> bool {
        todo!()
    }
}

pub struct Persistent {
    storage: Storage,
}

impl Persistent {
    pub fn has<K>(&self, key: &K) -> bool {
        self.storage.has(key, StorageType::Persistent)
    }

    pub fn get<K>(&self, key: &K) -> Option<V> {
        self.storage.get(key, StorageType::Persistent)
    }

    pub fn set<K, V>(&self, key: &K, val: &V) -> bool {
        self.storage.set(key, val, StorageType::Persistent)
    }

    pub fn bump<K>(&self, key: &K, min_ledgers_to_live: u32) {
        self.storage
            .bump(key, StorageType::Persistent, min_ledgers_to_live)
    }

    pub fn remove<K>(&self, key: &K) -> bool {
        self.storage.remove(key, StorageType::Persistent)
    }
}

pub struct Temporary {
    storage: Storage,
}

impl Temporary {
    pub fn has<K>(&self, key: &K) -> bool {
        self.storage.has(key, StorageType::Temporary)
    }

    pub fn get<K>(&self, key: &K) -> Option<V> {
        self.storage.get(key, StorageType::Temporary)
    }

    pub fn set<K, V>(&self, key: &K, val: &V) -> bool {
        self.storage.set(key, val, StorageType::Temporary)
    }

    pub fn bump<K>(&self, key: &K, min_ledgers_to_live: u32) {
        self.storage
            .bump(key, StorageType::Temporary, min_ledgers_to_live)
    }

    pub fn remove<K>(&self, key: &K) -> bool {
        self.storage.remove(key, StorageType::Temporary)
    }
}

pub struct Instance {
    storage: Storage,
}

impl Instance {
    pub fn has<K>(&self, key: &K) -> bool {
        self.storage.has(key, StorageType::Instance)
    }

    pub fn get<K>(&self, key: &K) -> Option<V> {
        self.storage.get(key, StorageType::Instance)
    }

    pub fn set<K, V>(&self, key: &K, val: &V) -> bool {
        self.storage.set(key, val, StorageType::Instance)
    }

    pub fn bump<K>(&self, key: &K, min_ledgers_to_live: u32) {
        self.storage
            .bump(key, StorageType::Instance, min_ledgers_to_live)
    }

    pub fn remove<K>(&self, key: &K) -> bool {
        self.storage.remove(key, StorageType::Instance)
    }
}

use std::fmt::Debug;

use crate::{env::Env, soroban_env_common::StorageType};

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

    #[inline(always)]
    pub(crate) fn has<K>(&self, key: &K, storage_type: StorageType) -> bool {
        todo!()
    }

    #[inline(always)]
    pub(crate) fn get<K>(&self, key: &K, storage_type: StorageType) -> bool {
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

pub struct Temporary {
    storage: Storage,
}

pub struct Instance {
    storage: Storage,
}

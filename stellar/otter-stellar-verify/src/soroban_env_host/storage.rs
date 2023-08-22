use std::{collections::BTreeMap, rc::Rc};

use super::xdr::{LedgerEntry, LedgerKey};

pub type StorageMap = BTreeMap<Rc<LedgerKey>, Option<Rc<LedgerEntry>>>;

#[derive(Clone, Default)]
pub struct Storage {
    pub map: StorageMap,
}

impl Storage {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn has(&self, key: &LedgerKey) -> bool {
        self.map.contains_key(key)
    }

    pub fn get(&self, key: &LedgerKey) -> Option<Rc<LedgerEntry>> {
        self.map.get(key).cloned().flatten()
    }

    pub fn put(&mut self, key: &LedgerKey, value: &LedgerEntry) {
        self.map
            .insert(Rc::new(key.clone()), Some(Rc::new(value.clone())));
    }
}

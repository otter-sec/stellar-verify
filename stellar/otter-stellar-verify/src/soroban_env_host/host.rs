use std::{cell::RefCell, rc::Rc};

use crate::soroban_env_common::StorageType;
use crate::types::{ContractExecutable, Hash, ScContractInstance, ScErrorCode};

use super::error::HostError;
use super::storage::Storage;
use super::xdr::{LedgerEntry, LedgerKey, ScErrorType};

#[derive(Debug, Clone, Default)]
pub struct LedgerInfo {
    pub protocol_version: u32,
    pub sequence_number: u32,
    pub timestamp: u64,
    pub network_id: [u8; 32],
    pub base_reserve: u32,
    pub min_temp_entry_expiration: u32,
    pub min_persistent_entry_expiration: u32,
    pub max_entry_expiration: u32,
}

#[derive(Clone, Default)]
pub(crate) struct HostImpl {
    storage: RefCell<Storage>,
}

#[derive(Clone)]
pub struct Host(pub(crate) Rc<HostImpl>);

#[allow(clippy::derivable_impls)]
impl Default for Host {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl Host {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn storage(&self) -> RefCell<Storage> {
        self.0.storage.clone()
    }

    pub fn get_ledger_info(&self) -> LedgerInfo {
        LedgerInfo::default()
    }

    pub fn with_mut_storage<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut Storage) -> R,
    {
        f(&mut self.0.storage.borrow_mut())
    }

    pub fn add_ledger_entry(
        &self,
        key: &Rc<LedgerKey>,
        val: &Rc<LedgerEntry>,
    ) -> Result<(), HostError> {
        self.with_mut_storage(|storage| storage.put(key, val));
        Ok(())
    }

    pub fn has(&self, key: &LedgerKey) -> bool {
        self.0.storage.borrow().has(key)
    }
}

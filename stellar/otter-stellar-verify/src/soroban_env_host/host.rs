use std::{cell::RefCell, rc::Rc};

use sha2::{Digest, Sha256, Sha512};

use crate::soroban_env_common::StorageType;
use crate::types::{ContractExecutable, Hash, ScAddress, ScContractInstance, ScErrorCode, ScVal};
use crate::Val;

use super::error::HostError;
use super::storage::Storage;
use super::xdr::{ContractDataDurability, CreateContractArgs, LedgerEntry, LedgerKey, ScErrorType};

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
    ledger: RefCell<Option<LedgerInfo>>,
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
        self.0.ledger.borrow().clone().unwrap()
    }

    pub fn with_mut_storage<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut Storage) -> R,
    {
        f(&mut self.0.storage.borrow_mut())
    }

    pub fn get_ledger_network_id(&self) -> Result<[u8; 32], HostError> {
        Ok(self.get_ledger_info().network_id)
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

    pub(crate) fn storage_key_from_scval(
        &self,
        key: ScVal,
        durability: ContractDataDurability,
    ) -> Result<Rc<LedgerKey>, HostError> {
        todo!("Impl storage_key_from_scval")
    }

    pub fn storage_key_from_rawval(
        &self,
        k: Val,
        durability: ContractDataDurability,
    ) -> Result<Rc<LedgerKey>, HostError> {
        // let sc_val = ScVal::from(k);
        // self.storage_key_from_scval(sc_val, durability)
        todo!()
    }

    fn has_contract_data(&self, k: Val, t: StorageType) -> Result<bool, HostError> {
        todo!("Impl has_contract_data")
    }

    pub fn create_contract_internal(
        &self,
        deployer: Option<ScAddress>,
        args: CreateContractArgs,
    ) -> Result<ScAddress, HostError> {
        if deployer.is_some() {
            todo!("Impl if deployer is some")
        }
        let res = self.create_contract_with_optional_auth(deployer, args);
        res
    }

    fn metered_hash_xdr(&self, xdr: &[u8]) -> Result<[u8; 32], HostError> {
        let mut hasher = Sha256::new();
        hasher.update(xdr);
        let hash = hasher.finalize();
        Ok(hash.into())
    }

    fn create_contract_with_optional_auth(
        &self,
        deployer: Option<ScAddress>,
        args: CreateContractArgs,
    ) -> Result<ScAddress, HostError> {
        if deployer.is_some() {
            todo!("Impl if deployer is some")
        }

        let id_preimage = self.get_full_contract_id_preimage(args.contract_id_preimage)?;
        todo!("Impl create_contract_with_optional_auth");
    }
}

use std::rc::Rc;

use crate::types::{Hash, ScAddress, ScVal};

use super::{
    error::HostError,
    xdr::{ContractDataDurability, ContractEntryBodyType, LedgerKey, LedgerKeyContractData},
    Host,
};

impl Host {
    pub fn contract_instance_ledger_key(
        &self,
        contract_id: &Hash,
    ) -> Result<Rc<LedgerKey>, HostError> {
        let contract_id = contract_id.clone();
        Ok(Rc::new(LedgerKey::ContractData(LedgerKeyContractData {
            key: ScVal::LedgerKeyContractInstance,
            durability: ContractDataDurability::Persistent,
            body_type: ContractEntryBodyType::DataEntry,
            contract: ScAddress::Contract(contract_id),
        })))
    }
}

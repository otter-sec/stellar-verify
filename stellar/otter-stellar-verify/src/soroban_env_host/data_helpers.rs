use std::rc::Rc;

use crate::{
    token::BytesObject,
    types::{Hash, ScAddress, ScVal},
};
use sha2::{Digest, Sha256};

use super::{
    error::HostError,
    xdr::{
        ContractDataDurability, ContractEntryBodyType, ContractIdPreimage, HashIdPreimage,
        HashIdPreimageContractId, LedgerKey, LedgerKeyContractData,
    },
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

    pub fn get_full_contract_id_preimage(
        &self,
        init_preimage: ContractIdPreimage,
    ) -> Result<HashIdPreimage, HostError> {
        Ok(HashIdPreimage::ContractId(HashIdPreimageContractId {
            network_id: self
                .hash_from_bytesobj_input("network_id", self.get_ledger_network_id()?)?,
            contract_id_preimage: init_preimage,
        }))
    }

    pub(crate) fn hash_from_bytesobj_input(
        &self,
        name: &'static str,
        bytes_arr: [u8; 32],
    ) -> Result<Hash, HostError> {
        let hash: Hash = Hash(bytes_arr);
        Ok(hash)
    }
}

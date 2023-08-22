use std::rc::Rc;

use self::internal::xdr::{
    AccountEntry, AccountEntryExt, AlphaNum4, Asset, AssetCode4, ContractExecutable,
    ContractIdPreimage, CreateContractArgs, HostFunction, LedgerEntry, LedgerEntryData,
    LedgerEntryExt, LedgerKey, LedgerKeyAccount, SequenceNumber, Thresholds,
};

use crate::{
    address::Address,
    crypto::Crypto,
    events::Events,
    ledger::Ledger,
    random::random,
    soroban_ledger_snapshot::LedgerSnapshot,
    storage::Storage,
    types::{AccountId, Hash, PublicKey, ScAddress, Uint256, VecM},
};

pub mod internal {
    pub use crate::soroban_env_host::*;

    pub type EnvImpl = Host;
    pub type MaybeEnvImpl = Option<Host>;

    pub trait Env {}
}

#[derive(Clone)]
pub struct Env {
    env_impl: internal::EnvImpl,
    _snapshot: Option<Rc<LedgerSnapshot>>,
}
impl Default for Env {
    fn default() -> Self {
        Self::default_with_testutils()
    }
}
impl Env {
    pub fn host(&self) -> &internal::Host {
        &self.env_impl
    }

    fn default_with_testutils() -> Env {
        Env {
            env_impl: internal::EnvImpl::default(),
            _snapshot: None,
        }
    }

    pub fn current_contract_address(&self) -> Address {
        Address {
            obj: ScAddress::Contract(Hash::default()),
        }
    }

    #[inline(always)]
    pub fn storage(&self) -> Storage {
        Storage::new(self)
    }

    #[inline(always)]
    pub fn events(&self) -> Events {
        Events::new(self)
    }

    #[inline(always)]
    pub fn ledger(&self) -> Ledger {
        Ledger::new(self)
    }

    #[inline(always)]
    pub fn crypto(&self) -> Crypto {
        Crypto::new(self)
    }

    pub fn register_stellar_asset_contract(&self, admin: Address) -> Address {
        let contract_address = Address {
            obj: ScAddress::Contract(Hash::default()),
        };
        let issuer_pk = random();
        let issuer_id = AccountId(PublicKey(Uint256(issuer_pk)));
        self.host().with_mut_storage(|storage| {
            let k = Rc::new(LedgerKey::Account(LedgerKeyAccount {
                account_id: issuer_id.clone(),
            }));
            if !storage.has(&k) {
                let v = Rc::new(LedgerEntry {
                    data: LedgerEntryData::Account(AccountEntry {
                        account_id: issuer_id.clone(),
                        balance: 0,
                        flags: 0,
                        home_domain: Default::default(),
                        inflation_dest: None,
                        num_sub_entries: 0,
                        seq_num: SequenceNumber(0),
                        thresholds: Thresholds([1; 4]),
                        signers: VecM::default(),
                        ext: AccountEntryExt::V0,
                    }),
                    last_modified_ledger_seq: 0,
                    ext: LedgerEntryExt::V0,
                });
                storage.put(&k, &v);
            }
        });
        let asset = Asset::CreditAlphanum4(AlphaNum4 {
            asset_code: AssetCode4([b'a', b'a', b'a', b'a']),
            issuer: issuer_id.clone(),
        });

        let create = HostFunction::CreateContract(CreateContractArgs {
            contract_id_preimage: ContractIdPreimage::Asset(asset),
            executable: ContractExecutable::Token,
        });

        // TODO: create token

        contract_address
    }
}

pub trait IntoVal<E: internal::Env, T> {
    fn into_val(&self, e: &E) -> T;
}

#[derive(Clone, Default)]
pub struct MaybeEnv {
    _maybe_env_impl: internal::MaybeEnvImpl,
    _snapshot: Option<Rc<LedgerSnapshot>>,
}

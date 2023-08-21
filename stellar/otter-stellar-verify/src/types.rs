#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ScVal {
    Bool(bool),
    Void,
    Error(ScError),
    U32(u32),
    I32(i32),
    U64(u64),
    I64(i64),
    Timepoint(TimePoint),
    Duration(Duration),
    U128(UInt128Parts),
    I128(Int128Parts),
    U256(UInt256Parts),
    I256(Int256Parts),
    Bytes(ScBytes),
    String(ScString),
    Symbol(ScSymbol),
    Vec(Option<ScVec>),
    Map(Option<ScMap>),
    Address(ScAddress),
    LedgerKeyContractInstance,
    LedgerKeyNonce(ScNonceKey),
    ContractInstance(ScContractInstance),
}

#[derive(Clone, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(kani, derive(kani::Arbitrary))]
pub struct Uint256(pub [u8; 32]);

impl Uint256 {
    pub fn from_u64(v: u64) -> Self {
        let mut ret = [0; 32];
        ret[24..].copy_from_slice(&v.to_be_bytes());
        Self(ret)
    }

    pub fn from_u128(v: u128) -> Self {
        let mut ret = [0; 32];
        ret[16..].copy_from_slice(&v.to_be_bytes());
        Self(ret)
    }

    pub fn from_u32(v: u32) -> Self {
        let mut ret = [0; 32];
        ret[28..].copy_from_slice(&v.to_be_bytes());
        Self(ret)
    }

    pub const ZERO: Self = Uint256([0; 32]);
}

impl Default for Uint256 {
    fn default() -> Self {
        Self::ZERO
    }
}

#[derive(Clone, Hash, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
#[cfg_attr(kani, derive(kani::Arbitrary))]
pub struct Hash(pub [u8; 32]);

#[derive(Clone, Hash, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
#[cfg_attr(kani, derive(kani::Arbitrary))]
pub struct PublicKey(Uint256);

#[derive(Clone, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(kani, derive(kani::Arbitrary))]
pub struct AccountId(pub PublicKey);

#[derive(Clone, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ScAddress {
    Account(AccountId),
    Contract(Hash),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(kani, derive(kani::Arbitrary))]
pub struct UInt128Parts {
    pub hi: u64,
    pub lo: u64,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(kani, derive(kani::Arbitrary))]
pub struct Int128Parts {
    pub hi: i64,
    pub lo: u64,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(kani, derive(kani::Arbitrary))]
pub struct UInt256Parts {
    pub hi_hi: u64,
    pub hi_lo: u64,
    pub lo_hi: u64,
    pub lo_lo: u64,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(kani, derive(kani::Arbitrary))]
pub struct Int256Parts {
    pub hi_hi: i64,
    pub hi_lo: u64,
    pub lo_hi: u64,
    pub lo_lo: u64,
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
#[cfg_attr(kani, derive(kani::Arbitrary))]
pub struct Duration(pub u64);

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
#[cfg_attr(kani, derive(kani::Arbitrary))]
pub struct TimePoint(pub u64);

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct BytesM<const MAX: u32 = { u32::MAX }>(Vec<u8>);

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScBytes(pub BytesM);

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct StringM<const MAX: u32 = { u32::MAX }>(Vec<u8>);

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScString(pub StringM);

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScSymbol(pub StringM<32>);

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct VecM<T, const MAX: u32 = { u32::MAX }>(Vec<T>);

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScVec(pub VecM<ScVal>);

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScMapEntry {
    pub key: ScVal,
    pub val: ScVal,
}
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScMap(pub VecM<ScMapEntry>);

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ScError {}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
#[cfg_attr(kani, derive(kani::Arbitrary))]
pub struct ScNonceKey {
    pub nonce: i64,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScContractInstance {
    pub executable: ContractExecutable,
    pub storage: Option<ScMap>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ContractExecutable {
    Wasm(Hash),
    Token,
}

#[repr(i32)]
pub enum ScValType {
    Bool = 0,
    Void = 1,
    Error = 2,
    U32 = 3,
    I32 = 4,
    U64 = 5,
    I64 = 6,
    Timepoint = 7,
    Duration = 8,
    U128 = 9,
    I128 = 10,
    U256 = 11,
    I256 = 12,
    Bytes = 13,
    String = 14,
    Symbol = 15,
    Vec = 16,
    Map = 17,
    Address = 18,
    ContractInstance = 19,
    LedgerKeyContractInstance = 20,
    LedgerKeyNonce = 21,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct String64(pub StringM<64>);

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct String32(pub StringM<32>);

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SignerKeyEd25519SignedPayload {
    pub ed25519: Uint256,
    pub payload: BytesM<64>,
}

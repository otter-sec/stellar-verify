#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ScVal {
    Address(ScAddress),
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
pub struct PublicKey(pub Uint256);

#[derive(Clone, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(kani, derive(kani::Arbitrary))]
pub struct AccountId(pub PublicKey);

#[derive(Clone, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ScAddress {
    Account(AccountId),
    Contract(Hash),
}

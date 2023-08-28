#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ScVal {
    Address(ScAddress),
}

#[derive(Clone, Hash, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
#[cfg_attr(kani, derive(kani::Arbitrary))]
pub struct Hash(pub [u8; 32]);

#[derive(Clone, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ScAddress {
    Contract(Hash),
}

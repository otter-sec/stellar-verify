use crate::Env;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Bytes(pub Vec<u8>);

impl Bytes {
    pub fn new(_env: Env, bytes: Vec<u8>) -> Self {
        Self(bytes)
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }

    pub fn from_slice(bytes: &[u8]) -> Self {
        Self(bytes.to_vec())
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.0.clone()
    }

    pub fn to_le_bytes(&self) -> Vec<u8> {
        self.0.clone()
    }

    pub fn from_le_bytes(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BytesN<const N: usize>(pub Bytes);

impl<const N: usize> BytesN<N> {
    // Create a new `BytesN` instance from an array of u8
    pub fn from_array(arr: &[u8; N]) -> Self {
        Self(Bytes(arr.to_vec()))
    }

    pub fn to_le_bytes(&self) -> [u8; N] {
        self.0.to_le_bytes().try_into().unwrap()
    }

    pub fn from_le_bytes(bytes: [u8; N]) -> Self {
        Self(Bytes::from_le_bytes(bytes.to_vec()))
    }

    pub fn unchecked_new(env: Env, bytes: Vec<u8>) -> Self {
        let mut arr = [0; N];
        arr.copy_from_slice(&bytes);
        Self(Bytes::new(env, bytes))
    }
}

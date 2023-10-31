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

    pub fn set(&mut self, i: u32, v: u8) {
        let val = self.0.get_mut(i as usize).unwrap();
        *val = v;
    }

    pub fn get(&self, i: u32) -> Option<u8> {
        if i < self.len() {
            Some(self.get_unchecked(i))
        } else {
            None
        }
    }

    pub fn get_unchecked(&self, i: u32) -> u8 {
        *self.0.get(i as usize).unwrap()
    }

    pub fn len(&self) -> u32 {
        self.0.len() as u32
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn first(&self) -> Option<u8> {
        if !self.is_empty() {
            Some(self.first_unchecked())
        } else {
            None
        }
    }

    pub fn first_unchecked(&self) -> u8 {
        *self.0.get(0).unwrap()
    }

    pub fn last(&self) -> Option<u8> {
        if !self.is_empty() {
            Some(self.last_unchecked())
        } else {
            None
        }
    }

    pub fn last_unchecked(&self) -> u8 {
        *self.0.get(self.len() as usize - 1).unwrap()
    }

    pub fn remove(&mut self, i: u32) -> Option<()> {
        if i < self.len() {
            self.remove_unchecked(i);
            Some(())
        } else {
            None
        }
    }

    pub fn remove_unchecked(&mut self, i: u32) {
        self.0.remove(i as usize);
    }

    pub fn push_back(&mut self, x: u8) {
        self.0.push(x);
    }

    pub fn pop_back(&mut self) -> Option<u8> {
        let last = self.last()?;
        self.0.pop();
        Some(last)
    }

    pub fn pop_back_unchecked(&mut self) -> u8 {
        let last = self.last_unchecked();
        self.0.pop();
        last
    }

    pub fn insert(&mut self, i: u32, b: u8) {
        self.0.insert(i as usize, b);
    }

    pub fn iter(&self) -> std::slice::Iter<u8> {
        self.0.iter()
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

    pub fn set(&mut self, i: u32, v: u8) {
        self.0.set(i, v);
    }

    pub fn get(&self, i: u32) -> Option<u8> {
        self.0.get(i)
    }

    pub fn get_unchecked(&self, i: u32) -> u8 {
        self.0.get_unchecked(i)
    }

    pub fn is_empty(&self) -> bool {
        false
    }

    pub fn len(&self) -> u32 {
        N as u32
    }

    pub fn first(&self) -> Option<u8> {
        self.0.first()
    }

    pub fn first_unchecked(&self) -> u8 {
        self.0.first_unchecked()
    }

    pub fn last(&self) -> Option<u8> {
        self.0.last()
    }

    pub fn last_unchecked(&self) -> u8 {
        self.0.last_unchecked()
    }

    pub fn iter(&self) -> std::slice::Iter<u8> {
        self.0.iter()
    }
}

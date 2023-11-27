use soroban_env_common::{env::internal, Env, FromValEnum, IntoVal, ToValEnum, Vec};
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Bytes(pub Vec<u8>);

impl Bytes {
    pub fn new(_env: Env) -> Self {
        Self(Vec::new())
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }

    pub fn from_slice(bytes: &[u8]) -> Self {
        Bytes(Vec::new_from_slice(bytes))
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.0
    }

    pub fn to_le_bytes(&self) -> Vec<u8> {
        self.0
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
        *self.0.first().unwrap()
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

    pub fn append(&mut self, other: &Bytes) {
        self.0.extend_from_slice(other.as_slice());
    }

    pub fn insert(&mut self, i: u32, b: u8) {
        self.0.insert(i as usize, b);
    }

    pub fn iter(&self) -> soroban_env_common::vec::VecIterator<u8> {
        self.0.iter()
    }
}

#[cfg(any(kani, feature = "kani"))]
impl kani::Arbitrary for Bytes {
    fn any() -> Self {
        let mut v = Vec::new();
        for _ in 0..kani::any::<u8>() % 10 {
            v.push(kani::any());
        }
        Bytes(v)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BytesN<const N: usize>(pub Vec<u8>);

impl<const N: usize> Default for BytesN<N> {
    fn default() -> Self {
        BytesN(Vec::new())
    }
}

impl<const N: usize> ToValEnum for BytesN<N> {
    fn to_val(&self) -> crate::Val {
        crate::Val::BytesNVal(self.0)
    }
}

impl<const N: usize> FromValEnum for BytesN<N> {
    fn from_val(val: crate::Val) -> Option<Self> {
        if let crate::Val::BytesNVal(u) = val {
            // if u.len() == N / 8 {
            //     Some(BytesN(u))
            // } else {
            //     None
            // }
            Some(BytesN(u))
        } else {
            None
        }
    }
}

impl<const N: usize> From<soroban_env_common::Val> for BytesN<N> {
    fn from(val: crate::Val) -> Self {
        if let crate::Val::BytesNVal(u) = val {
            // if u.len() == N / 8 {
            //     BytesN(u)
            // } else {
            //     panic!("Error")
            // }
            BytesN(u)
        } else {
            panic!("Error")
        }
    }
}

impl<const N: usize> BytesN<N> {
    // Create a new `BytesN` instance from an array of u8
    pub fn from_array(arr: &[u8; N]) -> Self {
        let v: Vec<u8> = arr.iter().take(N).cloned().collect();
        BytesN(v)
    }

    pub fn to_le_bytes(&self) -> [u8; N] {
        let mut result = [0u8; N];

        // Copy as many bytes as possible, up to N
        let len = self.0.len().min(N);
        result[..len].copy_from_slice(&self.0[..len]);

        result
    }

    pub fn from_le_bytes(bytes: [u8; N]) -> Self {
        Self::from_array(&bytes)
    }

    pub fn unchecked_new(_env: Env, bytes: Vec<u8>) -> Self {
        BytesN(bytes)
    }

    pub fn set(&mut self, i: u32, v: u8) {
        if i < (N as u32) {
            self.0[i as usize] = v;
        } else {
            panic!("OOB")
        }
    }

    pub fn get(&self, i: u32) -> Option<u8> {
        if i < (N) as u32 {
            Some(self.0[i as usize])
        } else {
            None
        }
    }

    pub fn get_unchecked(&self, i: u32) -> u8 {
        self.0[i as usize]
    }

    pub fn is_empty(&self) -> bool {
        false
    }

    pub fn len(&self) -> u32 {
        N as u32
    }

    pub fn first(&self) -> Option<u8> {
        Some(self.0[0])
    }

    pub fn first_unchecked(&self) -> u8 {
        self.0[0]
    }

    pub fn last(&self) -> Option<u8> {
        if N >= 8 {
            Some(self.0[N - 1])
        } else {
            None
        }
    }

    pub fn last_unchecked(&self) -> u8 {
        self.0[N - 1]
    }

    pub fn iter(&self) -> soroban_env_common::vec::VecIterator<u8> {
        self.0.iter()
    }
}

impl From<BytesN<32>> for Bytes {
    fn from(item: BytesN<32>) -> Self {
        Bytes(Vec::new_from_slice(&item.0))
    }
}

#[cfg(any(kani, feature = "kani"))]
impl<const N: usize> kani::Arbitrary for BytesN<N> {
    fn any() -> Self {
        let mut v = Vec::new();
        for _ in 0..N {
            v.push(kani::any::<u8>());
        }
        BytesN(v)
    }
}

impl<E: internal::Env> IntoVal<E, BytesN<32>> for BytesN<32> {
    fn into_val(self, _env: &E) -> BytesN<32> {
        self
    }
}

impl<const N: usize> From<Box<crate::Val>> for BytesN<N> {
    fn from(value: Box<crate::Val>) -> Self {
        if let crate::Val::BytesNVal(u) = *value {
            if u.len() == N {
                BytesN(u)
            } else {
                panic!("Err")
            }
        } else {
            panic!("Err")
        }
    }
}

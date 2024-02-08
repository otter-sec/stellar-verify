use crate::Bytes;
#[cfg(any(kani, feature = "kani"))]
use soroban_env_common::ToValEnum;
use soroban_env_common::{ConversionError, Env};

#[cfg(any(kani, feature = "kani"))]
use soroban_env_common::FromValEnum;

pub trait ToXdr {
    fn to_xdr(self, _env: &Env) -> Bytes;
}

pub trait FromXdr: Sized {
    fn from_xdr(_env: &Env, _b: &Bytes) -> Result<Self, ConversionError>;
}
#[cfg(any(kani, feature = "kani"))]
impl<T> ToXdr for T
where
    T: ToValEnum,
{
    fn to_xdr(self, env: &Env) -> Bytes {
        // kani::any::<Bytes>()
        Bytes::new(env)
    }
}

#[cfg(any(kani, feature = "kani"))]
impl<T> FromXdr for T
where
    T: FromValEnum,
    T: kani::Arbitrary,
{
    fn from_xdr(_env: &Env, _b: &Bytes) -> Result<Self, ConversionError> {
        Ok(kani::any::<T>())
    }
}

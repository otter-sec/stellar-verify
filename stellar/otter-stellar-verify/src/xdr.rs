// use crate::{
//     env::internal::Env as _, unwrap::UnwrapInfallible, Bytes, Env, IntoVal, TryFromVal, Val,
// };
use crate::{bytes::Bytes, Env};
use soroban_env_common::{FromValEnum, ToValEnum};

pub use stellar_xdr::*;

pub trait ToXdr {
    fn to_xdr(self, _env: &Env) -> Bytes;
}

pub trait FromXdr: Sized {
    fn from_xdr(_env: &Env, _b: &Bytes) -> Result<Self, Error>;
}

impl<T> ToXdr for T
where
    T: ToValEnum,
{
    fn to_xdr(self, _env: &Env) -> Bytes {
        kani::any::<Bytes>()
    }
}

impl<T> FromXdr for T
where
    T: FromValEnum,
    T: kani::Arbitrary,
{
    fn from_xdr(_env: &Env, _b: &Bytes) -> Result<Self, Error> {
        Ok(kani::any::<T>())
    }
}

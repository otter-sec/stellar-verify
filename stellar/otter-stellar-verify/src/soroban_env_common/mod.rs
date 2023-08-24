use num_derive::FromPrimitive;

pub mod convert;
pub mod error;
pub mod num;
pub mod val;
pub mod wrapper_macros;

#[repr(u64)]
#[derive(Debug, FromPrimitive, PartialEq, Eq, Clone)]
pub enum StorageType {
    Temporary = 0,
    Persistent = 1,
    Instance = 2,
}

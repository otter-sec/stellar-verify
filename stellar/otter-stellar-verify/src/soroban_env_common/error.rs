use crate::Val;

#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct Error(Val);

impl Error {}

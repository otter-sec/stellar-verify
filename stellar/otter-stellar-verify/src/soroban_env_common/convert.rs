use core::fmt::Debug;

pub trait TryIntoVal<V> {
    type Error: Debug;
    fn try_into_val(&self) -> Result<V, Self::Error>;
}

pub trait TryFromVal<V: ?Sized>: Sized {
    type Error: Debug;
    fn try_from_val(v: &V) -> Result<Self, Self::Error>;
}

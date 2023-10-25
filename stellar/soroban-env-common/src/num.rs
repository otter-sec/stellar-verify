use crate::{FromValEnum, ToValEnum, Val};

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub struct Timepoint(u64);

impl ToValEnum for Timepoint {
    fn to_val(&self) -> Val {
        Val::TimepointVal(Timepoint(self.0))
    }
}

impl FromValEnum for Timepoint {
    fn from_val(val: Val) -> Option<Timepoint> {
        if let Val::TimepointVal(t) = val {
            Some(t)
        } else {
            None
        }
    }
}

impl From<u64> for Timepoint {
    fn from(i: u64) -> Self {
        Timepoint(i)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub struct Duration(u64);

impl ToValEnum for Duration {
    fn to_val(&self) -> Val {
        Val::DurationVal(Duration(self.0))
    }
}

impl FromValEnum for Duration {
    fn from_val(val: Val) -> Option<Duration> {
        if let Val::DurationVal(d) = val {
            Some(d)
        } else {
            None
        }
    }
}

impl ToValEnum for u32 {
    fn to_val(&self) -> Val {
        Val::U32Val(*self)
    }
}

impl FromValEnum for u32 {
    fn from_val(val: Val) -> Option<u32> {
        if let Val::U32Val(u) = val {
            Some(u)
        } else {
            None
        }
    }
}

impl ToValEnum for i32 {
    fn to_val(&self) -> Val {
        Val::I32Val(*self)
    }
}

impl FromValEnum for i32 {
    fn from_val(val: Val) -> Option<i32> {
        if let Val::I32Val(i) = val {
            Some(i)
        } else {
            None
        }
    }
}

impl ToValEnum for u64 {
    fn to_val(&self) -> Val {
        Val::U64Val(*self)
    }
}

impl FromValEnum for u64 {
    fn from_val(val: Val) -> Option<u64> {
        if let Val::U64Val(u) = val {
            Some(u)
        } else {
            None
        }
    }
}

impl ToValEnum for i64 {
    fn to_val(&self) -> Val {
        Val::I64Val(*self)
    }
}

impl FromValEnum for i64 {
    fn from_val(val: Val) -> Option<i64> {
        if let Val::I64Val(i) = val {
            Some(i)
        } else {
            None
        }
    }
}

impl ToValEnum for i128 {
    fn to_val(&self) -> Val {
        Val::I128(*self)
    }
}

impl FromValEnum for i128 {
    fn from_val(val: Val) -> Option<i128> {
        if let Val::I128(i) = val {
            Some(i)
        } else {
            None
        }
    }
}

impl ToValEnum for u128 {
    fn to_val(&self) -> Val {
        Val::U128(*self)
    }
}

impl FromValEnum for u128 {
    fn from_val(val: Val) -> Option<u128> {
        if let Val::U128(i) = val {
            Some(i)
        } else {
            None
        }
    }
}

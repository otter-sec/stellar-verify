use crate::{
    enums::EnumType,
    num::{Duration, Timepoint},
    symbol::Symbol,
};

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum Val {
    SymbolVal(Symbol),
    I32Val(i32),
    U32Val(u32),
    I64Val(i64),
    U64Val(u64),
    USizeVal(usize),
    String(crate::String),
    TimepointVal(Timepoint),
    DurationVal(Duration),
    BoolVal(bool),
    AddressObj(u32),
    I128(i128),
    U128(u128),
    #[default]
    Void,
    Struct(Vec<u8>),
    VecVal(crate::Vec<Box<Val>>),
    EnumVal(EnumType),
    BytesNVal(crate::Vec<u8>),
    MapVal(Vec<(Val, Val)>),
}

impl Val {
    // To methods
    pub fn to_i32(&self) -> Option<i32> {
        if let Val::I32Val(i) = self {
            Some(*i)
        } else {
            None
        }
    }

    pub fn to_u32(&self) -> Option<u32> {
        if let Val::U32Val(i) = self {
            Some(*i)
        } else {
            None
        }
    }

    pub fn to_i64(&self) -> Option<i64> {
        if let Val::I64Val(i) = self {
            Some(*i)
        } else {
            None
        }
    }

    pub fn to_u64(&self) -> Option<u64> {
        if let Val::U64Val(i) = self {
            Some(*i)
        } else {
            None
        }
    }

    pub fn to_usize(&self) -> Option<usize> {
        if let Val::USizeVal(i) = self {
            Some(*i)
        } else {
            None
        }
    }

    pub fn to_bool(&self) -> Option<bool> {
        if let Val::BoolVal(b) = self {
            Some(*b)
        } else {
            None
        }
    }

    pub fn to_string(&self) -> Option<std::string::String> {
        if let Val::String(s) = self {
            Some(s.to_string())
        } else if let Val::SymbolVal(s) = self {
            Some(s.to_string())
        } else {
            None
        }
    }
}

// Define the ToVal trait
pub trait ToValEnum {
    fn to_val(&self) -> Val;
}

// Define the FromVal trait
pub trait FromValEnum: Sized {
    fn from_val(val: Val) -> Option<Self>;
}

impl ToValEnum for bool {
    fn to_val(&self) -> Val {
        Val::BoolVal(*self)
    }
}

impl FromValEnum for bool {
    fn from_val(val: Val) -> Option<bool> {
        if let Val::BoolVal(b) = val {
            Some(b)
        } else {
            None
        }
    }
}

impl From<usize> for Val {
    fn from(i: usize) -> Self {
        Val::USizeVal(i)
    }
}

impl From<&str> for Val {
    fn from(s: &str) -> Self {
        let custom_string = crate::String::from(s);
        Val::String(custom_string)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct BytesObject(crate::Val);

#[derive(Debug, Eq, PartialEq)]
pub struct ConversionError;

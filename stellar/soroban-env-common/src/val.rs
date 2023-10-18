use quote::{quote, ToTokens};
const SCSYMBOL_LIMIT: usize = 10;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ValEnum {
    Symbol(Symbol),
    I32(i32),
    U32(u32),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Copy, Default)]
pub struct Symbol([u8; SCSYMBOL_LIMIT]);

impl Symbol {
    pub fn new(symbol: &str) -> Self {
        let mut symbol_str = Self::default();
        let symbol_bytes = symbol.as_bytes();

        for (i, &c) in symbol_bytes.iter().enumerate().take(SCSYMBOL_LIMIT) {
            symbol_str.0[i] = c;
        }

        symbol_str
    }
}

// Define the ToVal trait
pub trait ToValEnum {
    fn to_val(&self) -> ValEnum;
}

// Define the FromVal trait
pub trait FromValEnum: Sized {
    fn from_val(val: ValEnum) -> Option<Self>;
}

impl ToValEnum for u32 {
    fn to_val(&self) -> ValEnum {
        ValEnum::U32(*self)
    }
}

impl FromValEnum for u32 {
    fn from_val(val: ValEnum) -> Option<u32> {
        if let ValEnum::U32(u) = val {
            Some(u)
        } else {
            None
        }
    }
}

impl ToValEnum for i32 {
    fn to_val(&self) -> ValEnum {
        ValEnum::I32(*self)
    }
}

impl FromValEnum for i32 {
    fn from_val(val: ValEnum) -> Option<i32> {
        if let ValEnum::I32(i) = val {
            Some(i)
        } else {
            None
        }
    }
}

impl ToValEnum for Symbol {
    fn to_val(&self) -> ValEnum {
        ValEnum::Symbol(*self)
    }
}

impl FromValEnum for Symbol {
    fn from_val(val: ValEnum) -> Option<Symbol> {
        if let ValEnum::Symbol(symbol) = val {
            Some(symbol)
        } else {
            None
        }
    }
}

// Implement ToTokens for your custom enum
impl ToTokens for Symbol {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let mut value = Vec::new();
        for i in 0..SCSYMBOL_LIMIT {
            value.push(self.0[i]);
        }
        tokens.extend(quote! {
            Symbol{
               vec![#(#value),*]
            }
        });
    }
}

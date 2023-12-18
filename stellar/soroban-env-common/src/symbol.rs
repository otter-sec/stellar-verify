use core::fmt;
use std::fmt::Display;

use crate::{Env, FromValEnum, ToValEnum, Val};

const SCSYMBOL_LIMIT: usize = 10;

#[derive(Debug)]
pub enum SymbolError {
    /// Returned when attempting to form a [Symbol] from a string with more
    /// than 10 characters.
    TooLong(usize),
    /// Returned when attempting to form a [Symbol] from
    /// a string with characters outside the range `[a-zA-Z0-9_]`.
    BadChar(char),
}

impl Display for SymbolError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SymbolError::TooLong(n) => write!(f, "SymbolError::TooLong({})", n),
            SymbolError::BadChar(ch) => write!(f, "SymbolError::BadChar({})", ch),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Copy, Default)]
pub struct Symbol(pub [u8; SCSYMBOL_LIMIT]);

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = self.as_str();
        write!(f, "{}", s)
    }
}

impl Symbol {
    pub fn as_str(&self) -> &str {
        // Convert the internal byte array to a string slice
        // Find the first null byte (0x00) in the byte array, if any.
        let null_byte_index = self.0.iter().position(|&byte| byte == 0);

        match null_byte_index {
            Some(index) => {
                // If a null byte is found, convert the slice up to that point to a string.
                std::str::from_utf8(&self.0[..index]).unwrap_or_default()
            }
            None => {
                // If no null byte is found, convert the entire array to a string.
                std::str::from_utf8(&self.0).unwrap_or_default()
            }
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    pub fn to_le_bytes(&self) -> [u8; SCSYMBOL_LIMIT] {
        self.0
    }

    pub fn from_le_bytes(bytes: [u8; SCSYMBOL_LIMIT]) -> Self {
        Symbol(bytes)
    }

    pub const fn from(s: &str) -> Self {
        Self::new_from_str(s)
    }

    pub const fn new(_env: &Env, s: &str) -> Self {
        Self::new_from_str(s)
    }

    pub const fn new_from_str(symbol: &str) -> Self {
        let mut n = 0;
        let sym_bytes = symbol.as_bytes();
        let mut bytes = [b'\x00'; SCSYMBOL_LIMIT];
        // Limit the number of bytes copied to the length of the symbol or 10, whichever is less
        let limit = if sym_bytes.len() > SCSYMBOL_LIMIT {
            SCSYMBOL_LIMIT
        } else {
            sym_bytes.len()
        };
        // Copy from sym_bytes to bytes
        while n < limit {
            bytes[n] = sym_bytes[n];
            n += 1;
        }
        Symbol(bytes)
    }

    pub const fn try_from_bytes(b: &[u8]) -> Result<Self, SymbolError> {
        let mut n = 0;
        let mut _accum: u64 = 0;
        let mut bytes = [0; 10];
        while n < b.len() {
            let ch = b[n] as char;
            if n >= SCSYMBOL_LIMIT {
                return Err(SymbolError::TooLong(b.len()));
            }
            let v = match Self::encode_char(ch) {
                Ok(v) => v,
                Err(e) => return Err(e),
            };
            _accum |= v;
            bytes[n] = v as u8;
            n += 1;
        }
        Ok(Symbol(bytes))
    }

    const fn encode_char(ch: char) -> Result<u64, SymbolError> {
        let v = match ch {
            '_' => 1,
            '0'..='9' => 2 + ((ch as u64) - ('0' as u64)),
            'A'..='Z' => 12 + ((ch as u64) - ('A' as u64)),
            'a'..='z' => 38 + ((ch as u64) - ('a' as u64)),
            _ => return Err(SymbolError::BadChar(ch)),
        };
        Ok(v)
    }
}

impl ToValEnum for Symbol {
    fn to_val(&self) -> Val {
        Val::SymbolVal(*self)
    }
}

#[cfg(any(kani, feature = "kani"))]
impl kani::Arbitrary for Symbol {
    fn any() -> Self {
        let maybe_symbol = Symbol::try_from_bytes(&kani::any::<[u8; 10]>());
        kani::assume(maybe_symbol.is_ok());
        maybe_symbol.unwrap()
    }
}

impl FromValEnum for Symbol {
    fn from_val(val: Val) -> Option<Symbol> {
        if let Val::SymbolVal(symbol) = val {
            Some(symbol)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{FromValEnum, ToValEnum};

    #[test]
    fn test_symbol() {
        let s = "hello";
        let sym = crate::Symbol::new_from_str(s);
        let val = sym.to_val();
        let sym2 = crate::Symbol::from_val(val).unwrap();
        assert_eq!(sym, sym2);
        let as_str = sym2.as_str();
        assert_eq!(as_str, s);
    }

    #[test]
    fn test_symbol_to_le_bytes() {
        let s = "hello";
        let sym = crate::Symbol::new_from_str(s);
        let bytes = sym.to_le_bytes();
        let sym2 = crate::Symbol::from_le_bytes(bytes);
        assert_eq!(sym, sym2);
        let as_str = sym2.as_str();
        assert_eq!(as_str, s);
    }
}

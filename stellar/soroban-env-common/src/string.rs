use std::fmt;

use crate::{FromValEnum, ToValEnum, Val};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Copy, Default)]
pub struct String(pub [u8; 10]);

impl fmt::Display for String {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = self.as_str();
        write!(f, "{}", s)
    }
}

impl String {
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

    pub fn to_le_bytes(&self) -> [u8; core::mem::size_of::<Self>()] {
        self.0
    }

    pub fn from_le_bytes(bytes: [u8; std::mem::size_of::<Self>()]) -> Self {
        String(bytes)
    }
}

impl From<&str> for String {
    fn from(s: &str) -> Self {
        let mut bytes = [0; 10];

        for (i, byte) in s.bytes().enumerate() {
            if i >= 10 {
                // Panic if the string is longer than the array size
                // panic!("String is too long");
                break;
            }
            bytes[i] = byte;
        }

        String(bytes)
    }
}

impl ToValEnum for crate::String {
    fn to_val(&self) -> Val {
        Val::String(*self)
    }
}

impl FromValEnum for crate::String {
    fn from_val(val: Val) -> Option<crate::String> {
        if let Val::String(str) = val {
            Some(str)
        } else {
            None
        }
    }
}

#[cfg(feature = "kani")]
impl kani::Arbitrary for crate::String {
    fn any() -> Self {
        crate::String(kani::any::<[u8; 10]>())
    }
}

#[cfg(test)]
mod test {
    use crate::{FromValEnum, ToValEnum};

    #[test]
    fn test_string() {
        let s = "hello";
        let s = crate::String::from(s);
        let val = s.to_val();
        let s2 = crate::String::from_val(val).unwrap();
        assert_eq!(s, s2);
        let as_str = s2.as_str();
        assert_eq!(as_str, "hello");
    }
}

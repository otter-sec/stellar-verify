use crate::declare_tag_based_wrapper;

use super::val::{Tag, Val};

declare_tag_based_wrapper!(U32Val);
declare_tag_based_wrapper!(I32Val);

impl Val {
    #[inline(always)]
    pub const fn from_u32(u: u32) -> U32Val {
        unsafe { U32Val(Val::from_major_minor_and_tag(u, 0, Tag::U32Val)) }
    }

    #[inline(always)]
    pub const fn from_i32(i: i32) -> I32Val {
        unsafe { I32Val(Val::from_major_minor_and_tag(i as u32, 0, Tag::I32Val)) }
    }

    #[inline(always)]
    pub const fn is_i32_zero(self) -> bool {
        self.shallow_eq(&Self::I32_ZERO.0)
    }

    #[inline(always)]
    pub const fn is_u32_zero(self) -> bool {
        self.shallow_eq(&Self::U32_ZERO.0)
    }
}

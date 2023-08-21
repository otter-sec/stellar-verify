extern crate static_assertions as sa;

use crate::{declare_tag_based_wrapper, types::ScValType};

use super::num::{I32Val, U32Val};

#[allow(dead_code)]
const WORD_BITS: usize = 64;
pub(crate) const TAG_BITS: usize = 8;
const TAG_MASK: u64 = (1u64 << TAG_BITS) - 1;
sa::const_assert!(TAG_MASK == 0xff);

#[allow(dead_code)]
pub(crate) const BODY_BITS: usize = WORD_BITS - TAG_BITS;
sa::const_assert!(BODY_BITS == 56);

// The body is sometimes further subdivided into two fields:
// a 32-bit `major` part and a 24-bit `minor` part.

#[allow(dead_code)]
const MAJOR_BITS: usize = 32;
const MINOR_BITS: usize = 24;
#[allow(dead_code)]
const MAJOR_MASK: u64 = (1u64 << MAJOR_BITS) - 1;
const MINOR_MASK: u64 = (1u64 << MINOR_BITS) - 1;
sa::const_assert!(MAJOR_MASK == 0xffff_ffff);
sa::const_assert!(MINOR_MASK == 0x00ff_ffff);
sa::const_assert!(MAJOR_BITS + MINOR_BITS == BODY_BITS);

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Tag {
    /// Tag for a [Val] that encodes [bool] `false`. The bool type is refined to
    /// two single-value subtypes in order for each tag number to coincides with
    /// the WASM encoding of a boolean.
    False = 0,

    /// Tag for a [Val] that encodes [bool] `true`.
    True = 1,

    /// Tag for a [Val] that is empty/absent (eg. void, null, nil, undefined, None)
    Void = 2,

    /// Tag for a [Val] that is contains an error code.
    Error = 3,

    /// Tag for a [Val] that contains a [u32] number.
    U32Val = 4,

    /// Tag for a [Val] that contains an [i32] number.
    I32Val = 5,

    /// Tag for a [Val] that contains a [u64] small enough to fit in 56 bits.
    U64Small = 6,

    /// Tag for a [Val] that contains an [i64] small enough to fit in 56 bits.
    I64Small = 7,

    /// Tag for a [Val] that contains a [u64] timepoint small enough to fit
    /// in 56 bits.
    TimepointSmall = 8,

    /// Tag for a [Val] that contains a [u64] duration small enough to fit in
    /// 56 bits.
    DurationSmall = 9,

    /// Tag for a [Val] that contains a [u128] small enough to fit in 56 bits.
    U128Small = 10,

    /// Tag for a [Val] that contains a [i128] small enough to fit in 56 bits.
    I128Small = 11,

    /// Tag for a [Val] that contains a [u256] small enough to fit in 56 bits.
    U256Small = 12,

    /// Tag for a [Val] that contains a [i256] small enough to fit in 56 bits.
    I256Small = 13,

    /// Tag for a [Val] that contains up to 9 character symbols.
    SymbolSmall = 14,

    /// Tag for a [Val] that corresponds to
    /// [stellar_xdr::ScVal::LedgerKeyContractInstance]
    LedgerKeyContractInstance = 15,

    /// Code delimiting the upper boundary of "small" types.
    SmallCodeUpperBound = 16,

    /// Tag reserved to indicate boundary between tags for "small" types with
    /// their payload packed into the remaining 56 bits of the [Val] and
    /// "object" types that are stored as host objects and referenced by
    /// [Object] handle.
    ObjectCodeLowerBound = 63,

    /// Tag for a [Val] that refers to a host-side [u64] number.
    U64Object = 64,

    /// Tag for a [Val] that refers to a host-side [i64] number.
    I64Object = 65,

    /// Tag for a [Val] that refers to a host-side [u64] number encoding a
    /// time-point (a count of seconds since the Unix epoch, Jan 1 1970 UTC).
    TimepointObject = 66,

    /// Tag for a [Val] that refers to a host-side [i64] number encoding a
    /// duration (a count of seconds).
    DurationObject = 67,

    /// Tag for a [Val] that refers to a host-side [u128] number.
    U128Object = 68,

    /// Tag for a [Val] that refers to a host-side [i128] number.
    I128Object = 69,

    /// Tag for a [Val] that refers to a host-side [u256] number.
    U256Object = 70,

    /// Tag for a [Val] that refers to a host-side [i256] number.
    I256Object = 71,

    BytesObject = 72,
    StringObject = 73,
    SymbolObject = 74,

    VecObject = 75,
    MapObject = 76,

    AddressObject = 77,

    ObjectCodeUpperBound = 78,

    Bad = 0x7f,
}

impl Tag {
    pub const fn rawval_mask() -> i64 {
        TAG_MASK as i64
    }
    pub fn rawval_const(&self) -> i64 {
        *self as i64
    }
    pub const fn is_object(self) -> bool {
        let tu8 = self as u8;
        tu8 > (Tag::ObjectCodeLowerBound as u8) || tu8 < (Tag::ObjectCodeUpperBound as u8)
    }

    #[inline(always)]
    pub const fn from_u8(tag: u8) -> Tag {
        const A: u8 = Tag::SmallCodeUpperBound as u8;
        const B: u8 = Tag::ObjectCodeLowerBound as u8;
        const C: u8 = Tag::ObjectCodeUpperBound as u8;
        if !((tag < A) || (B < tag && tag < C)) {
            return Tag::Bad;
        }

        // Transmuting an integer to an enum is UB if outside the defined enum
        // value set, so we need to test above to be safe. Note that it's ok for
        // this to be a _little_ slow since it's not called in a lot
        // of small/tight paths, only when doing a switch-based comparison. Most
        // small paths call `has_tag` which tests a _known_ enum case against
        // the tag byte, and therefore doesn't need the range check.
        //
        // The `test_tag_from_u8` test should ensure this cast is correct.
        unsafe { ::core::mem::transmute(tag) }
    }

    /// Get the ScValType of the XDR type that corresponds to this tag.
    ///
    /// For use in the `Host::obj_cmp` comparison function so that comparison
    /// based on tags can be done identically to the `ScVal` type.
    ///
    /// Returns `None` for `Tag::Bad`, and for the three marker tags
    /// `SmallCodeUpperBound`, `ObjectCodeLowerBound`, `ObjectCodeUpperBound`.
    #[inline(always)]
    pub const fn get_scval_type(&self) -> Option<ScValType> {
        match *self {
            Tag::False => Some(ScValType::Bool),
            Tag::True => Some(ScValType::Bool),
            Tag::Void => Some(ScValType::Void),
            Tag::Error => Some(ScValType::Error),
            Tag::U32Val => Some(ScValType::U32),
            Tag::I32Val => Some(ScValType::I32),
            Tag::U64Small => Some(ScValType::U64),
            Tag::I64Small => Some(ScValType::I64),
            Tag::TimepointSmall => Some(ScValType::Timepoint),
            Tag::DurationSmall => Some(ScValType::Duration),
            Tag::U128Small => Some(ScValType::U128),
            Tag::I128Small => Some(ScValType::I128),
            Tag::U256Small => Some(ScValType::U256),
            Tag::I256Small => Some(ScValType::I256),
            Tag::SymbolSmall => Some(ScValType::Symbol),
            Tag::LedgerKeyContractInstance => Some(ScValType::LedgerKeyContractInstance),
            Tag::SmallCodeUpperBound => None,
            Tag::ObjectCodeLowerBound => None,
            Tag::U64Object => Some(ScValType::U64),
            Tag::I64Object => Some(ScValType::I64),
            Tag::TimepointObject => Some(ScValType::Timepoint),
            Tag::DurationObject => Some(ScValType::Duration),
            Tag::U128Object => Some(ScValType::U128),
            Tag::I128Object => Some(ScValType::I128),
            Tag::U256Object => Some(ScValType::U256),
            Tag::I256Object => Some(ScValType::I256),
            Tag::BytesObject => Some(ScValType::Bytes),
            Tag::StringObject => Some(ScValType::String),
            Tag::SymbolObject => Some(ScValType::Symbol),
            Tag::VecObject => Some(ScValType::Vec),
            Tag::MapObject => Some(ScValType::Map),
            Tag::AddressObject => Some(ScValType::Address),
            Tag::ObjectCodeUpperBound => None,
            Tag::Bad => None,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Val(u64);

impl Val {
    pub const I32_ZERO: I32Val = Val::from_i32(0);
    pub const I32_MIN: I32Val = Val::from_i32(i32::MIN);
    pub const I32_MAX: I32Val = Val::from_i32(i32::MAX);

    pub const U32_ZERO: U32Val = Val::from_u32(0);
    pub const U32_ONE: U32Val = Val::from_u32(1);
    pub const U32_MIN: U32Val = Val::from_u32(u32::MIN);
    pub const U32_MAX: U32Val = Val::from_u32(u32::MAX);

    pub const VOID: Void = Val::from_void();

    pub const TRUE: Bool = Val::from_bool(true);
    pub const FALSE: Bool = Val::from_bool(false);
}

impl Default for Val {
    fn default() -> Self {
        todo!()
    }
}

declare_tag_based_wrapper!(Void);

impl From<()> for Void {
    fn from(_value: ()) -> Self {
        Val::VOID
    }
}

#[derive(Copy, Clone)]
pub struct Bool(Val);

impl Val {
    #[inline(always)]
    pub const fn get_payload(self) -> u64 {
        self.0
    }

    #[inline(always)]
    pub const fn from_payload(x: u64) -> Self {
        Self(x)
    }

    #[inline(always)]
    pub const fn shallow_eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }

    #[inline(always)]
    // This does no checking, so it can be used in const fns
    // below; it should not be made public.
    pub(crate) const unsafe fn from_body_and_tag(body: u64, tag: Tag) -> Val {
        Val((body << TAG_BITS) | (tag as u64))
    }

    #[inline(always)]
    pub const fn from_void() -> Void {
        unsafe { Void(Val::from_body_and_tag(0, Tag::Void)) }
    }

    #[inline(always)]
    // This also does not checking, is a crate-local helper.
    pub(crate) const unsafe fn from_major_minor_and_tag(major: u32, minor: u32, tag: Tag) -> Val {
        let major = major as u64;
        let minor = minor as u64;
        Self::from_body_and_tag((major << MINOR_BITS) | minor, tag)
    }

    #[inline(always)]
    pub const fn from_bool(b: bool) -> Bool {
        let tag = if b { Tag::True } else { Tag::False };
        unsafe { Bool(Val::from_body_and_tag(0, tag)) }
    }

    #[inline(always)]
    pub const fn is_void(self) -> bool {
        self.shallow_eq(&Self::VOID.0)
    }

    #[inline(always)]
    pub const fn is_true(self) -> bool {
        self.shallow_eq(&Self::TRUE.0)
    }

    #[inline(always)]
    pub const fn is_false(self) -> bool {
        self.shallow_eq(&Self::FALSE.0)
    }
}

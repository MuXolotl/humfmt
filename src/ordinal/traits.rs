use crate::common::sealed::Sealed;

/// Internal ordinal input representation.
///
/// Created via the [`OrdinalLike`] trait implementations.
#[derive(Copy, Clone, Debug)]
pub enum OrdinalValue {
    /// Signed integer input.
    Int(i128),
    /// Unsigned integer input.
    UInt(u128),
}

/// Trait for inputs accepted by [`crate::ordinal()`].
///
/// Implemented for all integer primitives (`i8`..`i128`, `u8`..`u128`, `isize`, `usize`).
/// This trait is sealed and cannot be implemented outside this crate.
pub trait OrdinalLike: Sealed + Copy {
    /// Converts the input value into the internal ordinal representation.
    fn into_ordinal(self) -> OrdinalValue;
}

macro_rules! impl_signed {
    ($($t:ty),* $(,)?) => {
        $(
            impl OrdinalLike for $t {
                #[inline]
                fn into_ordinal(self) -> OrdinalValue {
                    OrdinalValue::Int(self as i128)
                }
            }
        )*
    };
}

macro_rules! impl_unsigned {
    ($($t:ty),* $(,)?) => {
        $(
            impl OrdinalLike for $t {
                #[inline]
                fn into_ordinal(self) -> OrdinalValue {
                    OrdinalValue::UInt(self as u128)
                }
            }
        )*
    };
}

impl_signed!(i8, i16, i32, i64, i128, isize);
impl_unsigned!(u8, u16, u32, u64, u128, usize);

use crate::common::{numeric::NumericValue, sealed::Sealed};

pub trait NumberLike: Sealed + Copy {
    fn into_numeric(self) -> NumericValue;
}

macro_rules! impl_signed {
    ($($t:ty),* $(,)?) => {
        $(
            impl NumberLike for $t {
                fn into_numeric(self) -> NumericValue {
                    NumericValue::Int(self as i128)
                }
            }
        )*
    };
}

macro_rules! impl_unsigned {
    ($($t:ty),* $(,)?) => {
        $(
            impl NumberLike for $t {
                fn into_numeric(self) -> NumericValue {
                    NumericValue::UInt(self as u128)
                }
            }
        )*
    };
}

macro_rules! impl_float {
    ($($t:ty),* $(,)?) => {
        $(
            impl NumberLike for $t {
                fn into_numeric(self) -> NumericValue {
                    NumericValue::Float(self as f64)
                }
            }
        )*
    };
}

impl_signed!(i8, i16, i32, i64, i128, isize);
impl_unsigned!(u8, u16, u32, u64, u128, usize);
impl_float!(f32, f64);

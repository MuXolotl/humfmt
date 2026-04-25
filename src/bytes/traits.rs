use crate::common::sealed::Sealed;

#[derive(Copy, Clone, Debug)]
pub enum BytesValue {
    Int(i128),
    UInt(u128),
}

pub trait BytesLike: Sealed + Copy {
    fn into_bytes(self) -> BytesValue;
}

macro_rules! impl_signed {
    ($($t:ty),*) => {
        $(
            impl BytesLike for $t {
                fn into_bytes(self) -> BytesValue {
                    BytesValue::Int(self as i128)
                }
            }
        )*
    };
}

macro_rules! impl_unsigned {
    ($($t:ty),*) => {
        $(
            impl BytesLike for $t {
                fn into_bytes(self) -> BytesValue {
                    BytesValue::UInt(self as u128)
                }
            }
        )*
    };
}

impl_signed!(i8, i16, i32, i64, i128, isize);
impl_unsigned!(u8, u16, u32, u64, u128, usize);

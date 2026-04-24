use crate::common::{numeric::NumericValue, sealed::Sealed};

pub trait NumberLike: Sealed + Copy {
    fn into_numeric(self) -> NumericValue;
}
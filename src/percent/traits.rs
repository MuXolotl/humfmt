use crate::common::sealed::Sealed;

/// Trait for inputs accepted by [`crate::percent`] / [`crate::percent_with`].
///
/// Implemented for `f32` and `f64`.
/// This trait is sealed and cannot be implemented outside this crate.
pub trait PercentLike: Sealed + Copy {
    /// Converts the input value into an `f64` for percentage formatting.
    fn into_percent(self) -> f64;
}

impl PercentLike for f32 {
    #[inline]
    fn into_percent(self) -> f64 {
        self as f64
    }
}

impl PercentLike for f64 {
    #[inline]
    fn into_percent(self) -> f64 {
        self
    }
}

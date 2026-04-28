use crate::common::sealed::Sealed;

/// Trait for inputs accepted by [`crate::duration`] / [`crate::duration_with`]
/// and [`crate::ago`] / [`crate::ago_with`].
///
/// Currently implemented for `core::time::Duration`.
/// This trait is sealed and cannot be implemented outside this crate.
pub trait DurationLike: Sealed + Copy {
    /// Converts the input value into a standard library duration.
    fn into_duration(self) -> core::time::Duration;
}

impl DurationLike for core::time::Duration {
    fn into_duration(self) -> core::time::Duration {
        self
    }
}

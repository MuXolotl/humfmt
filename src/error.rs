use core::fmt;

/// Error returned by the duration adapters in `humfmt::chrono` and `humfmt::time`.
///
/// `core::time::Duration` is always non-negative and has a limited range, so
/// converting an ecosystem duration into one can fail in two ways. Both are
/// reported by this single type.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DurationConversionError {
    /// The provided duration is negative.
    NegativeDuration,
    /// The provided duration is out of the supported range.
    OutOfRange,
}

impl fmt::Display for DurationConversionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NegativeDuration => {
                f.write_str("negative durations are not supported by this formatter")
            }
            Self::OutOfRange => f.write_str("duration value is out of supported range"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for DurationConversionError {}

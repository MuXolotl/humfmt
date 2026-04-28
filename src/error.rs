use core::fmt;

/// Error returned when a duration-like value is negative.
///
/// This is primarily used by optional ecosystem adapters (`chrono` / `time`)
/// when converting into `core::time::Duration` (which is always non-negative).
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct NegativeDurationError;

/// Conversion error for duration adapters.
///
/// This error provides more explicit semantics than [`NegativeDurationError`]
/// by distinguishing between:
///
/// - negative inputs
/// - values that cannot be represented as `core::time::Duration`
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DurationConversionError {
    /// The provided duration is negative.
    NegativeDuration,
    /// The provided duration is out of the supported range.
    OutOfRange,
}

impl fmt::Display for NegativeDurationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("negative durations are not supported by this formatter")
    }
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

impl From<NegativeDurationError> for DurationConversionError {
    fn from(_: NegativeDurationError) -> Self {
        Self::NegativeDuration
    }
}

#[cfg(feature = "std")]
impl std::error::Error for NegativeDurationError {}

#[cfg(feature = "std")]
impl std::error::Error for DurationConversionError {}

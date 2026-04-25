use core::fmt;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct NegativeDurationError;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DurationConversionError {
    NegativeDuration,
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

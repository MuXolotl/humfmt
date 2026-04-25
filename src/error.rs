use core::fmt;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct NegativeDurationError;

impl fmt::Display for NegativeDurationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("negative durations are not supported by this formatter")
    }
}

#[cfg(feature = "std")]
impl std::error::Error for NegativeDurationError {}

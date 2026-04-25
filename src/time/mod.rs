//! Optional integration with [`time`](https://docs.rs/time).
//!
//! This module adapts `time::Duration` and `time::OffsetDateTime` values into
//! `humfmt` duration and relative-time formatters while preserving the crate's
//! locale-aware options.
//!
//! # Examples
//!
//! ```rust
//! use humfmt::{time as humtime, DurationOptions};
//!
//! let delta = time::Duration::seconds(90);
//! assert_eq!(humtime::duration(delta).unwrap().to_string(), "1m 30s");
//!
//! let then = time::OffsetDateTime::from_unix_timestamp(0).unwrap();
//! let now = time::OffsetDateTime::from_unix_timestamp(3665).unwrap();
//! let out = humtime::ago_since_with(
//!     then,
//!     now,
//!     DurationOptions::new().long_units().max_units(3),
//! )
//! .unwrap();
//! assert_eq!(out.to_string(), "1 hour 1 minute 5 seconds ago");
//! ```

use crate::{
    ago::AgoDisplay, duration::DurationDisplay, locale::Locale, DurationOptions,
    NegativeDurationError,
};

/// Extension methods for `time::Duration`.
pub trait TimeHumanize: Sized {
    fn try_human_duration(self) -> Result<DurationDisplay, NegativeDurationError>;
    fn try_human_duration_with<L: Locale>(
        self,
        options: DurationOptions<L>,
    ) -> Result<DurationDisplay<L>, NegativeDurationError>;
    fn try_human_ago(self) -> Result<AgoDisplay, NegativeDurationError>;
    fn try_human_ago_with<L: Locale>(
        self,
        options: DurationOptions<L>,
    ) -> Result<AgoDisplay<L>, NegativeDurationError>;
}

impl TimeHumanize for ::time::Duration {
    fn try_human_duration(self) -> Result<DurationDisplay, NegativeDurationError> {
        duration(self)
    }

    fn try_human_duration_with<L: Locale>(
        self,
        options: DurationOptions<L>,
    ) -> Result<DurationDisplay<L>, NegativeDurationError> {
        duration_with(self, options)
    }

    fn try_human_ago(self) -> Result<AgoDisplay, NegativeDurationError> {
        ago(self)
    }

    fn try_human_ago_with<L: Locale>(
        self,
        options: DurationOptions<L>,
    ) -> Result<AgoDisplay<L>, NegativeDurationError> {
        ago_with(self, options)
    }
}

/// Formats a non-negative `time::Duration` with default duration options.
pub fn duration(value: ::time::Duration) -> Result<DurationDisplay, NegativeDurationError> {
    duration_with(value, DurationOptions::new())
}

/// Formats a non-negative `time::Duration` with custom duration options.
pub fn duration_with<L: Locale>(
    value: ::time::Duration,
    options: DurationOptions<L>,
) -> Result<DurationDisplay<L>, NegativeDurationError> {
    Ok(crate::duration::duration_with(to_std(value)?, options))
}

/// Formats a non-negative `time::Duration` as relative time using default options.
pub fn ago(value: ::time::Duration) -> Result<AgoDisplay, NegativeDurationError> {
    ago_with(value, DurationOptions::new())
}

/// Formats a non-negative `time::Duration` as relative time with custom options.
pub fn ago_with<L: Locale>(
    value: ::time::Duration,
    options: DurationOptions<L>,
) -> Result<AgoDisplay<L>, NegativeDurationError> {
    Ok(crate::ago::ago_with(to_std(value)?, options))
}

/// Formats the elapsed time between two `time::OffsetDateTime` values as relative time.
pub fn ago_since(
    then: ::time::OffsetDateTime,
    now: ::time::OffsetDateTime,
) -> Result<AgoDisplay, NegativeDurationError> {
    ago(now - then)
}

/// Formats the elapsed time between two `time::OffsetDateTime` values as
/// relative time using custom duration options.
pub fn ago_since_with<L: Locale>(
    then: ::time::OffsetDateTime,
    now: ::time::OffsetDateTime,
    options: DurationOptions<L>,
) -> Result<AgoDisplay<L>, NegativeDurationError> {
    ago_with(now - then, options)
}

fn to_std(value: ::time::Duration) -> Result<core::time::Duration, NegativeDurationError> {
    if value.is_negative() {
        return Err(NegativeDurationError);
    }

    Ok(value.unsigned_abs())
}

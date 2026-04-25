//! Optional integration with [`chrono`](https://docs.rs/chrono).
//!
//! This module adapts `chrono::TimeDelta` and `chrono::DateTime` values into
//! `humfmt` duration and relative-time formatters while preserving the crate's
//! locale-aware options.
//!
//! # Examples
//!
//! ```rust
//! use humfmt::{chrono as humchrono, DurationOptions};
//!
//! let delta = chrono::TimeDelta::try_seconds(90).unwrap();
//! assert_eq!(humchrono::duration(delta).unwrap().to_string(), "1m 30s");
//!
//! let then = chrono::DateTime::from_timestamp(0, 0).unwrap();
//! let now = chrono::DateTime::from_timestamp(3665, 0).unwrap();
//! let out = humchrono::ago_since_with(
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

/// Extension methods for `chrono::TimeDelta`.
pub trait ChronoHumanize: Sized {
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

impl ChronoHumanize for ::chrono::TimeDelta {
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

/// Formats a non-negative `chrono::TimeDelta` with default duration options.
pub fn duration(value: ::chrono::TimeDelta) -> Result<DurationDisplay, NegativeDurationError> {
    duration_with(value, DurationOptions::new())
}

/// Formats a non-negative `chrono::TimeDelta` with custom duration options.
pub fn duration_with<L: Locale>(
    value: ::chrono::TimeDelta,
    options: DurationOptions<L>,
) -> Result<DurationDisplay<L>, NegativeDurationError> {
    Ok(crate::duration::duration_with(to_std(value)?, options))
}

/// Formats a non-negative `chrono::TimeDelta` as relative time using default options.
pub fn ago(value: ::chrono::TimeDelta) -> Result<AgoDisplay, NegativeDurationError> {
    ago_with(value, DurationOptions::new())
}

/// Formats a non-negative `chrono::TimeDelta` as relative time with custom options.
pub fn ago_with<L: Locale>(
    value: ::chrono::TimeDelta,
    options: DurationOptions<L>,
) -> Result<AgoDisplay<L>, NegativeDurationError> {
    Ok(crate::ago::ago_with(to_std(value)?, options))
}

/// Formats the elapsed time between two `chrono` datetimes as relative time.
pub fn ago_since<Tz1: ::chrono::TimeZone, Tz2: ::chrono::TimeZone>(
    then: ::chrono::DateTime<Tz1>,
    now: ::chrono::DateTime<Tz2>,
) -> Result<AgoDisplay, NegativeDurationError> {
    ago(now.signed_duration_since(then))
}

/// Formats the elapsed time between two chrono datetimes as relative time
/// using custom duration options.
pub fn ago_since_with<Tz1: ::chrono::TimeZone, Tz2: ::chrono::TimeZone, L: Locale>(
    then: ::chrono::DateTime<Tz1>,
    now: ::chrono::DateTime<Tz2>,
    options: DurationOptions<L>,
) -> Result<AgoDisplay<L>, NegativeDurationError> {
    ago_with(now.signed_duration_since(then), options)
}

fn to_std(value: ::chrono::TimeDelta) -> Result<core::time::Duration, NegativeDurationError> {
    if value < ::chrono::TimeDelta::zero() {
        return Err(NegativeDurationError);
    }

    value.to_std().map_err(|_| NegativeDurationError)
}

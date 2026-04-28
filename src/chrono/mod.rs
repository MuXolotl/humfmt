//! Optional integration with [`chrono`](https://docs.rs/chrono).
//!
//! This module adapts `chrono::TimeDelta` and `chrono::DateTime` values into
//! `humfmt` duration and relative-time formatters while preserving the crate's
//! locale-aware options.
//!
//! # Feature flag
//!
//! Enable with:
//!
//! ```toml
//! humfmt = { version = "0.2", features = ["chrono"] }
//! ```
//!
//! # What this module provides
//!
//! - Convenience functions (`duration`, `ago`, `ago_since`, ...)
//! - Checked variants that return [`crate::DurationConversionError`]
//! - An extension trait [`ChronoHumanize`] for ergonomic usage
//!
//! # Notes on negativity and range
//!
//! `core::time::Duration` cannot represent negative durations. Therefore:
//!
//! - Negative `chrono::TimeDelta` values are rejected.
//! - Values that cannot be converted into `core::time::Duration` are rejected
//!   by checked functions with [`crate::DurationConversionError::OutOfRange`].
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
    ago::AgoDisplay, duration::DurationDisplay, locale::Locale, DurationConversionError,
    DurationOptions, NegativeDurationError,
};

/// Extension methods for `chrono::TimeDelta`.
///
/// This trait is intended for ergonomic usage:
///
/// ```rust
/// use humfmt::chrono::ChronoHumanize;
///
/// let delta = chrono::TimeDelta::try_seconds(90).unwrap();
/// assert_eq!(delta.try_human_ago().unwrap().to_string(), "1m 30s ago");
/// ```
pub trait ChronoHumanize: Sized {
    /// Formats this timedelta as a human-readable duration.
    ///
    /// Returns [`NegativeDurationError`] if the timedelta is negative.
    fn try_human_duration(self) -> Result<DurationDisplay, NegativeDurationError>;

    /// Formats this timedelta as a human-readable duration using custom options.
    ///
    /// Returns [`NegativeDurationError`] if the timedelta is negative.
    fn try_human_duration_with<L: Locale>(
        self,
        options: DurationOptions<L>,
    ) -> Result<DurationDisplay<L>, NegativeDurationError>;

    /// Formats this timedelta as relative time (e.g. `"1m 30s ago"`).
    ///
    /// Returns [`NegativeDurationError`] if the timedelta is negative.
    fn try_human_ago(self) -> Result<AgoDisplay, NegativeDurationError>;

    /// Formats this timedelta as relative time using custom duration options.
    ///
    /// Returns [`NegativeDurationError`] if the timedelta is negative.
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
///
/// Returns [`NegativeDurationError`] if the timedelta is negative.
///
/// # Examples
///
/// ```rust
/// use humfmt::chrono as humchrono;
///
/// let delta = chrono::TimeDelta::try_seconds(90).unwrap();
/// assert_eq!(humchrono::duration(delta).unwrap().to_string(), "1m 30s");
/// ```
pub fn duration(value: ::chrono::TimeDelta) -> Result<DurationDisplay, NegativeDurationError> {
    duration_with(value, DurationOptions::new())
}

/// Formats a non-negative `chrono::TimeDelta` with custom duration options.
///
/// Returns [`NegativeDurationError`] if the timedelta is negative.
pub fn duration_with<L: Locale>(
    value: ::chrono::TimeDelta,
    options: DurationOptions<L>,
) -> Result<DurationDisplay<L>, NegativeDurationError> {
    duration_with_checked(value, options).map_err(|_| NegativeDurationError)
}

/// Formats a `chrono::TimeDelta` with default duration options and explicit conversion errors.
///
/// This function distinguishes between negative inputs and out-of-range values
/// via [`DurationConversionError`].
pub fn duration_checked(
    value: ::chrono::TimeDelta,
) -> Result<DurationDisplay, DurationConversionError> {
    duration_with_checked(value, DurationOptions::new())
}

/// Formats a `chrono::TimeDelta` with custom duration options and explicit conversion errors.
///
/// This function distinguishes between negative inputs and out-of-range values
/// via [`DurationConversionError`].
pub fn duration_with_checked<L: Locale>(
    value: ::chrono::TimeDelta,
    options: DurationOptions<L>,
) -> Result<DurationDisplay<L>, DurationConversionError> {
    Ok(crate::duration::duration_with(
        to_std_checked(value)?,
        options,
    ))
}

/// Formats a non-negative `chrono::TimeDelta` as relative time using default options.
///
/// Returns [`NegativeDurationError`] if the timedelta is negative.
///
/// # Examples
///
/// ```rust
/// use humfmt::chrono as humchrono;
///
/// let delta = chrono::TimeDelta::try_seconds(90).unwrap();
/// assert_eq!(humchrono::ago(delta).unwrap().to_string(), "1m 30s ago");
/// ```
pub fn ago(value: ::chrono::TimeDelta) -> Result<AgoDisplay, NegativeDurationError> {
    ago_with(value, DurationOptions::new())
}

/// Formats a non-negative `chrono::TimeDelta` as relative time with custom options.
///
/// Returns [`NegativeDurationError`] if the timedelta is negative.
pub fn ago_with<L: Locale>(
    value: ::chrono::TimeDelta,
    options: DurationOptions<L>,
) -> Result<AgoDisplay<L>, NegativeDurationError> {
    ago_with_checked(value, options).map_err(|_| NegativeDurationError)
}

/// Formats a `chrono::TimeDelta` as relative time using default options and explicit conversion errors.
///
/// This function distinguishes between negative inputs and out-of-range values
/// via [`DurationConversionError`].
pub fn ago_checked(value: ::chrono::TimeDelta) -> Result<AgoDisplay, DurationConversionError> {
    ago_with_checked(value, DurationOptions::new())
}

/// Formats a `chrono::TimeDelta` as relative time with custom options and explicit conversion errors.
///
/// This function distinguishes between negative inputs and out-of-range values
/// via [`DurationConversionError`].
pub fn ago_with_checked<L: Locale>(
    value: ::chrono::TimeDelta,
    options: DurationOptions<L>,
) -> Result<AgoDisplay<L>, DurationConversionError> {
    Ok(crate::ago::ago_with(to_std_checked(value)?, options))
}

/// Formats the elapsed time between two `chrono` datetimes as relative time.
///
/// Returns [`NegativeDurationError`] if the elapsed duration is negative.
///
/// # Examples
///
/// ```rust
/// use humfmt::chrono as humchrono;
///
/// let then = chrono::DateTime::from_timestamp(0, 0).unwrap();
/// let now = chrono::DateTime::from_timestamp(90, 0).unwrap();
/// assert_eq!(humchrono::ago_since(then, now).unwrap().to_string(), "1m 30s ago");
/// ```
pub fn ago_since<Tz1: ::chrono::TimeZone, Tz2: ::chrono::TimeZone>(
    then: ::chrono::DateTime<Tz1>,
    now: ::chrono::DateTime<Tz2>,
) -> Result<AgoDisplay, NegativeDurationError> {
    ago_checked(now.signed_duration_since(then)).map_err(|_| NegativeDurationError)
}

/// Formats the elapsed time between two `chrono` datetimes as relative time using custom options.
///
/// Returns [`NegativeDurationError`] if the elapsed duration is negative.
pub fn ago_since_with<Tz1: ::chrono::TimeZone, Tz2: ::chrono::TimeZone, L: Locale>(
    then: ::chrono::DateTime<Tz1>,
    now: ::chrono::DateTime<Tz2>,
    options: DurationOptions<L>,
) -> Result<AgoDisplay<L>, NegativeDurationError> {
    ago_since_with_checked(then, now, options).map_err(|_| NegativeDurationError)
}

/// Formats the elapsed time between two `chrono` datetimes as relative time with explicit conversion errors.
///
/// This function distinguishes between negative inputs and out-of-range values
/// via [`DurationConversionError`].
pub fn ago_since_checked<Tz1: ::chrono::TimeZone, Tz2: ::chrono::TimeZone>(
    then: ::chrono::DateTime<Tz1>,
    now: ::chrono::DateTime<Tz2>,
) -> Result<AgoDisplay, DurationConversionError> {
    ago_checked(now.signed_duration_since(then))
}

/// Formats the elapsed time between two `chrono` datetimes as relative time with custom options and explicit conversion errors.
///
/// This function distinguishes between negative inputs and out-of-range values
/// via [`DurationConversionError`].
pub fn ago_since_with_checked<Tz1: ::chrono::TimeZone, Tz2: ::chrono::TimeZone, L: Locale>(
    then: ::chrono::DateTime<Tz1>,
    now: ::chrono::DateTime<Tz2>,
    options: DurationOptions<L>,
) -> Result<AgoDisplay<L>, DurationConversionError> {
    ago_with_checked(now.signed_duration_since(then), options)
}

fn to_std_checked(
    value: ::chrono::TimeDelta,
) -> Result<core::time::Duration, DurationConversionError> {
    if value < ::chrono::TimeDelta::zero() {
        return Err(DurationConversionError::NegativeDuration);
    }

    value
        .to_std()
        .map_err(|_| DurationConversionError::OutOfRange)
}

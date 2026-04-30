//! Optional integration with [`time`](https://docs.rs/time).
//!
//! This module adapts `time::Duration` and `time::OffsetDateTime` values into
//! `humfmt` duration and relative-time formatters while preserving the crate's
//! locale-aware options.
//!
//! # Feature flag
//!
//! Enable with:
//!
//! ```toml
//! humfmt = { version = "0.4", features = ["time"] }
//! ```
//!
//! # What this module provides
//!
//! - Convenience functions (`duration`, `ago`, `ago_since`, ...)
//! - Checked variants that return [`crate::DurationConversionError`]
//! - An extension trait [`TimeHumanize`] for ergonomic usage
//!
//! # Notes on negativity and range
//!
//! `core::time::Duration` cannot represent negative durations. Therefore:
//!
//! - Negative `time::Duration` values are rejected.
//! - The checked APIs return [`crate::DurationConversionError::NegativeDuration`] for negatives.
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
    ago::AgoDisplay, duration::DurationDisplay, locale::Locale, DurationConversionError,
    DurationOptions, NegativeDurationError,
};

/// Extension methods for `time::Duration`.
///
/// This trait is intended for ergonomic usage:
///
/// ```rust
/// use humfmt::time::TimeHumanize;
///
/// let delta = time::Duration::seconds(90);
/// assert_eq!(delta.try_human_ago().unwrap().to_string(), "1m 30s ago");
/// ```
pub trait TimeHumanize: Sized {
    /// Formats this duration as a human-readable duration.
    ///
    /// Returns [`NegativeDurationError`] if the duration is negative.
    fn try_human_duration(self) -> Result<DurationDisplay, NegativeDurationError>;

    /// Formats this duration as a human-readable duration using custom options.
    ///
    /// Returns [`NegativeDurationError`] if the duration is negative.
    fn try_human_duration_with<L: Locale>(
        self,
        options: DurationOptions<L>,
    ) -> Result<DurationDisplay<L>, NegativeDurationError>;

    /// Formats this duration as relative time (e.g. `"1m 30s ago"`).
    ///
    /// Returns [`NegativeDurationError`] if the duration is negative.
    fn try_human_ago(self) -> Result<AgoDisplay, NegativeDurationError>;

    /// Formats this duration as relative time using custom duration options.
    ///
    /// Returns [`NegativeDurationError`] if the duration is negative.
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
///
/// Returns [`NegativeDurationError`] if the duration is negative.
///
/// # Examples
///
/// ```rust
/// use humfmt::time as humtime;
///
/// let delta = time::Duration::seconds(90);
/// assert_eq!(humtime::duration(delta).unwrap().to_string(), "1m 30s");
/// ```
pub fn duration(value: ::time::Duration) -> Result<DurationDisplay, NegativeDurationError> {
    duration_with(value, DurationOptions::new())
}

/// Formats a non-negative `time::Duration` with custom duration options.
///
/// Returns [`NegativeDurationError`] if the duration is negative.
pub fn duration_with<L: Locale>(
    value: ::time::Duration,
    options: DurationOptions<L>,
) -> Result<DurationDisplay<L>, NegativeDurationError> {
    duration_with_checked(value, options).map_err(|_| NegativeDurationError)
}

/// Formats a `time::Duration` with default duration options and explicit conversion errors.
///
/// This function distinguishes negative inputs via [`DurationConversionError`].
pub fn duration_checked(
    value: ::time::Duration,
) -> Result<DurationDisplay, DurationConversionError> {
    duration_with_checked(value, DurationOptions::new())
}

/// Formats a `time::Duration` with custom duration options and explicit conversion errors.
///
/// This function distinguishes negative inputs via [`DurationConversionError`].
pub fn duration_with_checked<L: Locale>(
    value: ::time::Duration,
    options: DurationOptions<L>,
) -> Result<DurationDisplay<L>, DurationConversionError> {
    Ok(crate::duration::duration_with(
        to_std_checked(value)?,
        options,
    ))
}

/// Formats a non-negative `time::Duration` as relative time using default options.
///
/// Returns [`NegativeDurationError`] if the duration is negative.
///
/// # Examples
///
/// ```rust
/// use humfmt::time as humtime;
///
/// let delta = time::Duration::seconds(90);
/// assert_eq!(humtime::ago(delta).unwrap().to_string(), "1m 30s ago");
/// ```
pub fn ago(value: ::time::Duration) -> Result<AgoDisplay, NegativeDurationError> {
    ago_with(value, DurationOptions::new())
}

/// Formats a non-negative `time::Duration` as relative time with custom options.
///
/// Returns [`NegativeDurationError`] if the duration is negative.
pub fn ago_with<L: Locale>(
    value: ::time::Duration,
    options: DurationOptions<L>,
) -> Result<AgoDisplay<L>, NegativeDurationError> {
    ago_with_checked(value, options).map_err(|_| NegativeDurationError)
}

/// Formats a `time::Duration` as relative time using default options and explicit conversion errors.
///
/// This function distinguishes negative inputs via [`DurationConversionError`].
pub fn ago_checked(value: ::time::Duration) -> Result<AgoDisplay, DurationConversionError> {
    ago_with_checked(value, DurationOptions::new())
}

/// Formats a `time::Duration` as relative time with custom options and explicit conversion errors.
///
/// This function distinguishes negative inputs via [`DurationConversionError`].
pub fn ago_with_checked<L: Locale>(
    value: ::time::Duration,
    options: DurationOptions<L>,
) -> Result<AgoDisplay<L>, DurationConversionError> {
    Ok(crate::ago::ago_with(to_std_checked(value)?, options))
}

/// Formats the elapsed time between two `time::OffsetDateTime` values as relative time.
///
/// Returns [`NegativeDurationError`] if the elapsed duration is negative.
///
/// # Examples
///
/// ```rust
/// use humfmt::time as humtime;
///
/// let then = time::OffsetDateTime::from_unix_timestamp(0).unwrap();
/// let now = time::OffsetDateTime::from_unix_timestamp(90).unwrap();
/// assert_eq!(humtime::ago_since(then, now).unwrap().to_string(), "1m 30s ago");
/// ```
pub fn ago_since(
    then: ::time::OffsetDateTime,
    now: ::time::OffsetDateTime,
) -> Result<AgoDisplay, NegativeDurationError> {
    ago_checked(now - then).map_err(|_| NegativeDurationError)
}

/// Formats the elapsed time between two `time::OffsetDateTime` values as relative time using custom options.
///
/// Returns [`NegativeDurationError`] if the elapsed duration is negative.
pub fn ago_since_with<L: Locale>(
    then: ::time::OffsetDateTime,
    now: ::time::OffsetDateTime,
    options: DurationOptions<L>,
) -> Result<AgoDisplay<L>, NegativeDurationError> {
    ago_since_with_checked(then, now, options).map_err(|_| NegativeDurationError)
}

/// Formats the elapsed time between two `time::OffsetDateTime` values as relative time with explicit conversion errors.
///
/// This function distinguishes negative inputs via [`DurationConversionError`].
pub fn ago_since_checked(
    then: ::time::OffsetDateTime,
    now: ::time::OffsetDateTime,
) -> Result<AgoDisplay, DurationConversionError> {
    ago_checked(now - then)
}

/// Formats the elapsed time between two `time::OffsetDateTime` values as relative time with custom options and explicit conversion errors.
///
/// This function distinguishes negative inputs via [`DurationConversionError`].
pub fn ago_since_with_checked<L: Locale>(
    then: ::time::OffsetDateTime,
    now: ::time::OffsetDateTime,
    options: DurationOptions<L>,
) -> Result<AgoDisplay<L>, DurationConversionError> {
    ago_with_checked(now - then, options)
}

fn to_std_checked(
    value: ::time::Duration,
) -> Result<core::time::Duration, DurationConversionError> {
    if value.is_negative() {
        return Err(DurationConversionError::NegativeDuration);
    }

    Ok(value.unsigned_abs())
}

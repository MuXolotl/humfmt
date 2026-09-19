//! Optional integration with [`chrono`](https://docs.rs/chrono).
//!
//! This module adapts `chrono::TimeDelta` and `chrono::DateTime` values into
//! `humfmt` duration and relative-time formatters.
//!
//! # Feature flag
//!
//! Enable with:
//!
//! ```toml
//! humfmt = { version = "0.6", features = ["chrono"] }
//! ```
//!
//! # What this module provides
//!
//! - Convenience functions (`duration`, `ago`, `ago_since`, ...)
//! - Explicitly named `*_checked` twins of those functions
//! - An extension trait [`ChronoHumanize`] for ergonomic usage
//!
//! # Notes on negativity and range
//!
//! `core::time::Duration` is non-negative and bounded, so the conversion can
//! fail and every function here returns [`crate::DurationConversionError`]:
//!
//! - Negative `chrono::TimeDelta` values are rejected with
//!   [`crate::DurationConversionError::NegativeDuration`].
//! - [`crate::DurationConversionError::OutOfRange`] exists for values that do
//!   not fit in `core::time::Duration`; `chrono::TimeDelta::to_std` cannot fail
//!   for a non-negative value, so it is a defensive variant.
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

use crate::{ago::AgoDisplay, duration::DurationDisplay, DurationConversionError, DurationOptions};

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
    /// Returns [`DurationConversionError::NegativeDuration`] if the timedelta is negative.
    fn try_human_duration(self) -> Result<DurationDisplay, DurationConversionError>;

    /// Formats this timedelta as a human-readable duration using custom options.
    ///
    /// Returns [`DurationConversionError::NegativeDuration`] if the timedelta is negative.
    fn try_human_duration_with(
        self,
        options: DurationOptions,
    ) -> Result<DurationDisplay, DurationConversionError>;

    /// Formats this timedelta as relative time (e.g. `"1m 30s ago"`).
    ///
    /// Returns [`DurationConversionError::NegativeDuration`] if the timedelta is negative.
    fn try_human_ago(self) -> Result<AgoDisplay, DurationConversionError>;

    /// Formats this timedelta as relative time using custom options.
    ///
    /// Returns [`DurationConversionError::NegativeDuration`] if the timedelta is negative.
    fn try_human_ago_with(self, options: DurationOptions)
        -> Result<AgoDisplay, DurationConversionError>;
}

impl ChronoHumanize for ::chrono::TimeDelta {
    #[inline]
    fn try_human_duration(self) -> Result<DurationDisplay, DurationConversionError> {
        duration(self)
    }

    #[inline]
    fn try_human_duration_with(
        self,
        options: DurationOptions,
    ) -> Result<DurationDisplay, DurationConversionError> {
        duration_with(self, options)
    }

    #[inline]
    fn try_human_ago(self) -> Result<AgoDisplay, DurationConversionError> {
        ago(self)
    }

    #[inline]
    fn try_human_ago_with(
        self,
        options: DurationOptions,
    ) -> Result<AgoDisplay, DurationConversionError> {
        ago_with(self, options)
    }
}

/// Formats a non-negative `chrono::TimeDelta` with default duration options.
///
/// Returns [`DurationConversionError::NegativeDuration`] if the timedelta is negative.
///
/// # Examples
///
/// ```rust
/// use humfmt::chrono as humchrono;
///
/// let delta = chrono::TimeDelta::try_seconds(90).unwrap();
/// assert_eq!(humchrono::duration(delta).unwrap().to_string(), "1m 30s");
/// ```
pub fn duration(value: ::chrono::TimeDelta) -> Result<DurationDisplay, DurationConversionError> {
    duration_with(value, DurationOptions::new())
}

/// Formats a non-negative `chrono::TimeDelta` with custom duration options.
///
/// Returns [`DurationConversionError::NegativeDuration`] if the timedelta is negative.
pub fn duration_with(
    value: ::chrono::TimeDelta,
    options: DurationOptions,
) -> Result<DurationDisplay, DurationConversionError> {
    duration_with_checked(value, options)
}

/// Formats a `chrono::TimeDelta` with default duration options and explicit conversion errors.
///
/// Explicitly named twin of the same operation; the conversion can fail.
pub fn duration_checked(
    value: ::chrono::TimeDelta,
) -> Result<DurationDisplay, DurationConversionError> {
    duration_with_checked(value, DurationOptions::new())
}

/// Formats a `chrono::TimeDelta` with custom options and explicit conversion errors.
///
/// Explicitly named twin of the same operation; the conversion can fail.
pub fn duration_with_checked(
    value: ::chrono::TimeDelta,
    options: DurationOptions,
) -> Result<DurationDisplay, DurationConversionError> {
    Ok(crate::duration::duration_with(
        to_std_checked(value)?,
        options,
    ))
}

/// Formats a non-negative `chrono::TimeDelta` as relative time using default options.
///
/// Returns [`DurationConversionError::NegativeDuration`] if the timedelta is negative.
///
/// # Examples
///
/// ```rust
/// use humfmt::chrono as humchrono;
///
/// let delta = chrono::TimeDelta::try_seconds(90).unwrap();
/// assert_eq!(humchrono::ago(delta).unwrap().to_string(), "1m 30s ago");
/// ```
pub fn ago(value: ::chrono::TimeDelta) -> Result<AgoDisplay, DurationConversionError> {
    ago_with(value, DurationOptions::new())
}

/// Formats a non-negative `chrono::TimeDelta` as relative time with custom options.
///
/// Returns [`DurationConversionError::NegativeDuration`] if the timedelta is negative.
pub fn ago_with(
    value: ::chrono::TimeDelta,
    options: DurationOptions,
) -> Result<AgoDisplay, DurationConversionError> {
    ago_with_checked(value, options)
}

/// Formats a `chrono::TimeDelta` as relative time using default options and explicit conversion errors.
///
/// Explicitly named twin of the same operation; the conversion can fail.
pub fn ago_checked(value: ::chrono::TimeDelta) -> Result<AgoDisplay, DurationConversionError> {
    ago_with_checked(value, DurationOptions::new())
}

/// Formats a `chrono::TimeDelta` as relative time with custom options and explicit conversion errors.
///
/// Explicitly named twin of the same operation; the conversion can fail.
pub fn ago_with_checked(
    value: ::chrono::TimeDelta,
    options: DurationOptions,
) -> Result<AgoDisplay, DurationConversionError> {
    Ok(crate::ago::ago_with(to_std_checked(value)?, options))
}

/// Formats the elapsed time between two `chrono` datetimes as relative time.
///
/// Returns [`DurationConversionError::NegativeDuration`] if `now` is earlier than `then`.
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
) -> Result<AgoDisplay, DurationConversionError> {
    ago_checked(now.signed_duration_since(then))
}

/// Formats the elapsed time between two `chrono` datetimes as relative time using custom options.
///
/// Returns [`DurationConversionError::NegativeDuration`] if `now` is earlier than `then`.
pub fn ago_since_with<Tz1: ::chrono::TimeZone, Tz2: ::chrono::TimeZone>(
    then: ::chrono::DateTime<Tz1>,
    now: ::chrono::DateTime<Tz2>,
    options: DurationOptions,
) -> Result<AgoDisplay, DurationConversionError> {
    ago_since_with_checked(then, now, options)
}

/// Formats the elapsed time between two `chrono` datetimes as relative time with explicit conversion errors.
///
/// Explicitly named twin of the same operation; the conversion can fail.
pub fn ago_since_checked<Tz1: ::chrono::TimeZone, Tz2: ::chrono::TimeZone>(
    then: ::chrono::DateTime<Tz1>,
    now: ::chrono::DateTime<Tz2>,
) -> Result<AgoDisplay, DurationConversionError> {
    ago_checked(now.signed_duration_since(then))
}

/// Formats the elapsed time between two `chrono` datetimes as relative time
/// with custom options and explicit conversion errors.
///
/// Explicitly named twin of the same operation; the conversion can fail.
pub fn ago_since_with_checked<Tz1: ::chrono::TimeZone, Tz2: ::chrono::TimeZone>(
    then: ::chrono::DateTime<Tz1>,
    now: ::chrono::DateTime<Tz2>,
    options: DurationOptions,
) -> Result<AgoDisplay, DurationConversionError> {
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

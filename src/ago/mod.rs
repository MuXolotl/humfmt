//! Relative-time formatting.
//!
//! This module builds on [`crate::duration()`] and shares the same
//! configuration surface through [`crate::DurationOptions`].
//!
//! # Quick start
//!
//! ```rust
//! use core::time::Duration;
//! use humfmt::{ago, ago_with, DurationOptions};
//!
//! assert_eq!(ago(Duration::from_secs(90)).to_string(), "1m 30s ago");
//! assert_eq!(ago(Duration::from_secs(3661)).to_string(), "1h 1m ago");
//! ```
//!
//! # Limitations
//!
//! **Past only:** Currently `ago` only formats past durations (time that has
//! already elapsed). Future-time support (`"in 5 minutes"`) is planned.
//!
//! **No "just now" case:** Very small durations (e.g. under 5 seconds) render
//! as `"0s ago"` rather than a special "just now" phrase. A configurable
//! threshold for this case is planned.

mod display;

pub use display::AgoDisplay;

use crate::duration::{DurationLike, DurationOptions};

/// Creates a human-readable relative-time formatter using default options.
///
/// # Examples
///
/// ```rust
/// let elapsed = core::time::Duration::from_secs(3661);
/// assert_eq!(humfmt::ago(elapsed).to_string(), "1h 1m ago");
/// ```
pub fn ago<T: DurationLike>(value: T) -> AgoDisplay {
    AgoDisplay::new(value, DurationOptions::new())
}

/// Creates a human-readable relative-time formatter with custom duration options.
///
/// # Examples
///
/// ```rust
/// use humfmt::DurationOptions;
///
/// let elapsed = core::time::Duration::from_millis(1500);
/// let out = humfmt::ago_with(elapsed, DurationOptions::new().long_units());
/// assert_eq!(out.to_string(), "1 second 500 milliseconds ago");
/// ```
pub fn ago_with<T: DurationLike>(value: T, options: DurationOptions) -> AgoDisplay {
    AgoDisplay::new(value, options)
}

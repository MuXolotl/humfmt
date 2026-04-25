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

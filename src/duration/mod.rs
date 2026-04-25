//! Human-readable duration formatting.
//!
//! This module focuses on compact and long-form duration rendering and shares
//! its locale configuration with relative-time formatting.

mod display;
mod format;
mod options;
mod traits;

pub use display::DurationDisplay;
pub use options::DurationOptions;
pub use traits::DurationLike;

use crate::locale::{English, Locale};

/// Creates a human-readable duration formatter using default options.
///
/// # Examples
///
/// ```rust
/// let value = core::time::Duration::from_secs(3661);
/// assert_eq!(humfmt::duration(value).to_string(), "1h 1m");
/// ```
pub fn duration<T: DurationLike>(value: T) -> DurationDisplay<English> {
    DurationDisplay::new(value.into_duration(), DurationOptions::new())
}

/// Creates a human-readable duration formatter with custom options.
///
/// # Examples
///
/// ```rust
/// use humfmt::DurationOptions;
///
/// let value = core::time::Duration::from_millis(1500);
/// let out = humfmt::duration_with(value, DurationOptions::new().long_units());
/// assert_eq!(out.to_string(), "1 second 500 milliseconds");
/// ```
pub fn duration_with<T: DurationLike, L: Locale>(
    value: T,
    options: DurationOptions<L>,
) -> DurationDisplay<L> {
    DurationDisplay::new(value.into_duration(), options)
}

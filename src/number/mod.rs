//! Compact number formatting.
//!
//! This module turns large integers and floats into short or long human-readable
//! forms while respecting locale-specific separators and suffix rules.

mod display;
mod format;
mod options;
mod traits;

pub use display::NumberDisplay;
pub use options::NumberOptions;
pub use traits::NumberLike;

use crate::locale::{English, Locale};

/// Creates a human-readable compact formatter using default options.
///
/// # Examples
///
/// ```rust
/// assert_eq!(humfmt::number(15320).to_string(), "15.3K");
/// assert_eq!(humfmt::number(1_500_000).to_string(), "1.5M");
/// ```
pub fn number<T: NumberLike>(value: T) -> NumberDisplay<English> {
    NumberDisplay::new(value.into_numeric(), NumberOptions::new())
}

/// Creates a human-readable compact formatter with custom options.
///
/// # Examples
///
/// ```rust
/// use humfmt::NumberOptions;
///
/// let out = humfmt::number_with(15320, NumberOptions::new().long_units());
/// assert_eq!(out.to_string(), "15.3 thousand");
/// ```
pub fn number_with<T: NumberLike, L: Locale>(
    value: T,
    options: NumberOptions<L>,
) -> NumberDisplay<L> {
    NumberDisplay::new(value.into_numeric(), options)
}

//! Ordinal formatting.
//!
//! Use this module for locale-aware ordinal markers such as `1st`, `21.` or
//! `42-й`.

mod display;
mod traits;

pub use display::OrdinalDisplay;
pub use traits::OrdinalLike;

use crate::locale::{English, Locale};

/// Creates a human-readable ordinal formatter using the default English locale.
///
/// # Examples
///
/// ```rust
/// assert_eq!(humfmt::ordinal(1).to_string(), "1st");
/// assert_eq!(humfmt::ordinal(23).to_string(), "23rd");
/// ```
pub fn ordinal<T: OrdinalLike>(value: T) -> OrdinalDisplay<English> {
    OrdinalDisplay::new(value.into_ordinal(), English)
}

/// Creates a human-readable ordinal formatter with a custom locale.
///
/// # Examples
///
/// ```rust
/// use humfmt::locale::English;
///
/// let out = humfmt::ordinal_with(11, English);
/// assert_eq!(out.to_string(), "11th");
/// ```
pub fn ordinal_with<T: OrdinalLike, L: Locale>(value: T, locale: L) -> OrdinalDisplay<L> {
    OrdinalDisplay::new(value.into_ordinal(), locale)
}

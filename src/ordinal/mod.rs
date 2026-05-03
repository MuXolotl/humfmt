//! Ordinal formatting.
//!
//! Use this module for locale-aware ordinal markers such as `1st`, `21.` or `42-й`.
//!
//! # Quick start
//!
//! ```rust
//! use humfmt::{ordinal, ordinal_with};
//!
//! assert_eq!(ordinal(1).to_string(), "1st");
//! assert_eq!(ordinal(21).to_string(), "21st");
//! assert_eq!(ordinal(11).to_string(), "11th");
//! ```
//!
//! # Edge case behaviour
//!
//! | Input | English | Russian | Polish |
//! |---:|---|---|---|
//! | `1` | `"1st"` | `"1-й"` | `"1."` |
//! | `2` | `"2nd"` | `"2-й"` | `"2."` |
//! | `11` | `"11th"` | `"11-й"` | `"11."` |
//! | `21` | `"21st"` | `"21-й"` | `"21."` |
//! | `103` | `"103rd"` | `"103-й"` | `"103."` |
//! | `111` | `"111th"` | `"111-й"` | `"111."` |
//! | `-1` | `"-1st"` | `"-1-й"` | `"-1."` |
//!
//! # Limitations
//!
//! **Russian gender:** The Russian ordinal suffix is always `-й` (masculine).
//! The library has no concept of grammatical gender since it only receives a
//! number. If you need feminine or neuter ordinals in Russian, you must handle
//! that outside the formatter.

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

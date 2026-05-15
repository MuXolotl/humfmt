//! Ordinal formatting (English).
//!
//! Use this module to render English ordinal markers like `1st`, `2nd`, `3rd`,
//! `4th`, ..., with the standard teen exceptions.
//!
//! # Quick start
//!
//! ```rust
//! use humfmt::ordinal;
//!
//! assert_eq!(ordinal(1).to_string(), "1st");
//! assert_eq!(ordinal(21).to_string(), "21st");
//! assert_eq!(ordinal(11).to_string(), "11th");
//! assert_eq!(ordinal(-1).to_string(), "-1st");
//! ```
//!
//! # Edge case behaviour
//!
//! | Input | Output |
//! |---:|---|
//! | `1` | `"1st"` |
//! | `2` | `"2nd"` |
//! | `3` | `"3rd"` |
//! | `4` | `"4th"` |
//! | `11` | `"11th"` |
//! | `12` | `"12th"` |
//! | `13` | `"13th"` |
//! | `21` | `"21st"` |
//! | `42` | `"42nd"` |
//! | `103` | `"103rd"` |
//! | `111` | `"111th"` |
//! | `-1` | `"-1st"` |
//! | `0` | `"0th"` |

mod display;
mod traits;

pub use display::OrdinalDisplay;
pub use traits::OrdinalLike;

/// Creates a human-readable ordinal formatter.
///
/// # Examples
///
/// ```rust
/// assert_eq!(humfmt::ordinal(1).to_string(), "1st");
/// assert_eq!(humfmt::ordinal(23).to_string(), "23rd");
/// ```
pub fn ordinal<T: OrdinalLike>(value: T) -> OrdinalDisplay {
    OrdinalDisplay::new(value.into_ordinal())
}

/// Returns the English ordinal suffix for a given non-negative integer.
///
/// Used internally and exposed for users who want just the suffix
/// without the value (e.g. for custom rendering pipelines).
///
/// # Examples
///
/// ```rust
/// assert_eq!(humfmt::ordinal::ordinal_suffix(1), "st");
/// assert_eq!(humfmt::ordinal::ordinal_suffix(11), "th");
/// assert_eq!(humfmt::ordinal::ordinal_suffix(22), "nd");
/// ```
#[inline]
pub fn ordinal_suffix(n: u128) -> &'static str {
    match n % 10 {
        1 if n % 100 != 11 => "st",
        2 if n % 100 != 12 => "nd",
        3 if n % 100 != 13 => "rd",
        _ => "th",
    }
}

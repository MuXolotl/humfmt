//! Compact number formatting.
//!
//! Turns large integers and floats into short or long human-readable forms.
//!
//! # Quick start
//!
//! ```rust
//! use humfmt::{number, number_with, NumberOptions};
//!
//! assert_eq!(number(15_320).to_string(), "15.3K");
//! assert_eq!(number(1_500_000).to_string(), "1.5M");
//! assert_eq!(number(-12_500).to_string(), "-12.5K");
//!
//! let opts = NumberOptions::new().long_units();
//! assert_eq!(number_with(15_320, opts).to_string(), "15.3 thousand");
//! ```
//!
//! # Edge case behaviour
//!
//! | Input | Default output | Notes |
//! |---:|---|---|
//! | `0` | `"0"` | No suffix, no sign |
//! | `1` | `"1"` | Below threshold |
//! | `999` | `"999"` | Below threshold |
//! | `1_000` | `"1K"` | First compact threshold |
//! | `999_950` | `"1M"` | Rounds up across suffix boundary |
//! | `-1` | `"-1"` | Sign preserved |
//! | `-12_500` | `"-12.5K"` | Sign + compact |
//! | `i128::MIN` | `"-170.1Dc"` | No panic, no overflow |
//! | `u128::MAX` | `"340.3Dc"` | No panic, no overflow |
//! | `0.0` | `"0"` | |
//! | `-0.0` | `"0"` | Negative zero suppressed |
//! | `-0.004` | `"0"` | Rounds to zero, sign suppressed |
//! | `f64::INFINITY` | `"inf"` | |
//! | `f64::NEG_INFINITY` | `"-inf"` | |
//! | `f64::NAN` | `"NaN"` | |
//!
//! # Suffix table
//!
//! | Index | Magnitude | Short | Long |
//! |:---:|---:|---|---|
//! | 0 | 1 | `""` | `""` |
//! | 1 | 10^3 | `K` | ` thousand` |
//! | 2 | 10^6 | `M` | ` million` |
//! | 3 | 10^9 | `B` | ` billion` |
//! | 4 | 10^12 | `T` | ` trillion` |
//! | 5 | 10^15 | `Qa` | ` quadrillion` |
//! | 6 | 10^18 | `Qi` | ` quintillion` |
//! | 7 | 10^21 | `Sx` | ` sextillion` |
//! | 8 | 10^24 | `Sp` | ` septillion` |
//! | 9 | 10^27 | `Oc` | ` octillion` |
//! | 10 | 10^30 | `No` | ` nonillion` |
//! | 11 | 10^33 | `Dc` | ` decillion` |
//!
//! # Float precision limits
//!
//! `f64` cannot exactly represent integers above `2^53`. For very large float
//! inputs, compact formatting may lose a few digits in the integer part before
//! scaling. This is a known limitation of IEEE 754 double precision and is
//! acceptable for display purposes.

mod display;
mod format;
mod options;
mod traits;

pub use display::NumberDisplay;
pub use options::NumberOptions;
pub use traits::NumberLike;

/// Creates a human-readable compact number formatter using default options.
///
/// Accepts all integer primitives (`i8`..`i128`, `u8`..`u128`, `isize`, `usize`)
/// and floats (`f32`, `f64`).
///
/// # Examples
///
/// ```rust
/// assert_eq!(humfmt::number(15_320).to_string(), "15.3K");
/// assert_eq!(humfmt::number(1_500_000).to_string(), "1.5M");
/// assert_eq!(humfmt::number(-12_500).to_string(), "-12.5K");
/// assert_eq!(humfmt::number(0).to_string(), "0");
/// assert_eq!(humfmt::number(f64::NAN).to_string(), "NaN");
/// ```
pub fn number<T: NumberLike>(value: T) -> NumberDisplay {
    NumberDisplay::new(value.into_numeric(), NumberOptions::new())
}

/// Creates a human-readable compact number formatter with custom options.
///
/// # Examples
///
/// ```rust
/// use humfmt::NumberOptions;
///
/// let out = humfmt::number_with(15_320, NumberOptions::new().long_units());
/// assert_eq!(out.to_string(), "15.3 thousand");
///
/// let out = humfmt::number_with(15_320, NumberOptions::new().precision(2));
/// assert_eq!(out.to_string(), "15.32K");
/// ```
pub fn number_with<T: NumberLike>(value: T, options: NumberOptions) -> NumberDisplay {
    NumberDisplay::new(value.into_numeric(), options)
}

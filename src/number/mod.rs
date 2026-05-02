//! Compact number formatting.
//!
//! Turns large integers and floats into short or long human-readable forms
//! while respecting locale-specific separators and suffix rules.
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
//! | `f64::INFINITY` | `"inf"` | Locale-agnostic |
//! | `f64::NEG_INFINITY` | `"-inf"` | Locale-agnostic |
//! | `f64::NAN` | `"NaN"` | Locale-agnostic |
//!
//! # Rounding
//!
//! All rounding uses half-up (round half away from zero). The integer path
//! uses exact `u128` long-division arithmetic. The float path uses
//! integer-cast rounding (`(x * 10^p + 0.5) as u64 / 10^p`) which is
//! equivalent for values in the safe range and avoids `std`-only float APIs.
//!
//! # Suffix rescaling
//!
//! When rounding pushes the scaled integer part to `1_000` or above, the
//! formatter rescales to the next suffix automatically:
//! `999_950` at `precision(1)` → `999.95K` → rounds to `1000K` → rescales to `1M`.
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

use crate::locale::{English, Locale};

/// Creates a human-readable compact number formatter using default options.
///
/// Accepts all integer primitives (`i8`–`i128`, `u8`–`u128`, `isize`, `usize`)
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
pub fn number<T: NumberLike>(value: T) -> NumberDisplay<English> {
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
pub fn number_with<T: NumberLike, L: Locale>(
    value: T,
    options: NumberOptions<L>,
) -> NumberDisplay<L> {
    NumberDisplay::new(value.into_numeric(), options)
}

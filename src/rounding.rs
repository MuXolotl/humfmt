//! Rounding primitives shared by the formatters: the public [`RoundingMode`]
//! and the internal decimal-precision helpers.

/// Specifies how numerical values should be rounded.
///
/// Used by [`NumberOptions`](crate::NumberOptions),
/// [`BytesOptions`](crate::BytesOptions), and
/// [`PercentOptions`](crate::PercentOptions).
///
/// # Examples
///
/// ```rust
/// use humfmt::{number_with, NumberOptions, RoundingMode};
///
/// let base = NumberOptions::new().precision(0);
///
/// // HalfUp: standard rounding, ties away from zero
/// assert_eq!(number_with(1_500, base.rounding(RoundingMode::HalfUp)).to_string(), "2K");
///
/// // Floor: always round down (towards negative infinity)
/// assert_eq!(number_with(1_900, base.rounding(RoundingMode::Floor)).to_string(), "1K");
///
/// // Ceil: always round up (towards positive infinity)
/// assert_eq!(number_with(1_100, base.rounding(RoundingMode::Ceil)).to_string(), "2K");
/// ```
///
/// # Behaviour
///
/// | Mode | Positive value | Negative value |
/// |---|---|---|
/// | `HalfUp` | `1.5` → `2` | `-1.5` → `-2` |
/// | `Floor` | `1.9` → `1` | `-1.1` → `-2` |
/// | `Ceil` | `1.1` → `2` | `-1.9` → `-1` |
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
pub enum RoundingMode {
    /// Round to the nearest value, with ties rounding away from zero (default).
    #[default]
    HalfUp,
    /// Round towards negative infinity (down).
    Floor,
    /// Round towards positive infinity (up).
    Ceil,
}

/// Selects between a fixed number of decimal places and a fixed number of
/// significant digits.
///
/// Shared by [`NumberOptions`](crate::NumberOptions) and
/// [`BytesOptions`](crate::BytesOptions); `percent` always counts decimal
/// places.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(crate) enum Precision {
    Decimals(u8),
    Significant(u8),
}

/// Magnitude at which `f64` values are all integers.
const TWO_POW_52: f64 = (1u64 << 52) as f64;

// Powers of ten as f64, indexed by decimal precision (0..=6).
const POW10_F64: [f64; 7] = [1.0, 10.0, 100.0, 1_000.0, 10_000.0, 100_000.0, 1_000_000.0];

/// Returns `10^precision` for a decimal precision clamped to `0..=6`.
#[inline]
pub(crate) fn decimal_factor(precision: u8) -> f64 {
    POW10_F64[precision.min(6) as usize]
}

/// Decides whether truncating a value increments it, given whether the dropped
/// fraction reaches a half and whether anything was dropped at all.
///
/// `Floor` and `Ceil` are defined on the signed value, so callers that work
/// with magnitudes pass the sign separately.
#[inline]
pub(crate) fn carry_after_truncation(
    dropped_at_least_half: bool,
    has_remainder: bool,
    rounding: RoundingMode,
    is_negative: bool,
) -> bool {
    match rounding {
        RoundingMode::HalfUp => dropped_at_least_half,
        RoundingMode::Floor => is_negative && has_remainder,
        RoundingMode::Ceil => !is_negative && has_remainder,
    }
}

/// Rounds a non-negative finite `f64` to `precision` decimal places.
///
/// Values whose scaled magnitude reaches the `u64` range are returned
/// unchanged: an `f64` that large is an integer, so it has no fractional
/// digits to round. Callers that must print such values fall back to the exact
/// decimal expansion produced by `{:.*}`.
#[inline]
pub(crate) fn round_to_decimals(
    value: f64,
    precision: u8,
    rounding: RoundingMode,
    is_negative: bool,
) -> f64 {
    debug_assert!(value.is_finite() && value >= 0.0);

    let factor = decimal_factor(precision);
    let shifted = value * factor;

    if shifted >= u64::MAX as f64 {
        return value;
    }

    let truncated = shifted as u64;
    let has_remainder = shifted > truncated as f64;

    // Adding `0.5` is exact below 2^52 and answers the half-up question
    // directly; at 2^52 and above it rounds to an even integer and would report
    // a carry for a scaled value that has no fractional digits left.
    let dropped_at_least_half = shifted < TWO_POW_52 && (shifted + 0.5) as u64 > truncated;

    let carry = carry_after_truncation(dropped_at_least_half, has_remainder, rounding, is_negative);

    let rounded = if carry { truncated + 1 } else { truncated };

    rounded as f64 / factor
}

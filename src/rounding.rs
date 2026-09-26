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

    if value * factor >= u64::MAX as f64 {
        return value;
    }

    let bits = value.to_bits();
    let exp_bias = ((bits >> 52) & 0x7FF) as i32;
    let mantissa = bits & 0xFFFFFFFFFFFFF;

    let (m, e) = if exp_bias == 0 {
        // Subnormal
        (mantissa, -1022 - 52)
    } else {
        // Normal
        (mantissa | 0x10000000000000, exp_bias - 1023 - 52)
    };

    let p = precision as i32;
    let pow5 = 5_u128.pow(precision as u32);
    // m is at most 53 bits. 5^6 is 15625 (14 bits).
    // m_pow5 fits comfortably in 67 bits, well within u128.
    let m_pow5 = (m as u128) * pow5;

    // We want X = M * 2^E * 10^P = (M * 5^P) * 2^{E+P}
    let shift = e + p;

    let (truncated, remainder_is_zero, dropped_at_least_half) = if shift >= 0 {
        ((m_pow5 << shift) as u64, true, false)
    } else {
        let right_shift = -shift as u32;
        if right_shift >= 128 {
            (0, m_pow5 == 0, false)
        } else {
            let truncated = (m_pow5 >> right_shift) as u64;
            let mask = (1_u128 << right_shift) - 1;
            let remainder = m_pow5 & mask;
            let half = 1_u128 << (right_shift - 1);
            let dropped_at_least_half = remainder >= half;
            (truncated, remainder == 0, dropped_at_least_half)
        }
    };

    let carry = carry_after_truncation(
        dropped_at_least_half,
        !remainder_is_zero,
        rounding,
        is_negative,
    );

    let rounded = if carry { truncated + 1 } else { truncated };

    rounded as f64 / factor
}

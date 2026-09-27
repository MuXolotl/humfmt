//! Rounding primitives shared by the formatters: the public [`RoundingMode`],
//! the internal precision selector and the carry rule.

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

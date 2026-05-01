#[derive(Copy, Clone, Debug)]
pub enum NumericValue {
    Int(i128),
    UInt(u128),
    Float(f64),
}

/// Returns `true` if `value` is a finite float with no fractional part.
///
/// This is the correct `no_std`-compatible check. It avoids casting to an
/// integer type, which saturates for negative values and large floats on
/// stable Rust, producing wrong results.
///
/// Used by locale packs that need to distinguish whole from fractional
/// scaled values for grammatical agreement (Russian, Polish, etc.).
#[allow(dead_code)]
#[inline]
pub fn is_integer_f64(value: f64) -> bool {
    value.is_finite() && value % 1.0 == 0.0
}

#[cfg(test)]
mod tests {
    use super::is_integer_f64;

    #[test]
    fn whole_floats_are_integers() {
        assert!(is_integer_f64(0.0));
        assert!(is_integer_f64(1.0));
        assert!(is_integer_f64(-1.0));
        assert!(is_integer_f64(1_000_000.0));
        assert!(is_integer_f64(-1_000_000.0));
    }

    #[test]
    fn fractional_floats_are_not_integers() {
        assert!(!is_integer_f64(0.5));
        assert!(!is_integer_f64(1.5));
        assert!(!is_integer_f64(-1.5));
        assert!(!is_integer_f64(0.001));
    }

    #[test]
    fn non_finite_floats_are_not_integers() {
        assert!(!is_integer_f64(f64::INFINITY));
        assert!(!is_integer_f64(f64::NEG_INFINITY));
        assert!(!is_integer_f64(f64::NAN));
    }

    #[test]
    fn large_negative_whole_float_is_integer() {
        // The old `value as u128` cast saturated to 0 for negative values,
        // causing -1.0, -42.0 etc. to incorrectly return false.
        // The `% 1.0 == 0.0` check is correct for all finite values.
        assert!(is_integer_f64(-1.0));
        assert!(is_integer_f64(-42.0));
        assert!(is_integer_f64(-999_999.0));
    }

    #[test]
    fn very_large_float_near_f64_precision_limit() {
        // 2^53 is the largest integer exactly representable as f64.
        let max_exact = (1u64 << 53) as f64;
        assert!(is_integer_f64(max_exact));
        assert!(is_integer_f64(-max_exact));
    }
}

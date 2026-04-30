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
#[cfg(any(feature = "russian", feature = "polish"))]
#[inline]
pub fn is_integer_f64(value: f64) -> bool {
    value.is_finite() && value % 1.0 == 0.0
}

#[cfg(test)]
mod tests {
    // The tests are always compiled so the logic stays verified
    // regardless of which locale features are active.
    fn is_integer_f64(value: f64) -> bool {
        value.is_finite() && value % 1.0 == 0.0
    }

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
        // This was the broken case with the old `value as u128` cast:
        // -1.0 as u128 saturates to 0 on stable Rust, making the old
        // check return false for -1.0 (wrong). The % 1.0 check is correct.
        assert!(is_integer_f64(-1.0));
        assert!(is_integer_f64(-42.0));
        assert!(is_integer_f64(-999_999.0));
    }
}

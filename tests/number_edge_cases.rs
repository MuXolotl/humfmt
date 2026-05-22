use humfmt::{number, number_with, NumberOptions};

// --- Zero and one ---

#[test]
fn formats_zero_for_all_main_numeric_kinds() {
    assert_eq!(number(0_i32).to_string(), "0");
    assert_eq!(number(0_u64).to_string(), "0");
    assert_eq!(number(0.0_f64).to_string(), "0");
    assert_eq!(number(-0.0_f64).to_string(), "0");
}

#[test]
fn formats_one_for_all_main_numeric_kinds() {
    assert_eq!(number(1_i32).to_string(), "1");
    assert_eq!(number(1_u64).to_string(), "1");
    assert_eq!(number(1.0_f64).to_string(), "1");
}

#[test]
fn formats_negative_one_for_signed_numbers_and_floats() {
    assert_eq!(number(-1_i32).to_string(), "-1");
    assert_eq!(number(-1_i128).to_string(), "-1");
    assert_eq!(number(-1.0_f64).to_string(), "-1");
}

// --- Negative zero and rounded zero ---

#[test]
fn avoids_negative_zero_output() {
    assert_eq!(number(-0.0).to_string(), "0");
}

#[test]
fn avoids_negative_zero_after_rounding_small_floats() {
    assert_eq!(number(-0.04).to_string(), "0");

    assert_eq!(
        number_with(-0.004, NumberOptions::new().precision(2)).to_string(),
        "0"
    );
}

#[test]
fn very_small_floats_round_to_zero() {
    assert_eq!(number(0.001_f64).to_string(), "0");
    assert_eq!(number(0.04_f64).to_string(), "0");
    assert_eq!(number(0.05_f64).to_string(), "0.1");

    assert_eq!(number(-0.001_f64).to_string(), "0");
    assert_eq!(number(-0.04_f64).to_string(), "0");
}

#[test]
fn force_sign_still_suppresses_sign_for_rounded_zero() {
    let opts = NumberOptions::new().force_sign(true).precision(1);

    assert_eq!(number_with(0.004_f64, opts).to_string(), "0");
    assert_eq!(number_with(-0.004_f64, opts).to_string(), "0");
}

// --- Non-finite floats ---

#[test]
fn preserves_non_finite_values() {
    assert_eq!(number(f64::INFINITY).to_string(), "inf");
    assert_eq!(number(f64::NEG_INFINITY).to_string(), "-inf");
    assert_eq!(number(f64::NAN).to_string(), "NaN");
}

#[test]
fn preserves_non_finite_f32_values() {
    assert_eq!(number(f32::INFINITY).to_string(), "inf");
    assert_eq!(number(f32::NEG_INFINITY).to_string(), "-inf");
    assert_eq!(number(f32::NAN).to_string(), "NaN");
}

// --- Suffix boundaries ---

#[test]
fn rescales_after_rounding_boundary() {
    assert_eq!(number(999_950).to_string(), "1M");
    assert_eq!(number(999_999).to_string(), "1M");
}

#[test]
fn default_precision_boundaries_around_one_million() {
    assert_eq!(number(999_499).to_string(), "999.5K");
    assert_eq!(number(999_500).to_string(), "999.5K");
    assert_eq!(number(999_949).to_string(), "999.9K");
    assert_eq!(number(999_950).to_string(), "1M");
    assert_eq!(number(999_999).to_string(), "1M");
    assert_eq!(number(1_000_000).to_string(), "1M");
}

#[test]
fn zero_precision_boundaries_around_one_million() {
    let opts = NumberOptions::new().precision(0);

    assert_eq!(number_with(999_499, opts).to_string(), "999K");
    assert_eq!(number_with(999_500, opts).to_string(), "1M");
    assert_eq!(number_with(999_999, opts).to_string(), "1M");
    assert_eq!(number_with(1_000_000, opts).to_string(), "1M");
}

#[test]
fn float_just_below_rescale_boundary() {
    let out = number_with(999_949.0_f64, NumberOptions::new().precision(1)).to_string();
    assert_eq!(out, "999.9K");
}

#[test]
fn float_precision_zero_at_rescale_boundary() {
    let opts = NumberOptions::new().precision(0);

    assert_eq!(number_with(999_500.0_f64, opts).to_string(), "1M");
    assert_eq!(number_with(999_499.0_f64, opts).to_string(), "999K");
}

#[test]
fn float_at_exact_suffix_boundary() {
    assert_eq!(number(1_000.0_f64).to_string(), "1K");
    assert_eq!(number(1_000_000.0_f64).to_string(), "1M");
    assert_eq!(number(1_000_000_000.0_f64).to_string(), "1B");
}

#[test]
fn float_scaling_matches_integer_scaling_for_round_values() {
    let pairs: &[(f64, &str)] = &[
        (1_000.0, "1K"),
        (1_500.0, "1.5K"),
        (1_000_000.0, "1M"),
        (1_500_000.0, "1.5M"),
        (1_000_000_000.0, "1B"),
        (1_000_000_000_000.0, "1T"),
        (1_000_000_000_000_000.0, "1Qa"),
    ];

    for &(input, expected) in pairs {
        assert_eq!(
            number(input).to_string(),
            expected,
            "failed for input {input}"
        );
    }
}

#[test]
fn rounds_from_decillion_to_undecillion_boundary() {
    // 999_950 * 10^30 = 999.95 decillion.
    // With default precision=1 this rounds to 1000.0Dc, then rescales to 1Ud.
    let value = 999_950_u128 * 1_000_000_000_000_000_000_000_000_000_000_u128;

    assert_eq!(number(value).to_string(), "1Ud");
}

// --- Tiny finite floats ---

#[test]
fn very_small_positive_float_below_threshold() {
    assert_eq!(number(0.1_f64).to_string(), "0.1");
    assert_eq!(number(0.01_f64).to_string(), "0");
    assert_eq!(number(999.9_f64).to_string(), "999.9");
}

#[test]
fn very_small_negative_float_below_threshold() {
    assert_eq!(number(-0.1_f64).to_string(), "-0.1");
    assert_eq!(number(-999.9_f64).to_string(), "-999.9");
}

#[test]
fn minimum_positive_finite_float_rounds_to_zero() {
    assert_eq!(number(f64::MIN_POSITIVE).to_string(), "0");
}

#[test]
fn negative_minimum_positive_finite_float_rounds_to_zero() {
    assert_eq!(number(-f64::MIN_POSITIVE).to_string(), "0");
}

// --- Sign symmetry ---

#[test]
fn sign_symmetry_for_floats() {
    let values: &[f64] = &[0.5, 1.0, 1.5, 100.0, 1_000.0, 1_500.0, 1_000_000.0];

    for &value in values {
        let positive = number(value).to_string();
        let negative = number(-value).to_string();

        assert_eq!(
            negative,
            format!("-{positive}"),
            "sign symmetry failed for {value}"
        );
    }
}

// --- Primitive input coverage ---

#[test]
fn formats_small_integer_types() {
    assert_eq!(number(u8::MAX).to_string(), "255");
    assert_eq!(number(i8::MIN).to_string(), "-128");
    assert_eq!(number(i8::MAX).to_string(), "127");
    assert_eq!(number(u16::MAX).to_string(), "65.5K");
    assert_eq!(number(i16::MIN).to_string(), "-32.8K");
}

#[test]
fn formats_usize_and_isize() {
    assert_eq!(number(0_usize).to_string(), "0");
    assert_eq!(number(1_000_usize).to_string(), "1K");

    assert_eq!(number(0_isize).to_string(), "0");
    assert_eq!(number(-1_000_isize).to_string(), "-1K");
}

#[test]
fn formats_f32_input() {
    assert_eq!(number(0.0_f32).to_string(), "0");
    assert_eq!(number(1_000.0_f32).to_string(), "1K");
    assert_eq!(number(1_500.0_f32).to_string(), "1.5K");
    assert_eq!(number(-1_500.0_f32).to_string(), "-1.5K");
}

// --- Extreme integer values ---

#[test]
fn formats_i128_min_with_exact_default_output() {
    assert_eq!(number(i128::MIN).to_string(), "-170.1Ud");
}

#[test]
fn formats_u128_max_with_exact_default_output() {
    assert_eq!(number(u128::MAX).to_string(), "340.3Ud");
}

#[test]
fn formats_i128_min_without_overflow_artifacts() {
    let out = number(i128::MIN).to_string();

    assert!(out.starts_with('-'));
    assert!(out.ends_with("Ud"));
    assert!(!out.contains("inf"));
    assert!(!out.contains("NaN"));
}

#[test]
fn formats_u128_max_without_overflow_artifacts() {
    let out = number(u128::MAX).to_string();

    assert!(out.ends_with("Ud"));
    assert!(!out.contains("inf"));
    assert!(!out.contains("NaN"));
}

#[test]
fn uncompacted_u128_max_is_exact() {
    let opts = NumberOptions::new().compact(false);
    assert_eq!(
        number_with(u128::MAX, opts).to_string(),
        u128::MAX.to_string()
    );
}

#[test]
fn uncompacted_i128_min_is_exact() {
    let opts = NumberOptions::new().compact(false);
    assert_eq!(
        number_with(i128::MIN, opts).to_string(),
        i128::MIN.to_string()
    );
}

#[test]
fn formats_extreme_u128_with_significant_digits_without_invalid_artifacts() {
    let options = [
        NumberOptions::new().significant_digits(1),
        NumberOptions::new().significant_digits(2),
        NumberOptions::new().significant_digits(3),
        NumberOptions::new().significant_digits(6),
    ];

    for opts in options {
        let out = number_with(u128::MAX, opts).to_string();

        assert!(!out.is_empty());
        assert!(!out.contains("inf"));
        assert!(!out.contains("NaN"));
    }
}

#[test]
fn keeps_rounding_stable_near_decillion_boundary_for_large_values() {
    let out = number(999_950_000_000_000_000_000_000_000_000_000_u128).to_string();
    assert_eq!(out, "1Dc");
}

#[test]
fn significant_digits_round_extreme_signed_values_correctly() {
    let sig1 = NumberOptions::new().significant_digits(1);
    let sig2 = NumberOptions::new().significant_digits(2);

    assert_eq!(number_with(i128::MIN, sig1).to_string(), "-200Ud");
    assert_eq!(number_with(i128::MAX, sig1).to_string(), "200Ud");

    assert_eq!(number_with(i128::MIN, sig2).to_string(), "-170Ud");
    assert_eq!(number_with(i128::MAX, sig2).to_string(), "170Ud");
}

#[test]
fn significant_digits_round_extreme_unsigned_values_correctly() {
    let sig1 = NumberOptions::new().significant_digits(1);
    let sig2 = NumberOptions::new().significant_digits(2);
    let sig4 = NumberOptions::new().significant_digits(4);

    assert_eq!(number_with(u128::MAX, sig1).to_string(), "300Ud");
    assert_eq!(number_with(u128::MAX, sig2).to_string(), "340Ud");
    assert_eq!(number_with(u128::MAX, sig4).to_string(), "340.3Ud");
}

// --- Large finite floats ---

#[test]
fn formats_large_finite_f64_without_invalid_artifacts() {
    let out = number(f64::MAX).to_string();

    assert!(!out.is_empty());
    assert!(!out.contains("inf"));
    assert!(!out.contains("NaN"));
}

#[test]
fn formats_large_finite_f64_without_compaction() {
    let opts = NumberOptions::new().compact(false);
    let out = number_with(f64::MAX, opts).to_string();

    assert!(!out.is_empty());
    assert!(!out.contains("inf"));
    assert!(!out.contains("NaN"));
}

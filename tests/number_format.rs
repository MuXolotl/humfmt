use humfmt::RoundingMode;
use humfmt::{number, NumberOptions};

// --- Basic rendering ---

#[test]
fn formats_small_numbers_without_suffix() {
    assert_eq!(number(999).to_string(), "999");
    assert_eq!(number(42).to_string(), "42");
}

#[test]
fn formats_thousands() {
    assert_eq!(number(15320).to_string(), "15.3K");
    assert_eq!(number(1200).to_string(), "1.2K");
}

#[test]
fn formats_millions() {
    assert_eq!(number(1_500_000).to_string(), "1.5M");
}

#[test]
fn formats_negative_numbers() {
    assert_eq!(number(-12_500).to_string(), "-12.5K");
}

#[test]
fn supports_long_units() {
    let opts = NumberOptions::new().long_units();
    assert_eq!(
        humfmt::number_with(15320, opts).to_string(),
        "15.3 thousand"
    );
}

#[test]
fn supports_precision_override() {
    let opts = NumberOptions::new().precision(2);
    assert_eq!(humfmt::number_with(15320, opts).to_string(), "15.32K");
}

#[test]
fn supports_disabling_compact_scaling() {
    let opts = NumberOptions::new().compact(false);
    assert_eq!(humfmt::number_with(1_500_000, opts).to_string(), "1500000");
    assert_eq!(
        humfmt::number_with(1_500_000.5_f64, opts).to_string(),
        "1500000.5"
    );
}

#[test]
fn rescales_after_rounding_boundary() {
    assert_eq!(number(999_950).to_string(), "1M");
    assert_eq!(number(999_999).to_string(), "1M");
}

#[test]
fn supports_separator_rendering() {
    let opts = NumberOptions::new().separators(true).precision(2);
    assert_eq!(humfmt::number_with(123.45, opts).to_string(), "123.45");
}

#[test]
fn avoids_negative_zero_output() {
    assert_eq!(number(-0.0).to_string(), "0");
}

#[test]
fn avoids_negative_zero_after_rounding_small_floats() {
    assert_eq!(number(-0.04).to_string(), "0");
    assert_eq!(
        humfmt::number_with(-0.004, NumberOptions::new().precision(2)).to_string(),
        "0"
    );
}

#[test]
fn preserves_non_finite_values() {
    assert_eq!(number(f64::INFINITY).to_string(), "inf");
    assert_eq!(number(f64::NEG_INFINITY).to_string(), "-inf");
    assert_eq!(number(f64::NAN).to_string(), "NaN");
}

#[test]
fn supports_large_units_beyond_trillion() {
    assert_eq!(number(1_500_000_000_000_000_i128).to_string(), "1.5Qa");
    assert_eq!(number(1_000_000_000_000_000_000_i128).to_string(), "1Qi");
}

#[test]
fn supports_large_long_units_beyond_trillion() {
    let opts = NumberOptions::new().long_units();
    assert_eq!(
        humfmt::number_with(1_000_000_000_000_000_i128, opts).to_string(),
        "1 quadrillion"
    );
}

#[test]
fn formats_extreme_u128_without_overflow_artifacts() {
    let out = number(u128::MAX).to_string();
    assert!(out.ends_with("Dc"));
    assert!(!out.contains("inf"));
    assert!(!out.contains("NaN"));
}

#[test]
fn keeps_rounding_stable_near_suffix_boundary_for_large_values() {
    let out = number(999_950_000_000_000_000_000_000_000_000_000_u128).to_string();
    assert_eq!(out, "1Dc");
}

#[test]
fn fixed_precision_preserves_trailing_zeros_for_integers() {
    let opts = NumberOptions::new().precision(2).fixed_precision(true);
    assert_eq!(humfmt::number_with(1_500, opts).to_string(), "1.50K");
    assert_eq!(humfmt::number_with(1_000, opts).to_string(), "1.00K");
    assert_eq!(humfmt::number_with(1_540, opts).to_string(), "1.54K");
}

#[test]
fn fixed_precision_preserves_trailing_zeros_for_floats() {
    let opts = NumberOptions::new().precision(2).fixed_precision(true);
    assert_eq!(humfmt::number_with(1_500.0_f64, opts).to_string(), "1.50K");
    assert_eq!(humfmt::number_with(1_000.0_f64, opts).to_string(), "1.00K");
}

#[test]
fn fixed_precision_false_trims_zeros_by_default() {
    let opts = NumberOptions::new().precision(2);
    assert_eq!(humfmt::number_with(1_500, opts).to_string(), "1.5K");
    assert_eq!(humfmt::number_with(1_000, opts).to_string(), "1K");
}

#[test]
fn fixed_precision_with_zero_precision_emits_no_decimal() {
    let opts = NumberOptions::new().precision(0).fixed_precision(true);
    assert_eq!(humfmt::number_with(1_500, opts).to_string(), "2K");
    assert_eq!(humfmt::number_with(1_000, opts).to_string(), "1K");
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

// --- Edge cases ---

#[test]
fn formats_zero() {
    assert_eq!(number(0_i32).to_string(), "0");
    assert_eq!(number(0_u64).to_string(), "0");
    assert_eq!(number(0.0_f64).to_string(), "0");
    assert_eq!(number(-0.0_f64).to_string(), "0");
}

#[test]
fn formats_one() {
    assert_eq!(number(1_i32).to_string(), "1");
    assert_eq!(number(1_u64).to_string(), "1");
    assert_eq!(number(1.0_f64).to_string(), "1");
}

#[test]
fn formats_i128_min() {
    let out = number(i128::MIN).to_string();
    assert!(out.starts_with('-'));
    assert!(!out.contains("inf"));
    assert!(!out.contains("NaN"));
}

#[test]
fn formats_u128_max() {
    let out = number(u128::MAX).to_string();
    assert!(out.ends_with("Dc"));
    assert!(!out.contains("inf"));
    assert!(!out.contains("NaN"));
}

#[test]
fn formats_negative_one() {
    assert_eq!(number(-1_i32).to_string(), "-1");
    assert_eq!(number(-1_i128).to_string(), "-1");
    assert_eq!(number(-1.0_f64).to_string(), "-1");
}

#[test]
fn precision_zero_rounds_correctly() {
    let opts = NumberOptions::new().precision(0);
    assert_eq!(humfmt::number_with(1_400, opts).to_string(), "1K");
    assert_eq!(humfmt::number_with(1_500, opts).to_string(), "2K");
    assert_eq!(humfmt::number_with(1_499, opts).to_string(), "1K");
}

#[test]
fn precision_zero_on_float() {
    let opts = NumberOptions::new().precision(0);
    assert_eq!(humfmt::number_with(1_400.0_f64, opts).to_string(), "1K");
    assert_eq!(humfmt::number_with(1_500.0_f64, opts).to_string(), "2K");
}

#[test]
fn separators_apply_only_when_unscaled() {
    let opts = NumberOptions::new().separators(true);
    assert_eq!(humfmt::number_with(15_320, opts).to_string(), "15.3K");
    assert_eq!(humfmt::number_with(999, opts).to_string(), "999");
}

#[test]
fn separators_apply_when_scaling_disabled() {
    let opts = NumberOptions::new().compact(false).separators(true);
    assert_eq!(humfmt::number_with(12_345, opts).to_string(), "12,345");
    assert_eq!(
        humfmt::number_with(1_234_567, opts).to_string(),
        "1,234,567"
    );
}

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
fn float_just_below_rescale_boundary() {
    let out = humfmt::number_with(999_949.0_f64, NumberOptions::new().precision(1)).to_string();
    assert_eq!(out, "999.9K");
}

#[test]
fn float_at_exact_suffix_boundary() {
    assert_eq!(number(1_000.0_f64).to_string(), "1K");
    assert_eq!(number(1_000_000.0_f64).to_string(), "1M");
    assert_eq!(number(1_000_000_000.0_f64).to_string(), "1B");
}

#[test]
fn sign_symmetry_for_floats() {
    let pairs: &[f64] = &[0.5, 1.0, 1.5, 100.0, 1_000.0, 1_500.0, 1_000_000.0];
    for &v in pairs {
        let pos = number(v).to_string();
        let neg = number(-v).to_string();
        assert_eq!(neg, format!("-{pos}"), "sign symmetry failed for {v}");
    }
}

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
    assert_eq!(number(f32::INFINITY).to_string(), "inf");
    assert_eq!(number(f32::NAN).to_string(), "NaN");
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
fn fixed_precision_and_long_units_together() {
    let opts = NumberOptions::new()
        .precision(2)
        .fixed_precision(true)
        .long_units();
    assert_eq!(
        humfmt::number_with(1_000, opts).to_string(),
        "1.00 thousand"
    );
    assert_eq!(
        humfmt::number_with(1_500, opts).to_string(),
        "1.50 thousand"
    );
    assert_eq!(
        humfmt::number_with(1_540, opts).to_string(),
        "1.54 thousand"
    );
}

#[test]
fn separators_with_negative_unscaled_values() {
    let opts = NumberOptions::new().compact(false).separators(true);
    assert_eq!(humfmt::number_with(-12_345, opts).to_string(), "-12,345");
    assert_eq!(
        humfmt::number_with(-1_234_567, opts).to_string(),
        "-1,234,567"
    );
}

#[test]
fn precision_six_is_maximum() {
    let opts_6 = NumberOptions::new().precision(6);
    let opts_10 = NumberOptions::new().precision(10);
    assert_eq!(
        humfmt::number_with(1_234_567, opts_6).to_string(),
        humfmt::number_with(1_234_567, opts_10).to_string(),
    );
}

#[test]
fn float_precision_zero_at_rescale_boundary() {
    let opts = NumberOptions::new().precision(0);
    assert_eq!(humfmt::number_with(999_500.0_f64, opts).to_string(), "1M");
    assert_eq!(humfmt::number_with(999_499.0_f64, opts).to_string(), "999K");
}

// --- Custom separators (replaces locale tests) ---

#[test]
fn custom_decimal_separator() {
    let opts = NumberOptions::new().precision(2).decimal_separator(',');
    assert_eq!(humfmt::number_with(1.5_f64, opts).to_string(), "1,5");
    assert_eq!(humfmt::number_with(1_500, opts).to_string(), "1,5K");
}

#[test]
fn custom_decimal_separator_with_compact_value() {
    let opts = NumberOptions::new().decimal_separator(',');
    assert_eq!(humfmt::number_with(15_320, opts).to_string(), "15,3K");
}

#[test]
fn custom_group_separator_when_uncompacted() {
    let opts = NumberOptions::new()
        .compact(false)
        .separators(true)
        .group_separator(' ');
    assert_eq!(
        humfmt::number_with(1_234_567, opts).to_string(),
        "1 234 567"
    );
}

#[test]
fn combined_custom_separators() {
    let opts = NumberOptions::new()
        .compact(false)
        .separators(true)
        .decimal_separator(',')
        .group_separator(' ');
    // Float path with grouping + custom decimal separator.
    assert_eq!(
        humfmt::number_with(1_234_567.5_f64, opts).to_string(),
        "1 234 567,5"
    );
}

// --- Rounding modes ---

#[test]
fn rounding_modes_for_positive_integers() {
    let base = NumberOptions::new().precision(2);
    assert_eq!(
        humfmt::number_with(1234, base.rounding(RoundingMode::HalfUp)).to_string(),
        "1.23K"
    );
    assert_eq!(
        humfmt::number_with(1234, base.rounding(RoundingMode::Floor)).to_string(),
        "1.23K"
    );
    assert_eq!(
        humfmt::number_with(1234, base.rounding(RoundingMode::Ceil)).to_string(),
        "1.24K"
    );
}

#[test]
fn rounding_modes_for_negative_integers() {
    let base = NumberOptions::new().precision(2);
    assert_eq!(
        humfmt::number_with(-1234, base.rounding(RoundingMode::HalfUp)).to_string(),
        "-1.23K"
    );
    assert_eq!(
        humfmt::number_with(-1234, base.rounding(RoundingMode::Floor)).to_string(),
        "-1.24K"
    );
    assert_eq!(
        humfmt::number_with(-1234, base.rounding(RoundingMode::Ceil)).to_string(),
        "-1.23K"
    );
}

#[test]
fn rounding_modes_near_suffix_boundary() {
    let base = NumberOptions::new().precision(0);
    assert_eq!(
        humfmt::number_with(999_500, base.rounding(RoundingMode::HalfUp)).to_string(),
        "1M"
    );
    assert_eq!(
        humfmt::number_with(999_500, base.rounding(RoundingMode::Floor)).to_string(),
        "999K"
    );
    assert_eq!(
        humfmt::number_with(999_500, base.rounding(RoundingMode::Ceil)).to_string(),
        "1M"
    );

    assert_eq!(
        humfmt::number_with(-999_500, base.rounding(RoundingMode::HalfUp)).to_string(),
        "-1M"
    );
    assert_eq!(
        humfmt::number_with(-999_500, base.rounding(RoundingMode::Floor)).to_string(),
        "-1M"
    );
    assert_eq!(
        humfmt::number_with(-999_500, base.rounding(RoundingMode::Ceil)).to_string(),
        "-999K"
    );
}

#[test]
fn rounding_modes_for_floats() {
    let base = NumberOptions::new().precision(1);
    assert_eq!(
        humfmt::number_with(1.55_f64, base.rounding(RoundingMode::HalfUp)).to_string(),
        "1.6"
    );
    assert_eq!(
        humfmt::number_with(1.55_f64, base.rounding(RoundingMode::Floor)).to_string(),
        "1.5"
    );
    assert_eq!(
        humfmt::number_with(1.55_f64, base.rounding(RoundingMode::Ceil)).to_string(),
        "1.6"
    );

    assert_eq!(
        humfmt::number_with(-1.55_f64, base.rounding(RoundingMode::HalfUp)).to_string(),
        "-1.6"
    );
    assert_eq!(
        humfmt::number_with(-1.55_f64, base.rounding(RoundingMode::Floor)).to_string(),
        "-1.6"
    );
    assert_eq!(
        humfmt::number_with(-1.55_f64, base.rounding(RoundingMode::Ceil)).to_string(),
        "-1.5"
    );
}

#[test]
fn significant_digits_formatting() {
    let opts = NumberOptions::new().significant_digits(3);
    assert_eq!(humfmt::number_with(1234, opts).to_string(), "1.23K");
    assert_eq!(humfmt::number_with(12345, opts).to_string(), "12.3K");
    assert_eq!(humfmt::number_with(123456, opts).to_string(), "123K");
    assert_eq!(humfmt::number_with(1234567, opts).to_string(), "1.23M");

    let unscaled = NumberOptions::new().compact(false).significant_digits(2);
    assert_eq!(humfmt::number_with(1234, unscaled).to_string(), "1200");
    assert_eq!(humfmt::number_with(1250, unscaled).to_string(), "1300");

    assert_eq!(
        humfmt::number_with(0.01234_f64, unscaled).to_string(),
        "0.012"
    );
    assert_eq!(humfmt::number_with(123.45_f64, unscaled).to_string(), "120");

    assert_eq!(humfmt::number_with(0, opts).to_string(), "0");

    let fixed = NumberOptions::new()
        .significant_digits(3)
        .fixed_precision(true);
    assert_eq!(humfmt::number_with(1, fixed).to_string(), "1.00");
    assert_eq!(humfmt::number_with(10, fixed).to_string(), "10.0");
    assert_eq!(humfmt::number_with(100, fixed).to_string(), "100");
}

#[test]
fn force_sign_renders_plus_for_positive_numbers() {
    let opts = NumberOptions::new().force_sign(true);
    assert_eq!(humfmt::number_with(1500, opts).to_string(), "+1.5K");
    assert_eq!(humfmt::number_with(42, opts).to_string(), "+42");
    assert_eq!(humfmt::number_with(0, opts).to_string(), "0");
    assert_eq!(humfmt::number_with(-1500, opts).to_string(), "-1.5K");
}

#[test]
fn force_sign_with_floats_avoids_plus_zero() {
    let opts = NumberOptions::new().force_sign(true).precision(1);
    assert_eq!(humfmt::number_with(0.004_f64, opts).to_string(), "0");
    assert_eq!(humfmt::number_with(1.5_f64, opts).to_string(), "+1.5");
}

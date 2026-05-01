use humfmt::{number, NumberOptions};

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
    // i128::MIN = -170_141_183_460_469_231_731_687_303_715_884_105_728
    // Should render as a large negative compact number, no panic, no inf/NaN.
    let out = number(i128::MIN).to_string();
    assert!(out.starts_with('-'));
    assert!(!out.contains("inf"));
    assert!(!out.contains("NaN"));
}

#[test]
fn formats_u128_max() {
    // u128::MAX = 340_282_366_920_938_463_463_374_607_431_768_211_455
    // Should render as a compact number ending in "Dc", no panic, no inf/NaN.
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
    // Compacted: separators have no effect on the suffix part.
    assert_eq!(humfmt::number_with(15_320, opts).to_string(), "15.3K");
    // Unscaled (below 1000): no separator needed for 3 digits.
    assert_eq!(humfmt::number_with(999, opts).to_string(), "999");
}

#[test]
fn separators_apply_when_scaling_disabled() {
    use humfmt::locale::CustomLocale;
    let locale = CustomLocale::english().max_compact_suffix_index(0);
    let opts = NumberOptions::new().locale(locale).separators(true);
    assert_eq!(humfmt::number_with(12_345, opts).to_string(), "12,345");
    assert_eq!(
        humfmt::number_with(1_234_567, opts).to_string(),
        "1,234,567"
    );
}

#[test]
fn very_small_positive_float_below_threshold() {
    // Values below 1000 should render without suffix.
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
    // 999_949.x should stay as ~999.9K, not rescale to M.
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

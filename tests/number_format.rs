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

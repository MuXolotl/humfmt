use humfmt::{number, number_with, Humanize, NumberOptions, RoundingMode};

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
fn supports_extension_trait_usage() {
    assert_eq!(1_500_000.human_number().to_string(), "1.5M");
    assert_eq!((-12_500).human_number().to_string(), "-12.5K");
}

// --- Long units ---

#[test]
fn supports_long_units() {
    let opts = NumberOptions::new().long_units();

    assert_eq!(number_with(15320, opts).to_string(), "15.3 thousand");

    assert_eq!(number_with(1_000_000, opts).to_string(), "1 million");
}

#[test]
fn fixed_precision_and_long_units_together() {
    let opts = NumberOptions::new()
        .precision(2)
        .fixed_precision(true)
        .long_units();

    assert_eq!(number_with(1_000, opts).to_string(), "1.00 thousand");
    assert_eq!(number_with(1_500, opts).to_string(), "1.50 thousand");
    assert_eq!(number_with(1_540, opts).to_string(), "1.54 thousand");
}

// --- Precision ---

#[test]
fn supports_precision_override() {
    let opts = NumberOptions::new().precision(2);
    assert_eq!(number_with(15320, opts).to_string(), "15.32K");
}

#[test]
fn precision_zero_rounds_correctly() {
    let opts = NumberOptions::new().precision(0);

    assert_eq!(number_with(1_400, opts).to_string(), "1K");
    assert_eq!(number_with(1_500, opts).to_string(), "2K");
    assert_eq!(number_with(1_499, opts).to_string(), "1K");
}

#[test]
fn precision_zero_on_float() {
    let opts = NumberOptions::new().precision(0);

    assert_eq!(number_with(1_400.0_f64, opts).to_string(), "1K");
    assert_eq!(number_with(1_500.0_f64, opts).to_string(), "2K");
}

#[test]
fn precision_six_is_maximum() {
    let opts_6 = NumberOptions::new().precision(6);
    let opts_10 = NumberOptions::new().precision(10);

    assert_eq!(
        number_with(1_234_567, opts_6).to_string(),
        number_with(1_234_567, opts_10).to_string(),
    );
}

// --- Fixed precision ---

#[test]
fn fixed_precision_preserves_trailing_zeros_for_integers() {
    let opts = NumberOptions::new().precision(2).fixed_precision(true);

    assert_eq!(number_with(1_500, opts).to_string(), "1.50K");
    assert_eq!(number_with(1_000, opts).to_string(), "1.00K");
    assert_eq!(number_with(1_540, opts).to_string(), "1.54K");
}

#[test]
fn fixed_precision_preserves_trailing_zeros_for_floats() {
    let opts = NumberOptions::new().precision(2).fixed_precision(true);

    assert_eq!(number_with(1_500.0_f64, opts).to_string(), "1.50K");
    assert_eq!(number_with(1_000.0_f64, opts).to_string(), "1.00K");
}

#[test]
fn fixed_precision_false_trims_zeros_by_default() {
    let opts = NumberOptions::new().precision(2);

    assert_eq!(number_with(1_500, opts).to_string(), "1.5K");
    assert_eq!(number_with(1_000, opts).to_string(), "1K");
}

#[test]
fn fixed_precision_with_zero_precision_emits_no_decimal() {
    let opts = NumberOptions::new().precision(0).fixed_precision(true);

    assert_eq!(number_with(1_500, opts).to_string(), "2K");
    assert_eq!(number_with(1_000, opts).to_string(), "1K");
}

// --- Compact control ---

#[test]
fn supports_disabling_compact_scaling() {
    let opts = NumberOptions::new().compact(false);

    assert_eq!(number_with(1_500_000, opts).to_string(), "1500000");
    assert_eq!(number_with(1_500_000.5_f64, opts).to_string(), "1500000.5");
}

// --- Separators ---

#[test]
fn supports_separator_rendering_for_float_values() {
    let opts = NumberOptions::new().separators(true).precision(2);
    assert_eq!(number_with(123.45, opts).to_string(), "123.45");
}

#[test]
fn separators_apply_only_when_unscaled() {
    let opts = NumberOptions::new().separators(true);

    assert_eq!(number_with(15_320, opts).to_string(), "15.3K");
    assert_eq!(number_with(999, opts).to_string(), "999");
}

#[test]
fn separators_apply_when_scaling_disabled() {
    let opts = NumberOptions::new().compact(false).separators(true);

    assert_eq!(number_with(12_345, opts).to_string(), "12,345");
    assert_eq!(number_with(1_234_567, opts).to_string(), "1,234,567");
}

#[test]
fn separators_with_negative_unscaled_values() {
    let opts = NumberOptions::new().compact(false).separators(true);

    assert_eq!(number_with(-12_345, opts).to_string(), "-12,345");
    assert_eq!(number_with(-1_234_567, opts).to_string(), "-1,234,567");
}

#[test]
fn supports_custom_decimal_separator() {
    let opts = NumberOptions::new().precision(2).decimal_separator(',');

    assert_eq!(number_with(1.5_f64, opts).to_string(), "1,5");
    assert_eq!(number_with(1_500, opts).to_string(), "1,5K");
}

#[test]
fn supports_custom_decimal_separator_with_compact_value() {
    let opts = NumberOptions::new().decimal_separator(',');

    assert_eq!(number_with(15_320, opts).to_string(), "15,3K");
}

#[test]
fn supports_custom_group_separator_when_uncompacted() {
    let opts = NumberOptions::new()
        .compact(false)
        .separators(true)
        .group_separator(' ');

    assert_eq!(number_with(1_234_567, opts).to_string(), "1 234 567");
}

#[test]
fn supports_combined_custom_separators() {
    let opts = NumberOptions::new()
        .compact(false)
        .separators(true)
        .decimal_separator(',')
        .group_separator(' ');

    assert_eq!(
        number_with(1_234_567.5_f64, opts).to_string(),
        "1 234 567,5"
    );
}

// --- Rounding modes ---

#[test]
fn rounding_modes_for_positive_integers() {
    let base = NumberOptions::new().precision(2);

    assert_eq!(
        number_with(1234, base.rounding(RoundingMode::HalfUp)).to_string(),
        "1.23K"
    );
    assert_eq!(
        number_with(1234, base.rounding(RoundingMode::Floor)).to_string(),
        "1.23K"
    );
    assert_eq!(
        number_with(1234, base.rounding(RoundingMode::Ceil)).to_string(),
        "1.24K"
    );
}

#[test]
fn rounding_modes_for_negative_integers() {
    let base = NumberOptions::new().precision(2);

    assert_eq!(
        number_with(-1234, base.rounding(RoundingMode::HalfUp)).to_string(),
        "-1.23K"
    );
    assert_eq!(
        number_with(-1234, base.rounding(RoundingMode::Floor)).to_string(),
        "-1.24K"
    );
    assert_eq!(
        number_with(-1234, base.rounding(RoundingMode::Ceil)).to_string(),
        "-1.23K"
    );
}

#[test]
fn rounding_modes_for_floats() {
    let base = NumberOptions::new().precision(1);

    assert_eq!(
        number_with(1.55_f64, base.rounding(RoundingMode::HalfUp)).to_string(),
        "1.6"
    );
    assert_eq!(
        number_with(1.55_f64, base.rounding(RoundingMode::Floor)).to_string(),
        "1.5"
    );
    assert_eq!(
        number_with(1.55_f64, base.rounding(RoundingMode::Ceil)).to_string(),
        "1.6"
    );

    assert_eq!(
        number_with(-1.55_f64, base.rounding(RoundingMode::HalfUp)).to_string(),
        "-1.6"
    );
    assert_eq!(
        number_with(-1.55_f64, base.rounding(RoundingMode::Floor)).to_string(),
        "-1.6"
    );
    assert_eq!(
        number_with(-1.55_f64, base.rounding(RoundingMode::Ceil)).to_string(),
        "-1.5"
    );
}

#[test]
fn rounding_modes_near_suffix_boundary() {
    let base = NumberOptions::new().precision(0);

    assert_eq!(
        number_with(999_500, base.rounding(RoundingMode::HalfUp)).to_string(),
        "1M"
    );
    assert_eq!(
        number_with(999_500, base.rounding(RoundingMode::Floor)).to_string(),
        "999K"
    );
    assert_eq!(
        number_with(999_500, base.rounding(RoundingMode::Ceil)).to_string(),
        "1M"
    );

    assert_eq!(
        number_with(-999_500, base.rounding(RoundingMode::HalfUp)).to_string(),
        "-1M"
    );
    assert_eq!(
        number_with(-999_500, base.rounding(RoundingMode::Floor)).to_string(),
        "-1M"
    );
    assert_eq!(
        number_with(-999_500, base.rounding(RoundingMode::Ceil)).to_string(),
        "-999K"
    );
}

// --- Significant digits ---

#[test]
fn supports_significant_digits_for_compact_integers() {
    let opts = NumberOptions::new().significant_digits(3);

    assert_eq!(number_with(1234, opts).to_string(), "1.23K");
    assert_eq!(number_with(12345, opts).to_string(), "12.3K");
    assert_eq!(number_with(123456, opts).to_string(), "123K");
    assert_eq!(number_with(1234567, opts).to_string(), "1.23M");
}

#[test]
fn supports_significant_digits_for_unscaled_integers() {
    let opts = NumberOptions::new().compact(false).significant_digits(2);

    assert_eq!(number_with(1234, opts).to_string(), "1200");
    assert_eq!(number_with(1250, opts).to_string(), "1300");
}

#[test]
fn supports_significant_digits_for_unscaled_floats() {
    let opts = NumberOptions::new().compact(false).significant_digits(2);

    assert_eq!(number_with(0.01234_f64, opts).to_string(), "0.012");
    assert_eq!(number_with(123.45_f64, opts).to_string(), "120");
}

#[test]
fn significant_digits_zero_value_formats_as_zero() {
    let opts = NumberOptions::new().significant_digits(3);

    assert_eq!(number_with(0, opts).to_string(), "0");
}

#[test]
fn significant_digits_with_fixed_precision_pads_fractional_zeros() {
    let opts = NumberOptions::new()
        .significant_digits(3)
        .fixed_precision(true);

    assert_eq!(number_with(1, opts).to_string(), "1.00");
    assert_eq!(number_with(10, opts).to_string(), "10.0");
    assert_eq!(number_with(100, opts).to_string(), "100");
}

// --- Force sign ---

#[test]
fn force_sign_renders_plus_for_positive_numbers() {
    let opts = NumberOptions::new().force_sign(true);

    assert_eq!(number_with(1500, opts).to_string(), "+1.5K");
    assert_eq!(number_with(42, opts).to_string(), "+42");
    assert_eq!(number_with(0, opts).to_string(), "0");
    assert_eq!(number_with(-1500, opts).to_string(), "-1.5K");
}

#[test]
fn force_sign_with_floats_avoids_plus_zero() {
    let opts = NumberOptions::new().force_sign(true).precision(1);

    assert_eq!(number_with(0.004_f64, opts).to_string(), "0");
    assert_eq!(number_with(1.5_f64, opts).to_string(), "+1.5");
}

// --- Larger named suffixes ---

#[test]
fn supports_large_units_beyond_trillion() {
    assert_eq!(number(1_500_000_000_000_000_i128).to_string(), "1.5Qa");
    assert_eq!(number(1_000_000_000_000_000_000_i128).to_string(), "1Qi");
}

#[test]
fn supports_large_long_units_beyond_trillion() {
    let opts = NumberOptions::new().long_units();

    assert_eq!(
        number_with(1_000_000_000_000_000_i128, opts).to_string(),
        "1 quadrillion"
    );
}

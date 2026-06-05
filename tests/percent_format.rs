use humfmt::{percent, percent_with, Humanize, PercentOptions, RoundingMode};

// --- Basic rendering ---

#[test]
fn formats_common_ratios() {
    assert_eq!(percent(0.0_f64).to_string(), "0%");
    assert_eq!(percent(0.5_f64).to_string(), "50%");
    assert_eq!(percent(1.0_f64).to_string(), "100%");
}

#[test]
fn formats_fractional_ratio_with_default_precision() {
    assert_eq!(percent(0.423_f64).to_string(), "42.3%");
    assert_eq!(percent(0.4235_f64).to_string(), "42.4%");
}

#[test]
fn trims_trailing_zeros_by_default() {
    assert_eq!(percent(0.5_f64).to_string(), "50%");
    assert_eq!(percent(0.10_f64).to_string(), "10%");
    assert_eq!(percent(0.42_f64).to_string(), "42%");
}

// --- Precision ---

#[test]
fn supports_precision_override_for_f64() {
    let opts = PercentOptions::new().precision(2);
    assert_eq!(percent_with(0.4236_f64, opts).to_string(), "42.36%");
    assert_eq!(percent_with(0.5_f64, opts).to_string(), "50%");
}

#[test]
fn supports_precision_override_for_f32() {
    let opts = PercentOptions::new().precision(2);
    assert_eq!(percent_with(0.4236_f32, opts).to_string(), "42.36%");
    assert_eq!(percent_with(0.5_f32, opts).to_string(), "50%");
}

#[test]
fn precision_zero_rounds_to_integer() {
    let opts = PercentOptions::new().precision(0);
    assert_eq!(percent_with(0.424_f64, opts).to_string(), "42%");
    assert_eq!(percent_with(0.425_f64, opts).to_string(), "43%");
    assert_eq!(percent_with(0.426_f64, opts).to_string(), "43%");
}

// --- Fixed precision ---

#[test]
fn fixed_precision_preserves_trailing_zeros_for_f64() {
    let opts = PercentOptions::new().precision(2).fixed_precision(true);
    assert_eq!(percent_with(0.5_f64, opts).to_string(), "50.00%");
    assert_eq!(percent_with(0.425_f64, opts).to_string(), "42.50%");
    assert_eq!(percent_with(0.4236_f64, opts).to_string(), "42.36%");
}

#[test]
fn fixed_precision_preserves_trailing_zeros_for_f32() {
    let opts = PercentOptions::new().precision(2).fixed_precision(true);
    assert_eq!(percent_with(0.5_f32, opts).to_string(), "50.00%");
    assert_eq!(percent_with(0.425_f32, opts).to_string(), "42.50%");
    assert_eq!(percent_with(0.4236_f32, opts).to_string(), "42.36%");
}

#[test]
fn fixed_precision_with_zero_precision_emits_no_decimal() {
    let opts = PercentOptions::new().precision(0).fixed_precision(true);
    assert_eq!(percent_with(0.424_f64, opts).to_string(), "42%");
    assert_eq!(percent_with(0.5_f64, opts).to_string(), "50%");
}

// --- Edge cases ---

#[test]
fn formats_zero() {
    assert_eq!(percent(0.0_f64).to_string(), "0%");
    assert_eq!(percent(-0.0_f64).to_string(), "0%");
}

#[test]
fn formats_one_hundred_percent() {
    assert_eq!(percent(1.0_f64).to_string(), "100%");
}

#[test]
fn formats_values_above_one() {
    assert_eq!(percent(1.5_f64).to_string(), "150%");
    assert_eq!(percent(2.0_f64).to_string(), "200%");
}

#[test]
fn formats_negative_ratios() {
    assert_eq!(percent(-0.5_f64).to_string(), "-50%");
    assert_eq!(percent(-0.423_f64).to_string(), "-42.3%");
}

#[test]
fn negative_value_rounding_to_zero_suppresses_minus() {
    assert_eq!(percent(-0.0004_f64).to_string(), "0%");
}

#[test]
fn positive_value_rounding_to_zero_with_force_sign_suppresses_plus() {
    let opts = PercentOptions::new().force_sign(true);
    assert_eq!(percent_with(0.0004_f64, opts).to_string(), "0%");
}

#[test]
fn preserves_non_finite_values() {
    assert_eq!(percent(f64::INFINITY).to_string(), "inf%");
    assert_eq!(percent(f64::NEG_INFINITY).to_string(), "-inf%");
    assert_eq!(percent(f64::NAN).to_string(), "NaN%");
}

#[test]
fn supports_f32_input() {
    assert_eq!(percent(0.5_f32).to_string(), "50%");
    assert_eq!(percent(1.0_f32).to_string(), "100%");
}

// --- Custom decimal separator ---

#[test]
fn supports_custom_decimal_separator() {
    let opts = PercentOptions::new().precision(1).decimal_separator(',');
    assert_eq!(percent_with(0.423_f64, opts).to_string(), "42,3%");
}

// --- Extension trait ---

#[test]
fn supports_extension_trait_usage() {
    assert_eq!(0.423_f64.human_percent().to_string(), "42.3%");
    assert_eq!(1.0_f64.human_percent().to_string(), "100%");
}

#[test]
fn supports_extension_trait_with_options() {
    let opts = PercentOptions::new().precision(2).fixed_precision(true);
    assert_eq!(0.5_f64.human_percent_with(opts).to_string(), "50.00%");
}

// --- Rounding ---

#[test]
fn half_up_rounding() {
    let opts = PercentOptions::new().precision(1);
    assert_eq!(percent_with(0.4250_f64, opts).to_string(), "42.5%");
    assert_eq!(percent_with(0.4255_f64, opts).to_string(), "42.6%");
    assert_eq!(percent_with(0.4244_f64, opts).to_string(), "42.4%");
}

#[test]
fn floor_rounding() {
    let opts = PercentOptions::new()
        .precision(0)
        .rounding(RoundingMode::Floor);
    // 42.9% with floor -> 42%
    assert_eq!(percent_with(0.429_f64, opts).to_string(), "42%");
    // 42.1% with floor -> 42%
    assert_eq!(percent_with(0.421_f64, opts).to_string(), "42%");
    // -42.1% with floor -> -43% (towards negative infinity)
    assert_eq!(percent_with(-0.421_f64, opts).to_string(), "-43%");
}

#[test]
fn ceil_rounding() {
    let opts = PercentOptions::new()
        .precision(0)
        .rounding(RoundingMode::Ceil);
    // 42.1% with ceil -> 43%
    assert_eq!(percent_with(0.421_f64, opts).to_string(), "43%");
    // 42.9% with ceil -> 43%
    assert_eq!(percent_with(0.429_f64, opts).to_string(), "43%");
    // -42.9% with ceil -> -42% (towards positive infinity)
    assert_eq!(percent_with(-0.429_f64, opts).to_string(), "-42%");
}

#[test]
fn rounding_mode_does_not_affect_exact_values() {
    for mode in [
        RoundingMode::HalfUp,
        RoundingMode::Floor,
        RoundingMode::Ceil,
    ] {
        let opts = PercentOptions::new().precision(1).rounding(mode);
        assert_eq!(
            percent_with(0.5_f64, opts).to_string(),
            "50%",
            "mode {mode:?} should not affect 0.5 at precision 1"
        );
    }
}

#[test]
fn sign_symmetry() {
    let test_values: &[f64] = &[0.1, 0.25, 0.5, 0.75, 1.0, 1.5];
    for &v in test_values {
        let pos = percent(v).to_string();
        let neg = percent(-v).to_string();
        assert_eq!(neg, format!("-{pos}"), "sign symmetry failed for {v}");
    }
}

#[test]
fn force_sign_renders_plus_for_positive_percents() {
    let opts = PercentOptions::new().force_sign(true);
    assert_eq!(percent_with(0.42_f64, opts).to_string(), "+42%");
    assert_eq!(percent_with(0.0_f64, opts).to_string(), "0%");
    assert_eq!(percent_with(-0.42_f64, opts).to_string(), "-42%");
}

#[test]
fn force_sign_avoids_plus_zero_when_rounding() {
    let opts = PercentOptions::new().force_sign(true).precision(1);
    assert_eq!(percent_with(0.0004_f64, opts).to_string(), "0%");
}

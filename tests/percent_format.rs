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

    // 0.4255_f64 * 100.0 is 42.54999... which rounds down to 42.5.
    // 0.4256_f64 * 100.0 is 42.55999... which rounds up to 42.6.
    assert_eq!(percent_with(0.4255_f64, opts).to_string(), "42.5%");
    assert_eq!(percent_with(0.4256_f64, opts).to_string(), "42.6%");
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

// --- Ratios above the u128 percentage range ---

#[test]
fn formats_ratios_above_the_u128_percent_range() {
    // `1e37` is exactly 9999999999999999538762658202121142272 as f64, and the
    // `* 100` step only appends two zeros to that expansion.
    assert_eq!(
        percent(1e37_f64).to_string(),
        "999999999999999953876265820212114227200%"
    );
    assert_eq!(
        percent(-1e37_f64).to_string(),
        "-999999999999999953876265820212114227200%"
    );
    assert_eq!(
        percent(3.5e36_f64).to_string(),
        "349999999999999977953734933487183462400%"
    );
}

#[test]
fn largest_ratios_are_not_saturated() {
    let out = percent(f64::MAX).to_string();

    // f64::MAX has 309 integer digits, `* 100` appends two zeros, then '%'.
    assert_eq!(out.len(), 312);
    assert!(out.starts_with("1797693134862315708"));
    assert!(out.ends_with("40402618412485836800%"));
    assert!(!out.contains("340282366920938463463374607431768211455"));

    assert!(percent(f64::MIN).to_string().starts_with('-'));
}

#[test]
fn unbounded_ratios_ignore_the_rounding_mode() {
    // There are no fractional digits to round away at this magnitude.
    let expected = percent(f64::MAX).to_string();

    for mode in [
        RoundingMode::HalfUp,
        RoundingMode::Floor,
        RoundingMode::Ceil,
    ] {
        let opts = PercentOptions::new().rounding(mode);
        assert_eq!(percent_with(f64::MAX, opts).to_string(), expected);
    }
}

#[test]
fn unbounded_ratios_keep_sign_and_fixed_precision() {
    let fixed = PercentOptions::new().precision(2).fixed_precision(true);
    assert!(percent_with(f64::MAX, fixed).to_string().ends_with(".00%"));

    let signed = PercentOptions::new().force_sign(true);
    assert!(percent_with(f64::MAX, signed)
        .to_string()
        .starts_with("+17"));
}

#[test]
fn rounds_ratios_at_the_limits_of_f64_precision() {
    // The scaled value reaches 2^52, where `f64` has no fractional digits left;
    // the direction still has to come from the first dropped decimal digit.
    assert_eq!(
        percent(-8_599_595_309_949.889_f64).to_string(),
        "-859959530994988.9%"
    );
    assert_eq!(
        percent(8_494_863_370_152.222_f64).to_string(),
        "849486337015222.1%"
    );
}

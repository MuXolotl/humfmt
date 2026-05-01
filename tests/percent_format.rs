use humfmt::{percent, percent_with, Humanize, PercentOptions};

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
fn supports_precision_override() {
    let opts = PercentOptions::new().precision(2);
    assert_eq!(percent_with(0.4236_f64, opts).to_string(), "42.36%");
    assert_eq!(percent_with(0.5_f64, opts).to_string(), "50%");
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
fn fixed_precision_preserves_trailing_zeros() {
    let opts = PercentOptions::new().precision(2).fixed_precision(true);
    assert_eq!(percent_with(0.5_f64, opts).to_string(), "50.00%");
    assert_eq!(percent_with(0.425_f64, opts).to_string(), "42.50%");
    assert_eq!(percent_with(0.4236_f64, opts).to_string(), "42.36%");
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
    // -0.0004 * 100 = -0.04, rounds to 0 at precision=1 — should not show "-0%".
    assert_eq!(percent(-0.0004_f64).to_string(), "0%");
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

// --- Locale ---

#[test]
fn supports_locale_decimal_separator() {
    let locale = humfmt::locale::CustomLocale::english().decimal_separator(',');
    let opts = PercentOptions::new().precision(1).locale(locale);
    assert_eq!(percent_with(0.423_f64, opts).to_string(), "42,3%");
}

#[cfg(feature = "russian")]
#[test]
fn supports_russian_decimal_separator() {
    let opts = PercentOptions::new()
        .precision(1)
        .locale(humfmt::locale::Russian);
    assert_eq!(percent_with(0.423_f64, opts).to_string(), "42,3%");
}

#[cfg(feature = "polish")]
#[test]
fn supports_polish_decimal_separator() {
    let opts = PercentOptions::new()
        .precision(1)
        .locale(humfmt::locale::Polish);
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
    // 0.4250 * 100 = 42.50, rounds to 42.5 (exact).
    assert_eq!(percent_with(0.4250_f64, opts).to_string(), "42.5%");
    // 0.4255 * 100 = 42.55, rounds up to 42.6.
    assert_eq!(percent_with(0.4255_f64, opts).to_string(), "42.6%");
    // 0.4244 * 100 = 42.44, rounds down to 42.4.
    assert_eq!(percent_with(0.4244_f64, opts).to_string(), "42.4%");
}

#[test]
fn sign_symmetry() {
    let pairs: &[f64] = &[0.1, 0.25, 0.5, 0.75, 1.0, 1.5];
    for &v in pairs {
        let pos = percent(v).to_string();
        let neg = percent(-v).to_string();
        assert_eq!(neg, format!("-{pos}"), "sign symmetry failed for {v}");
    }
}

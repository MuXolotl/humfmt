//! Golden (snapshot) tests for humfmt.
//!
//! These tests act as a strict regression net. They store exact inputs and their
//! expected string outputs across all formatters. If a future refactoring or
//! bug fix changes the output by even a single character, these tests will fail,
//! preventing silent formatting regressions.

use core::time::Duration;

use humfmt::{
    ago_with, bytes_with, duration_with, list_with, number_with, percent_with, ByteUnit,
    BytesOptions, DurationOptions, ListOptions, NumberOptions, PercentOptions, RoundingMode,
};

#[test]
fn golden_numbers() {
    let cases_i128 = [
        // (Input, Options, Expected Output)
        (0_i128, NumberOptions::new(), "0"),
        (1_i128, NumberOptions::new(), "1"),
        (-1_i128, NumberOptions::new(), "-1"),
        (999_i128, NumberOptions::new(), "999"),
        (1_000_i128, NumberOptions::new(), "1K"),
        (1_500_i128, NumberOptions::new(), "1.5K"),
        (-1_500_i128, NumberOptions::new(), "-1.5K"),
        (999_950_i128, NumberOptions::new(), "1M"),
        (1_234_567_i128, NumberOptions::new(), "1.2M"),
        (1_234_567_i128, NumberOptions::new().precision(2), "1.23M"),
        (
            1_234_567_i128,
            NumberOptions::new().significant_digits(3),
            "1.23M",
        ),
        (
            1_500_i128,
            NumberOptions::new().long_units(),
            "1.5 thousand",
        ),
        (
            1_234_567_i128,
            NumberOptions::new().compact(false).separators(true),
            "1,234,567",
        ),
        (
            1_234_567_i128,
            NumberOptions::new()
                .compact(false)
                .separators(true)
                .decimal_separator(',')
                .group_separator(' '),
            "1 234 567",
        ),
        (1_500_i128, NumberOptions::new().force_sign(true), "+1.5K"),
        (
            1_500_i128,
            NumberOptions::new().precision(2).fixed_precision(true),
            "1.50K",
        ),
        (
            1_900_i128,
            NumberOptions::new()
                .precision(0)
                .rounding(RoundingMode::Floor),
            "1K",
        ),
        // Extreme values
        (i128::MIN, NumberOptions::new(), "-170.1Ud"),
        (i128::MAX, NumberOptions::new(), "170.1Ud"),
    ];

    for (input, opts, expected) in cases_i128 {
        assert_eq!(
            number_with(input, opts).to_string(),
            expected,
            "Golden mismatch for number: {input} with {opts:?}"
        );
    }

    // Unsigned extreme value handled separately to avoid sign-casting overflow issues
    assert_eq!(
        number_with(u128::MAX, NumberOptions::new()).to_string(),
        "340.3Ud",
        "Golden mismatch for u128::MAX"
    );
}

#[test]
fn golden_floats() {
    let cases = [
        (0.0_f64, NumberOptions::new(), "0"),
        (0.42_f64, NumberOptions::new(), "0.4"),
        (1.5_f64, NumberOptions::new(), "1.5"),
        (-1.5_f64, NumberOptions::new(), "-1.5"),
        (999.9_f64, NumberOptions::new(), "999.9"),
        (1_000.0_f64, NumberOptions::new(), "1K"),
        (1_234_567.89_f64, NumberOptions::new(), "1.2M"),
        (
            1_234_567.89_f64,
            NumberOptions::new().precision(3),
            "1.235M",
        ),
        (
            0.004_f64,
            NumberOptions::new().precision(2).force_sign(true),
            "0", // Rounds to zero, suppresses sign
        ),
        (f64::INFINITY, NumberOptions::new(), "inf"),
        (f64::NEG_INFINITY, NumberOptions::new(), "-inf"),
        (f64::NAN, NumberOptions::new(), "NaN"),
    ];

    for (input, opts, expected) in cases {
        assert_eq!(
            number_with(input, opts).to_string(),
            expected,
            "Golden mismatch for float: {input} with {opts:?}"
        );
    }
}

#[test]
fn golden_bytes() {
    let cases = [
        (0_u64, BytesOptions::new(), "0B"),
        (512_u64, BytesOptions::new(), "512B"),
        (1_000_u64, BytesOptions::new(), "1KB"),
        (1_024_u64, BytesOptions::new(), "1KB"),
        (1_536_u64, BytesOptions::new(), "1.5KB"),
        (1_024_u64, BytesOptions::new().binary(), "1KiB"),
        (1_536_u64, BytesOptions::new().binary(), "1.5KiB"),
        (
            1_536_u64,
            BytesOptions::new().binary().precision(2).space(true),
            "1.5 KiB",
        ),
        (
            1_536_u64,
            BytesOptions::new()
                .binary()
                .precision(2)
                .space(true)
                .fixed_precision(true),
            "1.50 KiB",
        ),
        (1_000_u64, BytesOptions::new().bits(true), "8Kb"),
        (1_536_u64, BytesOptions::new().long_units(), "1.5 kilobytes"),
        (
            1_024_u64,
            BytesOptions::new().binary().long_units(),
            "1 kibibyte",
        ),
        (
            500_u64,
            BytesOptions::new().min_unit(ByteUnit::KB).precision(2),
            "0.5KB",
        ),
        (
            2_000_000_000_000_u64,
            BytesOptions::new().max_unit(ByteUnit::GB),
            "2000GB",
        ),
    ];

    for (input, opts, expected) in cases {
        assert_eq!(
            bytes_with(input, opts).to_string(),
            expected,
            "Golden mismatch for bytes: {input} with {opts:?}"
        );
    }
}

#[test]
fn golden_percent() {
    let cases = [
        (0.0_f64, PercentOptions::new(), "0%"),
        (0.5_f64, PercentOptions::new(), "50%"),
        (1.0_f64, PercentOptions::new(), "100%"),
        (1.5_f64, PercentOptions::new(), "150%"),
        (-0.5_f64, PercentOptions::new(), "-50%"),
        (0.423_f64, PercentOptions::new(), "42.3%"),
        (0.4236_f64, PercentOptions::new().precision(2), "42.36%"),
        (
            0.5_f64,
            PercentOptions::new().precision(2).fixed_precision(true),
            "50.00%",
        ),
        (0.15_f64, PercentOptions::new().force_sign(true), "+15%"),
        (
            -0.0004_f64,
            PercentOptions::new(),
            "0%", // Rounds to zero, suppresses minus
        ),
        (f64::INFINITY, PercentOptions::new(), "inf%"),
        (f64::NAN, PercentOptions::new(), "NaN%"),
    ];

    for (input, opts, expected) in cases {
        assert_eq!(
            percent_with(input, opts).to_string(),
            expected,
            "Golden mismatch for percent: {input} with {opts:?}"
        );
    }
}

#[test]
fn golden_duration_and_ago() {
    let cases = [
        (Duration::ZERO, DurationOptions::new(), "0s", "0s ago"),
        (
            Duration::from_millis(900),
            DurationOptions::new(),
            "900ms",
            "900ms ago",
        ),
        (
            Duration::from_millis(1_500),
            DurationOptions::new(),
            "1s 500ms",
            "1s 500ms ago",
        ),
        (
            Duration::from_secs(90),
            DurationOptions::new(),
            "1m 30s",
            "1m 30s ago",
        ),
        (
            Duration::from_secs(3661),
            DurationOptions::new(),
            "1h 1m",
            "1h 1m ago", // Truncates seconds due to default max_units(2)
        ),
        (
            Duration::from_secs(3665),
            DurationOptions::new().max_units(3),
            "1h 1m 5s",
            "1h 1m 5s ago",
        ),
        (
            Duration::from_secs(3665),
            DurationOptions::new().long_units().max_units(3),
            "1 hour 1 minute 5 seconds",
            "1 hour 1 minute 5 seconds ago",
        ),
    ];

    for (input, opts, expected_dur, expected_ago) in cases {
        assert_eq!(
            duration_with(input, opts).to_string(),
            expected_dur,
            "Golden mismatch for duration: {input:?} with {opts:?}"
        );
        assert_eq!(
            ago_with(input, opts).to_string(),
            expected_ago,
            "Golden mismatch for ago: {input:?} with {opts:?}"
        );
    }
}

#[test]
fn golden_ordinals() {
    let cases = [
        (0, "0th"),
        (1, "1st"),
        (2, "2nd"),
        (3, "3rd"),
        (4, "4th"),
        (11, "11th"),
        (12, "12th"),
        (13, "13th"),
        (21, "21st"),
        (42, "42nd"),
        (103, "103rd"),
        (111, "111th"),
    ];

    for (input, expected) in cases {
        assert_eq!(
            humfmt::ordinal(input).to_string(),
            expected,
            "Golden mismatch for ordinal: {input}"
        );
    }

    // Explicitly test negative
    assert_eq!(humfmt::ordinal(-1).to_string(), "-1st");
}

#[test]
fn golden_lists() {
    // Cannot easily map arrays of different lengths in a single typed loop,
    // so we evaluate them directly.

    let empty: &[&str] = &[];
    assert_eq!(list_with(empty, ListOptions::new()).to_string(), "");

    assert_eq!(list_with(&["red"], ListOptions::new()).to_string(), "red");

    assert_eq!(
        list_with(&["red", "green"], ListOptions::new()).to_string(),
        "red and green"
    );

    assert_eq!(
        list_with(&["red", "green", "blue"], ListOptions::new()).to_string(),
        "red, green, and blue"
    );

    assert_eq!(
        list_with(
            &["red", "green", "blue"],
            ListOptions::new().no_serial_comma()
        )
        .to_string(),
        "red, green and blue"
    );

    assert_eq!(
        list_with(
            &["red", "green", "blue"],
            ListOptions::new().conjunction("plus").no_serial_comma()
        )
        .to_string(),
        "red, green plus blue"
    );

    assert_eq!(
        list_with(
            &["red", "green", "blue"],
            ListOptions::new().separator(" | ").conjunction("&")
        )
        .to_string(),
        "red | green & blue"
    );
}

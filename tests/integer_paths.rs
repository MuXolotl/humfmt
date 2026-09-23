//! Values on both sides of the `u64` fast path in the integer formatters.
//!
//! Magnitudes that fit in a `u64` are formatted with 64-bit arithmetic, wider
//! ones fall back to 128-bit arithmetic. These cases pin the handover point, so
//! the two paths cannot drift apart unnoticed.

use humfmt::{
    bytes, bytes_with, number, number_with, ordinal, BytesOptions, NumberOptions, RoundingMode,
};

#[test]
fn formats_compact_numbers_across_the_u64_boundary() {
    let cases: [(u128, &str); 8] = [
        (999_999_999_999_999_999, "1Qi"),
        (1_000_000_000_000_000_000, "1Qi"),
        (9_999_999_999_999_999_999, "10Qi"),
        (10_000_000_000_000_000_000, "10Qi"),
        (18_446_744_073_709_551_614, "18.4Qi"),
        (18_446_744_073_709_551_615, "18.4Qi"),
        (18_446_744_073_709_551_616, "18.4Qi"),
        (55_340_232_221_128_654_845, "55.3Qi"),
    ];

    for (value, expected) in cases {
        assert_eq!(number(value).to_string(), expected, "value {value}");
    }
}

#[test]
fn formats_fixed_precision_across_the_u64_boundary() {
    let options = NumberOptions::new().precision(3);
    let cases: [(u128, &str); 8] = [
        (999_999_999_999_999_999, "1Qi"),
        (1_000_000_000_000_000_000, "1Qi"),
        (9_999_999_999_999_999_999, "10Qi"),
        (10_000_000_000_000_000_000, "10Qi"),
        (18_446_744_073_709_551_614, "18.447Qi"),
        (18_446_744_073_709_551_615, "18.447Qi"),
        (18_446_744_073_709_551_616, "18.447Qi"),
        (55_340_232_221_128_654_845, "55.34Qi"),
    ];

    for (value, expected) in cases {
        assert_eq!(
            number_with(value, options).to_string(),
            expected,
            "value {value}"
        );
    }
}

#[test]
fn rounds_significant_digits_across_the_u64_boundary() {
    let options = NumberOptions::new()
        .significant_digits(1)
        .rounding(RoundingMode::Ceil);
    let cases: [(u128, &str); 8] = [
        (999_999_999_999_999_999, "1Qi"),
        (1_000_000_000_000_000_000, "1Qi"),
        (9_999_999_999_999_999_999, "10Qi"),
        (10_000_000_000_000_000_000, "10Qi"),
        (18_446_744_073_709_551_614, "20Qi"),
        (18_446_744_073_709_551_615, "20Qi"),
        (18_446_744_073_709_551_616, "20Qi"),
        (55_340_232_221_128_654_845, "60Qi"),
    ];

    for (value, expected) in cases {
        assert_eq!(
            number_with(value, options).to_string(),
            expected,
            "value {value}"
        );
    }
}

#[test]
fn groups_uncompacted_digits_across_the_u64_boundary() {
    let options = NumberOptions::new().compact(false).separators(true);
    let cases: [(u128, &str); 8] = [
        (999_999_999_999_999_999, "999,999,999,999,999,999"),
        (1_000_000_000_000_000_000, "1,000,000,000,000,000,000"),
        (9_999_999_999_999_999_999, "9,999,999,999,999,999,999"),
        (10_000_000_000_000_000_000, "10,000,000,000,000,000,000"),
        (18_446_744_073_709_551_614, "18,446,744,073,709,551,614"),
        (18_446_744_073_709_551_615, "18,446,744,073,709,551,615"),
        (18_446_744_073_709_551_616, "18,446,744,073,709,551,616"),
        (55_340_232_221_128_654_845, "55,340,232,221,128,654,845"),
    ];

    for (value, expected) in cases {
        assert_eq!(
            number_with(value, options).to_string(),
            expected,
            "value {value}"
        );
    }
}

#[test]
fn formats_byte_sizes_across_the_u64_boundary() {
    let cases: [(u128, &str); 8] = [
        (999_999_999_999_999_999, "1EB"),
        (1_000_000_000_000_000_000, "1EB"),
        (9_999_999_999_999_999_999, "10EB"),
        (10_000_000_000_000_000_000, "10EB"),
        (18_446_744_073_709_551_614, "18.4EB"),
        (18_446_744_073_709_551_615, "18.4EB"),
        (18_446_744_073_709_551_616, "18.4EB"),
        (55_340_232_221_128_654_845, "55.3EB"),
    ];

    for (value, expected) in cases {
        assert_eq!(bytes(value).to_string(), expected, "value {value}");
    }
}

#[test]
fn formats_binary_byte_sizes_across_the_u64_boundary() {
    let options = BytesOptions::new().binary();
    let cases: [(u128, &str); 8] = [
        (999_999_999_999_999_999, "888.2PiB"),
        (1_000_000_000_000_000_000, "888.2PiB"),
        (9_999_999_999_999_999_999, "8.7EiB"),
        (10_000_000_000_000_000_000, "8.7EiB"),
        (18_446_744_073_709_551_614, "16EiB"),
        (18_446_744_073_709_551_615, "16EiB"),
        (18_446_744_073_709_551_616, "16EiB"),
        (55_340_232_221_128_654_845, "48EiB"),
    ];

    for (value, expected) in cases {
        assert_eq!(
            bytes_with(value, options).to_string(),
            expected,
            "value {value}"
        );
    }
}

#[test]
fn formats_ordinals_across_the_u64_boundary() {
    let cases: [(u128, &str); 8] = [
        (999_999_999_999_999_999, "999999999999999999th"),
        (1_000_000_000_000_000_000, "1000000000000000000th"),
        (9_999_999_999_999_999_999, "9999999999999999999th"),
        (10_000_000_000_000_000_000, "10000000000000000000th"),
        (18_446_744_073_709_551_614, "18446744073709551614th"),
        (18_446_744_073_709_551_615, "18446744073709551615th"),
        (18_446_744_073_709_551_616, "18446744073709551616th"),
        (55_340_232_221_128_654_845, "55340232221128654845th"),
    ];

    for (value, expected) in cases {
        assert_eq!(ordinal(value).to_string(), expected, "value {value}");
    }
}

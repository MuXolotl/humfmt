use humfmt::ByteUnit;
use humfmt::RoundingMode;
use humfmt::{bytes, BytesOptions, Humanize};

#[test]
fn formats_decimal_bytes_by_default() {
    assert_eq!(bytes(999).to_string(), "999B");
    assert_eq!(bytes(1536).to_string(), "1.5KB");
    assert_eq!(bytes(1_500_000).to_string(), "1.5MB");
}

#[test]
fn supports_binary_units() {
    let opts = BytesOptions::new().binary();
    assert_eq!(humfmt::bytes_with(1024, opts).to_string(), "1KiB");
    assert_eq!(humfmt::bytes_with(1536, opts).to_string(), "1.5KiB");
}

#[test]
fn supports_long_units() {
    let opts = BytesOptions::new().long_units();
    assert_eq!(humfmt::bytes_with(1, opts).to_string(), "1 byte");
    assert_eq!(humfmt::bytes_with(1536, opts).to_string(), "1.5 kilobytes");
}

#[test]
fn supports_binary_long_units() {
    let opts = BytesOptions::new().binary().long_units();
    assert_eq!(humfmt::bytes_with(1024, opts).to_string(), "1 kibibyte");
}

#[test]
fn supports_precision_override() {
    let opts = BytesOptions::new().precision(2);
    // 1536 / 1000 = 1.536, rounds to 1.54 at precision=2
    assert_eq!(humfmt::bytes_with(1536, opts).to_string(), "1.54KB");
}

#[test]
fn supports_custom_decimal_separator() {
    let opts = BytesOptions::new().decimal_separator(',');
    assert_eq!(humfmt::bytes_with(1536, opts).to_string(), "1,5KB");
}

#[test]
fn supports_locale_decimal_separator_via_custom_locale() {
    let locale = humfmt::locale::CustomLocale::english().decimal_separator(',');
    let opts = BytesOptions::new().locale(locale);
    assert_eq!(humfmt::bytes_with(1536, opts).to_string(), "1,5KB");
}

#[cfg(feature = "russian")]
#[test]
fn supports_locale_decimal_separator_russian() {
    let opts = BytesOptions::new().locale(humfmt::locale::Russian);
    assert_eq!(humfmt::bytes_with(1536, opts).to_string(), "1,5KB");
}

#[cfg(feature = "polish")]
#[test]
fn supports_locale_decimal_separator_polish() {
    let opts = BytesOptions::new().locale(humfmt::locale::Polish);
    assert_eq!(humfmt::bytes_with(1536, opts).to_string(), "1,5KB");
}

#[test]
fn supports_optional_space_before_short_units() {
    let opts = BytesOptions::new().space(true);
    assert_eq!(humfmt::bytes_with(999_u64, opts).to_string(), "999 B");
    assert_eq!(humfmt::bytes_with(1536_u64, opts).to_string(), "1.5 KB");

    let bin = BytesOptions::new().binary().precision(2).space(true);
    // 1536 / 1024 = 1.5, no trailing zeros to trim
    assert_eq!(humfmt::bytes_with(1536_u64, bin).to_string(), "1.5 KiB");
}

#[test]
fn supports_negative_values() {
    assert_eq!(bytes(-1536).to_string(), "-1.5KB");
}

#[test]
fn supports_extension_trait_usage() {
    assert_eq!(1536_u64.human_bytes().to_string(), "1.5KB");
}

#[test]
fn formats_extreme_u128_in_decimal_mode() {
    let out = bytes(u128::MAX).to_string();
    assert!(out.ends_with("EB"));
    assert!(!out.contains("inf"));
    assert!(!out.contains("NaN"));
}

#[test]
fn formats_extreme_u128_in_binary_mode() {
    let opts = BytesOptions::new().binary();
    let out = humfmt::bytes_with(u128::MAX, opts).to_string();
    assert!(out.ends_with("EiB"));
    assert!(!out.contains("inf"));
    assert!(!out.contains("NaN"));
}

#[test]
fn rounds_up_across_decimal_unit_boundary() {
    assert_eq!(bytes(999_950).to_string(), "1MB");
}

#[test]
fn fixed_precision_preserves_trailing_zeros() {
    let opts = BytesOptions::new().precision(2).fixed_precision(true);
    // 1536 / 1000 = 1.536 → rounds to 1.54, no trailing zeros to pad here
    assert_eq!(humfmt::bytes_with(1536_u64, opts).to_string(), "1.54KB");
    // 1500 / 1000 = 1.5 → fixed_precision pads to 1.50
    assert_eq!(humfmt::bytes_with(1500_u64, opts).to_string(), "1.50KB");
    // 1_000_000 / 1_000_000 = 1.0 → fixed_precision pads to 1.00
    assert_eq!(
        humfmt::bytes_with(1_000_000_u64, opts).to_string(),
        "1.00MB"
    );
}

#[test]
fn fixed_precision_with_space_and_binary() {
    let opts = BytesOptions::new()
        .binary()
        .precision(2)
        .space(true)
        .fixed_precision(true);
    // 1536 / 1024 = 1.5 → fixed_precision pads to 1.50
    assert_eq!(humfmt::bytes_with(1536_u64, opts).to_string(), "1.50 KiB");
    // 1024 / 1024 = 1.0 → fixed_precision pads to 1.00
    assert_eq!(humfmt::bytes_with(1024_u64, opts).to_string(), "1.00 KiB");
}

#[test]
fn fixed_precision_false_trims_by_default() {
    let opts = BytesOptions::new().binary().precision(2).space(true);
    // 1536 / 1024 = 1.5 → trimmed, no trailing zero
    assert_eq!(humfmt::bytes_with(1536_u64, opts).to_string(), "1.5 KiB");
    // 1024 / 1024 = 1.0 → trimmed to integer
    assert_eq!(humfmt::bytes_with(1024_u64, opts).to_string(), "1 KiB");
}

#[test]
fn fixed_precision_with_zero_precision_emits_no_decimal() {
    let opts = BytesOptions::new().precision(0).fixed_precision(true);
    // 1536 / 1000 = 1.536 → rounds to 2 at precision=0
    assert_eq!(humfmt::bytes_with(1536_u64, opts).to_string(), "2KB");
    // 1024 / 1000 = 1.024 → rounds to 1 at precision=0
    assert_eq!(humfmt::bytes_with(1024_u64, opts).to_string(), "1KB");
}

#[test]
fn supports_forcing_specific_unit() {
    let opts = BytesOptions::new().unit(ByteUnit::MB).precision(3);

    // Scale up (small values padded with zeros)
    assert_eq!(humfmt::bytes_with(150_000_u64, opts).to_string(), "0.15MB");

    // Exact match
    assert_eq!(humfmt::bytes_with(1_500_000_u64, opts).to_string(), "1.5MB");

    // Scale down (large values left unscaled past the limit)
    assert_eq!(
        humfmt::bytes_with(1_500_000_000_u64, opts).to_string(),
        "1500MB"
    );
}

#[test]
fn supports_min_unit_clamping() {
    let opts = BytesOptions::new().min_unit(ByteUnit::KB).precision(2);

    // 500 B is forced to 0.5 KB
    assert_eq!(humfmt::bytes_with(500_u64, opts).to_string(), "0.5KB");

    // 1.5 MB scales normally
    assert_eq!(humfmt::bytes_with(1_500_000_u64, opts).to_string(), "1.5MB");
}

#[test]
fn supports_max_unit_clamping() {
    let opts = BytesOptions::new().max_unit(ByteUnit::GB);

    // 2 TB is clamped to 2000 GB
    assert_eq!(
        humfmt::bytes_with(2_000_000_000_000_u64, opts).to_string(),
        "2000GB"
    );
}

#[test]
fn min_unit_greater_than_max_unit_safely_clamps() {
    // If the user maliciously sets min = GB and max = KB, it safely normalizes.
    let opts = BytesOptions::new()
        .min_unit(ByteUnit::GB)
        .max_unit(ByteUnit::KB);
    assert_eq!(
        humfmt::bytes_with(1_500_000_000_000_u64, opts).to_string(),
        "1500GB"
    );
}

#[test]
fn supports_significant_digits() {
    let opts = BytesOptions::new().significant_digits(3);
    assert_eq!(humfmt::bytes_with(1234_u64, opts).to_string(), "1.23KB");
    assert_eq!(humfmt::bytes_with(12345_u64, opts).to_string(), "12.3KB");
    assert_eq!(humfmt::bytes_with(123456_u64, opts).to_string(), "123KB");
}

#[test]
fn supports_rounding_modes() {
    // 1500 B = 1.5 KB
    let base = BytesOptions::new().precision(0);
    assert_eq!(
        humfmt::bytes_with(1500_u64, base.rounding(RoundingMode::HalfUp)).to_string(),
        "2KB"
    );
    assert_eq!(
        humfmt::bytes_with(1500_u64, base.rounding(RoundingMode::Floor)).to_string(),
        "1KB"
    );
    assert_eq!(
        humfmt::bytes_with(1500_u64, base.rounding(RoundingMode::Ceil)).to_string(),
        "2KB"
    );

    // -1500 B = -1.5 KB
    assert_eq!(
        humfmt::bytes_with(-1500_i64, base.rounding(RoundingMode::HalfUp)).to_string(),
        "-2KB"
    );
    assert_eq!(
        humfmt::bytes_with(-1500_i64, base.rounding(RoundingMode::Floor)).to_string(),
        "-2KB"
    );
    assert_eq!(
        humfmt::bytes_with(-1500_i64, base.rounding(RoundingMode::Ceil)).to_string(),
        "-1KB"
    );
}

#[test]
fn supports_bits_mode_decimal() {
    let opts = BytesOptions::new().bits(true);
    // 1000 Bytes = 8000 bits = 8 Kb
    assert_eq!(humfmt::bytes_with(1000_u64, opts).to_string(), "8Kb");

    // 1_500_000 Bytes = 12_000_000 bits = 12 Mb
    assert_eq!(humfmt::bytes_with(1_500_000_u64, opts).to_string(), "12Mb");
}

#[test]
fn supports_bits_mode_binary() {
    let opts = BytesOptions::new().bits(true).binary();
    // 1024 Bytes = 8192 bits = 8 Kib
    assert_eq!(humfmt::bytes_with(1024_u64, opts).to_string(), "8Kib");
}

#[test]
fn supports_bits_mode_long_units() {
    let opts = BytesOptions::new().bits(true).long_units();
    // 1 Byte = 8 bits
    assert_eq!(humfmt::bytes_with(1_u64, opts).to_string(), "8 bits");

    // 125 Bytes = 1000 bits = 1 kilobit
    assert_eq!(humfmt::bytes_with(125_u64, opts).to_string(), "1 kilobit");
}

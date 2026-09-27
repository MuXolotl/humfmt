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
    assert_eq!(humfmt::bytes_with(1536, opts).to_string(), "1.54KB");
}

#[test]
fn supports_custom_decimal_separator() {
    let opts = BytesOptions::new().decimal_separator(',');
    assert_eq!(humfmt::bytes_with(1536, opts).to_string(), "1,5KB");
}

#[test]
fn supports_optional_space_before_short_units() {
    let opts = BytesOptions::new().space(true);
    assert_eq!(humfmt::bytes_with(999_u64, opts).to_string(), "999 B");
    assert_eq!(humfmt::bytes_with(1536_u64, opts).to_string(), "1.5 KB");

    let bin = BytesOptions::new().binary().precision(2).space(true);
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
fn significant_digits_round_up_across_decimal_unit_boundary() {
    let opts = BytesOptions::new()
        .min_unit(ByteUnit::KB)
        .significant_digits(1);
    assert_eq!(humfmt::bytes_with(999_900_u64, opts).to_string(), "1MB");
}

#[test]
fn significant_digits_round_up_past_u128_range_in_forced_unit() {
    let opts = BytesOptions::new()
        .unit(ByteUnit::B)
        .significant_digits(1)
        .rounding(RoundingMode::Ceil);

    assert_eq!(
        humfmt::bytes_with(u128::MAX, opts).to_string(),
        "400000000000000000000000000000000000000B"
    );
}

#[test]
fn significant_digits_keep_long_unit_labels_plural_when_rounded_up() {
    let opts = BytesOptions::new()
        .long_units()
        .significant_digits(1)
        .rounding(RoundingMode::Ceil);

    assert_eq!(
        humfmt::bytes_with(u128::MAX, opts).to_string(),
        "400000000000000000000 exabytes"
    );
}

#[test]
fn rounds_up_across_decimal_unit_boundary() {
    assert_eq!(bytes(999_950).to_string(), "1MB");
}

#[test]
fn fixed_precision_preserves_trailing_zeros() {
    let opts = BytesOptions::new().precision(2).fixed_precision(true);
    assert_eq!(humfmt::bytes_with(1536_u64, opts).to_string(), "1.54KB");
    assert_eq!(humfmt::bytes_with(1500_u64, opts).to_string(), "1.50KB");
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
    assert_eq!(humfmt::bytes_with(1536_u64, opts).to_string(), "1.50 KiB");
    assert_eq!(humfmt::bytes_with(1024_u64, opts).to_string(), "1.00 KiB");
}

#[test]
fn fixed_precision_false_trims_by_default() {
    let opts = BytesOptions::new().binary().precision(2).space(true);
    assert_eq!(humfmt::bytes_with(1536_u64, opts).to_string(), "1.5 KiB");
    assert_eq!(humfmt::bytes_with(1024_u64, opts).to_string(), "1 KiB");
}

#[test]
fn fixed_precision_with_zero_precision_emits_no_decimal() {
    let opts = BytesOptions::new().precision(0).fixed_precision(true);
    assert_eq!(humfmt::bytes_with(1536_u64, opts).to_string(), "2KB");
    assert_eq!(humfmt::bytes_with(1024_u64, opts).to_string(), "1KB");
}

#[test]
fn supports_forcing_specific_unit() {
    let opts = BytesOptions::new().unit(ByteUnit::MB).precision(3);

    assert_eq!(humfmt::bytes_with(150_000_u64, opts).to_string(), "0.15MB");
    assert_eq!(humfmt::bytes_with(1_500_000_u64, opts).to_string(), "1.5MB");
    assert_eq!(
        humfmt::bytes_with(1_500_000_000_u64, opts).to_string(),
        "1500MB"
    );
}

#[test]
fn supports_min_unit_clamping() {
    let opts = BytesOptions::new().min_unit(ByteUnit::KB).precision(2);

    assert_eq!(humfmt::bytes_with(500_u64, opts).to_string(), "0.5KB");
    assert_eq!(humfmt::bytes_with(1_500_000_u64, opts).to_string(), "1.5MB");
}

#[test]
fn supports_max_unit_clamping() {
    let opts = BytesOptions::new().max_unit(ByteUnit::GB);

    assert_eq!(
        humfmt::bytes_with(2_000_000_000_000_u64, opts).to_string(),
        "2000GB"
    );
}

#[test]
fn min_unit_greater_than_max_unit_safely_clamps() {
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
    assert_eq!(humfmt::bytes_with(1000_u64, opts).to_string(), "8Kb");
    assert_eq!(humfmt::bytes_with(1_500_000_u64, opts).to_string(), "12Mb");
}

#[test]
fn supports_bits_mode_binary() {
    let opts = BytesOptions::new().bits(true).binary();
    assert_eq!(humfmt::bytes_with(1024_u64, opts).to_string(), "8Kib");
}

#[test]
fn supports_bits_mode_long_units() {
    let opts = BytesOptions::new().bits(true).long_units();
    assert_eq!(humfmt::bytes_with(1_u64, opts).to_string(), "8 bits");
    assert_eq!(humfmt::bytes_with(125_u64, opts).to_string(), "1 kilobit");
}

#[test]
fn forces_sign_on_positive_values() {
    let opts = BytesOptions::new().force_sign(true);

    assert_eq!(humfmt::bytes_with(1_536_u64, opts).to_string(), "+1.5KB");
    assert_eq!(humfmt::bytes_with(1_536_i64, opts).to_string(), "+1.5KB");
    assert_eq!(humfmt::bytes_with(512_u64, opts).to_string(), "+512B");
    assert_eq!(humfmt::bytes_with(-1_536_i64, opts).to_string(), "-1.5KB");
}

#[test]
fn forced_sign_leaves_zero_unsigned() {
    let opts = BytesOptions::new().force_sign(true);

    assert_eq!(humfmt::bytes_with(0_u64, opts).to_string(), "0B");
    assert_eq!(humfmt::bytes_with(0_i64, opts).to_string(), "0B");
    assert_eq!(
        humfmt::bytes_with(0_u64, BytesOptions::new().force_sign(false)).to_string(),
        "0B"
    );
}

#[test]
fn value_rounded_to_zero_keeps_no_sign() {
    // One byte is a thousandth of a kilobyte, which rounds to zero. The sign
    // belongs to the value that is written, not to the input.
    let kb = BytesOptions::new().min_unit(ByteUnit::KB);

    assert_eq!(humfmt::bytes_with(-1_i64, kb).to_string(), "0KB");
    assert_eq!(
        humfmt::bytes_with(-1_i64, kb.precision(0)).to_string(),
        "0KB"
    );
    assert_eq!(
        humfmt::bytes_with(-1_i64, kb.fixed_precision(true)).to_string(),
        "0.0KB"
    );
    assert_eq!(
        humfmt::bytes_with(-1_i64, kb.force_sign(true)).to_string(),
        "0KB"
    );
}

#[test]
fn significant_digits_count_from_the_first_nonzero_digit() {
    // `12_345` bytes is `0.012345` of a megabyte, so its leading digit sits two
    // places behind the decimal point and three significant digits reach the
    // fourth.
    let below_unit = BytesOptions::new()
        .min_unit(ByteUnit::MB)
        .significant_digits(3);

    assert_eq!(
        humfmt::bytes_with(12_345_u64, below_unit).to_string(),
        "0.0123MB"
    );
    assert_eq!(
        humfmt::bytes_with(12_345_u64, below_unit.significant_digits(1)).to_string(),
        "0.01MB"
    );
    assert_eq!(
        humfmt::bytes_with(12_345_u64, below_unit.significant_digits(6)).to_string(),
        "0.012345MB"
    );

    // Rounding the leading nine up carries into the unit itself.
    assert_eq!(
        humfmt::bytes_with(999_999_u64, below_unit).to_string(),
        "1MB"
    );
}

#[test]
fn bits_scale_units_instead_of_saturating() {
    // `u128::MAX` bytes are `2.7e39` bits, which no `u128` holds. Scaling the
    // unit by eight keeps the magnitude, and with it the digits, exact.
    let bits = BytesOptions::new().bits(true);

    assert_eq!(
        humfmt::bytes_with(u128::MAX, bits).to_string(),
        "2722258935367507707707Eb"
    );
    assert_eq!(
        humfmt::bytes_with(u128::MAX, bits.precision(0)).to_string(),
        "2722258935367507707707Eb"
    );
    assert_eq!(
        humfmt::bytes_with(u128::MAX, bits.binary()).to_string(),
        "2361183241434822606848Eib"
    );
    assert_eq!(
        humfmt::bytes_with(u128::MAX, bits.significant_digits(1)).to_string(),
        "3000000000000000000000Eb"
    );

    // A kilobyte in bits is eight kilobit, still one unit up from bytes.
    assert_eq!(humfmt::bytes_with(1000_u64, bits).to_string(), "8Kb");
    assert_eq!(
        humfmt::bytes_with(500_u64, bits.min_unit(ByteUnit::KB)).to_string(),
        "4Kb"
    );
}

#[test]
fn forced_sign_combines_with_units_and_precision() {
    let binary = BytesOptions::new()
        .force_sign(true)
        .binary()
        .precision(2)
        .space(true);
    assert_eq!(
        humfmt::bytes_with(1_536_u64, binary).to_string(),
        "+1.5 KiB"
    );

    let bits = BytesOptions::new().force_sign(true).bits(true);
    assert_eq!(humfmt::bytes_with(1_500_000_u64, bits).to_string(), "+12Mb");

    let long = BytesOptions::new().force_sign(true).long_units();
    assert_eq!(humfmt::bytes_with(1_u64, long).to_string(), "+1 byte");

    let clamped = BytesOptions::new()
        .force_sign(true)
        .min_unit(humfmt::ByteUnit::KB)
        .precision(2);
    assert_eq!(humfmt::bytes_with(500_u64, clamped).to_string(), "+0.5KB");
}

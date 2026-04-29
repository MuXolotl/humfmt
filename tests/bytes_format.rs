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

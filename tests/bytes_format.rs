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
fn supports_negative_values() {
    assert_eq!(bytes(-1536).to_string(), "-1.5KB");
}

#[test]
fn supports_extension_trait_usage() {
    assert_eq!(1536_u64.human_bytes().to_string(), "1.5KB");
}

use humfmt::{ordinal, Humanize};

#[test]
fn formats_basic_ordinals() {
    assert_eq!(ordinal(1).to_string(), "1st");
    assert_eq!(ordinal(2).to_string(), "2nd");
    assert_eq!(ordinal(3).to_string(), "3rd");
    assert_eq!(ordinal(4).to_string(), "4th");
}

#[test]
fn handles_teen_suffix_exceptions() {
    assert_eq!(ordinal(11).to_string(), "11th");
    assert_eq!(ordinal(12).to_string(), "12th");
    assert_eq!(ordinal(13).to_string(), "13th");
}

#[test]
fn formats_larger_ordinals() {
    assert_eq!(ordinal(21).to_string(), "21st");
    assert_eq!(ordinal(42).to_string(), "42nd");
    assert_eq!(ordinal(103).to_string(), "103rd");
    assert_eq!(ordinal(111).to_string(), "111th");
}

#[test]
fn supports_extension_trait_usage() {
    assert_eq!(21_u32.human_ordinal().to_string(), "21st");
}

#[test]
fn preserves_negative_prefix_for_signed_values() {
    assert_eq!(ordinal(-1).to_string(), "-1st");
    assert_eq!(ordinal(-12).to_string(), "-12th");
}

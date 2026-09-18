use core::time::Duration;

use humfmt::{ago, bytes, duration, number, ordinal, percent};

#[test]
fn number_honours_width_and_alignment() {
    // Unspecified align uses string-like left padding (`Formatter::pad`).
    assert_eq!(format!("{:10}", number(1_500)), "1.5K      ");
    assert_eq!(format!("{:>10}", number(1_500)), "      1.5K");
    assert_eq!(format!("{:<10}", number(1_500)), "1.5K      ");
    assert_eq!(format!("{:0>8}", number(42)), "00000042");
}

#[test]
fn bytes_honours_width() {
    assert_eq!(format!("{:>8}", bytes(1536_u64)), "   1.5KB");
}

#[test]
fn duration_and_ago_honour_width() {
    let value = Duration::from_secs(90);
    assert_eq!(format!("{:>10}", duration(value)), "    1m 30s");
    assert_eq!(format!("{:>14}", ago(value)), "    1m 30s ago");
}

#[test]
fn ordinal_and_percent_honour_width() {
    assert_eq!(format!("{:>6}", ordinal(21)), "  21st");
    assert_eq!(format!("{:>8}", percent(0.5_f64)), "     50%");
}

#[test]
fn width_none_matches_to_string() {
    assert_eq!(format!("{}", number(15_320)), number(15_320).to_string());
    assert_eq!(format!("{}", bytes(1536_u64)), bytes(1536_u64).to_string());
}

//! Format specifier support (`width`, `fill`, `align`) for every formatter.
//!
//! The formatters honour the same padding rules as a `&str`: width, fill and
//! alignment are applied, while sign, alternate and zero flags and the precision
//! are not, matching what `core::fmt` exposes to a custom `Display`.

use core::fmt;
use core::time::Duration;

use humfmt::{
    ago, bytes, duration, duration_with, list, number, ordinal, percent, DurationOptions,
};

/// Reference type: its `Display` delegates to `Formatter::pad`, which is how
/// `&str` itself honours width, fill and alignment.
struct StdRef(&'static str);

impl fmt::Display for StdRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad(self.0)
    }
}

/// Asserts that `humfmt_value` pads exactly like a plain string of the same width.
macro_rules! assert_pads_like_std {
    ($spec:literal, $humfmt_value:expr, $plain:literal) => {{
        assert_eq!(
            format!($spec, $humfmt_value),
            format!($spec, StdRef($plain)),
            "specifier {} on {:?}",
            stringify!($spec),
            $plain
        );
    }};
}

#[test]
fn applies_width_to_every_formatter() {
    assert_eq!(format!("{:>10}", number(15_320)), "     15.3K");
    assert_eq!(format!("{:>10}", bytes(1536)), "     1.5KB");
    assert_eq!(format!("{:>10}", percent(0.423)), "     42.3%");
    assert_eq!(format!("{:>10}", ordinal(21)), "      21st");
    assert_eq!(
        format!("{:>10}", duration(Duration::from_secs(3661))),
        "     1h 1m"
    );
    assert_eq!(
        format!("{:>12}", ago(Duration::from_secs(3661))),
        "   1h 1m ago"
    );
    assert_eq!(
        format!("{:>24}", list(&["red", "green", "blue"])),
        "    red, green, and blue"
    );
}

#[test]
fn width_only_sets_a_minimum() {
    // A value wider than the requested width is written in full.
    assert_eq!(format!("{:>3}", bytes(1536)), "1.5KB");
    assert_eq!(format!("{:>1}", number(15_320)), "15.3K");
    assert_eq!(format!("{:0}", number(15_320)), "15.3K");

    let opts = DurationOptions::new().long_units().max_units(3);
    assert_eq!(
        format!("{:>10}", duration_with(Duration::from_secs(3665), opts)),
        "1 hour 1 minute 5 seconds"
    );
}

#[test]
fn unpadded_output_is_unchanged() {
    assert_eq!(format!("{}", number(15_320)), "15.3K");
    assert_eq!(format!("{}", bytes(1536)), "1.5KB");
    assert_eq!(format!("{}", percent(0.423)), "42.3%");
    assert_eq!(format!("{}", ordinal(21)), "21st");
    assert_eq!(format!("{}", duration(Duration::from_secs(3661))), "1h 1m");
    assert_eq!(format!("{}", ago(Duration::from_secs(3661))), "1h 1m ago");
    assert_eq!(
        format!("{}", list(&["red", "green", "blue"])),
        "red, green, and blue"
    );
}

#[test]
fn default_alignment_is_left() {
    assert_pads_like_std!("{:10}", number(15_320), "15.3K");
    assert_pads_like_std!("{:12}", ordinal(-3), "-3rd");
    assert_pads_like_std!("{:10}", list(&["red", "green"]), "red and green");
}

#[test]
fn honours_explicit_alignment_and_fill() {
    assert_pads_like_std!("{:<10}", number(15_320), "15.3K");
    assert_pads_like_std!("{:>10}", number(15_320), "15.3K");
    assert_pads_like_std!("{:^10}", number(15_320), "15.3K");
    assert_pads_like_std!("{:*^9}", number(15_320), "15.3K");
    assert_pads_like_std!("{:*>12}", bytes(1536), "1.5KB");
    assert_pads_like_std!("{:->12}", number(-15_320), "-15.3K");
    assert_pads_like_std!("{:^11}", percent(0.423), "42.3%");
}

#[test]
fn centers_extra_column_on_the_right() {
    // An odd pad puts the shorter run on the left, as `Formatter` does.
    assert_pads_like_std!("{:^6}", number(15_320), "15.3K");
    assert_pads_like_std!("{:^8}", number(15_320), "15.3K");
    assert_eq!(format!("{:^8}", number(15_320)), " 15.3K  ");
}

#[test]
fn width_is_character_based() {
    let items = ["café", "naïve"];
    assert_pads_like_std!("{:>16}", list(&items), "café and naïve");
    assert_pads_like_std!("{:^16}", list(&items), "café and naïve");
}

#[test]
fn ignores_sign_and_alternate_flags() {
    // `+` and `#` only mean something to the numeric `Display` impls in `core`;
    // for a custom `Display`, `Formatter` exposes neither.
    assert_pads_like_std!("{:+10}", number(15_320), "15.3K");
    assert_pads_like_std!("{:#10}", bytes(1536), "1.5KB");
}

#[test]
fn ignores_the_zero_flag() {
    // `{:08}` fills with spaces, not zeros, exactly like `&str` padding.
    assert_pads_like_std!("{:08}", number(15_320), "15.3K");
    assert_pads_like_std!("{:08}", ordinal(21), "21st");
}

#[test]
fn precision_never_truncates() {
    // Truncating would cut units off (`"1.5KB"` -> `"1."`), so precision belongs
    // to the options — `bytes_with(.., opts.precision(2))` — never to the specifier.
    assert_eq!(format!("{:>10.2}", bytes(1536)), "     1.5KB");
    assert_eq!(format!("{:.1}", bytes(1536)), "1.5KB");
    assert_eq!(format!("{:.0}", number(15_320)), "15.3K");
}

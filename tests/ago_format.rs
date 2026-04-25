use core::time::Duration;

use humfmt::{ago, DurationOptions, Humanize};

#[test]
fn formats_basic_relative_times() {
    assert_eq!(ago(Duration::from_secs(90)).to_string(), "1m 30s ago");
    assert_eq!(ago(Duration::from_secs(3661)).to_string(), "1h 1m ago");
}

#[test]
fn formats_zero_duration_as_relative_time() {
    assert_eq!(ago(Duration::ZERO).to_string(), "0s ago");
}

#[test]
fn supports_long_units() {
    let opts = DurationOptions::new().long_units();
    assert_eq!(
        humfmt::ago_with(Duration::from_millis(1500), opts).to_string(),
        "1 second 500 milliseconds ago"
    );
}

#[test]
fn supports_max_unit_override() {
    let opts = DurationOptions::new().max_units(3);
    assert_eq!(
        humfmt::ago_with(Duration::from_secs(3665), opts).to_string(),
        "1h 1m 5s ago"
    );
}

#[test]
fn supports_extension_trait_usage() {
    assert_eq!(
        Duration::from_secs(90).human_ago().to_string(),
        "1m 30s ago"
    );
}

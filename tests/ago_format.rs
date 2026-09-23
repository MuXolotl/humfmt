use core::time::Duration;

use humfmt::{ago, AgoOptions, DurationOptions, Humanize};

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
    let opts = AgoOptions::new().long_units();
    assert_eq!(
        humfmt::ago_with(Duration::from_millis(1500), opts).to_string(),
        "1 second 500 milliseconds ago"
    );
}

#[test]
fn supports_max_unit_override() {
    let opts = AgoOptions::new().max_units(3);
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

#[test]
fn converts_to_and_from_duration_options() {
    let shared = DurationOptions::new().long_units().max_units(3);
    let ago_opts = AgoOptions::from(shared);
    assert_eq!(
        humfmt::ago_with(Duration::from_secs(3665), ago_opts).to_string(),
        "1 hour 1 minute 5 seconds ago"
    );

    let back = DurationOptions::from(ago_opts);
    assert_eq!(
        humfmt::duration_with(Duration::from_secs(3665), back).to_string(),
        "1 hour 1 minute 5 seconds"
    );
}

#[test]
fn clamps_max_units_like_duration_options() {
    let below = AgoOptions::new().max_units(0);
    assert_eq!(
        humfmt::ago_with(Duration::from_secs(3665), below).to_string(),
        "1h ago"
    );

    let above = AgoOptions::new().max_units(200);
    assert_eq!(
        humfmt::ago_with(Duration::from_nanos(1_001_001_001), above.long_units()).to_string(),
        "1 second 1 millisecond 1 microsecond 1 nanosecond ago"
    );
}

#[test]
fn default_options_match_new() {
    assert_eq!(
        humfmt::ago_with(Duration::from_secs(3661), AgoOptions::default()).to_string(),
        humfmt::ago_with(Duration::from_secs(3661), AgoOptions::new()).to_string()
    );
}

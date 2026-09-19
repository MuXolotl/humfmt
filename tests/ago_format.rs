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

#[test]
fn renders_short_durations_as_just_now() {
    let opts = AgoOptions::new().just_now(Duration::from_secs(5));

    assert_eq!(
        humfmt::ago_with(Duration::ZERO, opts).to_string(),
        "just now"
    );
    assert_eq!(
        humfmt::ago_with(Duration::from_secs(3), opts).to_string(),
        "just now"
    );
    assert_eq!(
        humfmt::ago_with(Duration::from_secs(5), opts).to_string(),
        "5s ago"
    );
    assert_eq!(
        humfmt::ago_with(Duration::from_secs(90), opts).to_string(),
        "1m 30s ago"
    );

    // The threshold is a plain comparison, so the extremes stay untouched.
    let huge = humfmt::ago_with(Duration::MAX, opts).to_string();
    assert!(huge.ends_with(" ago"), "unexpected output: {huge}");
}

#[test]
fn just_now_is_off_by_default() {
    assert_eq!(
        humfmt::ago_with(Duration::ZERO, AgoOptions::default()).to_string(),
        "0s ago"
    );
    assert_eq!(
        humfmt::ago_with(Duration::from_secs(3), AgoOptions::new()).to_string(),
        "3s ago"
    );

    // A zero threshold means "nothing is below it".
    let disabled = AgoOptions::new().just_now(Duration::ZERO);
    assert_eq!(
        humfmt::ago_with(Duration::ZERO, disabled).to_string(),
        "0s ago"
    );
}

#[test]
fn just_now_ignores_unit_options() {
    let opts = AgoOptions::new()
        .just_now(Duration::from_secs(5))
        .long_units()
        .max_units(1);

    assert_eq!(
        humfmt::ago_with(Duration::from_secs(2), opts).to_string(),
        "just now"
    );
}

#[test]
fn supports_just_now_through_the_extension_trait() {
    let opts = AgoOptions::new().just_now(Duration::from_millis(500));
    assert_eq!(
        Duration::from_millis(200).human_ago_with(opts).to_string(),
        "just now"
    );
}

#[test]
fn from_duration_options_starts_with_just_now_off() {
    let fresh = AgoOptions::from(DurationOptions::new());
    assert_eq!(
        humfmt::ago_with(Duration::ZERO, fresh).to_string(),
        "0s ago"
    );

    // Converting back into DurationOptions drops the phrase, which has no
    // meaning for plain duration output.
    let with_phrase = AgoOptions::new().just_now(Duration::from_secs(5));
    let back = DurationOptions::from(with_phrase);
    assert_eq!(
        humfmt::duration_with(Duration::from_secs(3), back).to_string(),
        "3s"
    );
}

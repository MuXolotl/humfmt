use core::time::Duration;

use humfmt::{duration, DurationOptions, Humanize};

#[test]
fn formats_zero_duration() {
    assert_eq!(duration(Duration::ZERO).to_string(), "0s");
}

#[test]
fn formats_compound_durations() {
    assert_eq!(duration(Duration::from_secs(3661)).to_string(), "1h 1m");
    assert_eq!(duration(Duration::from_secs(90061)).to_string(), "1d 1h");
}

#[test]
fn formats_subsecond_durations() {
    assert_eq!(
        duration(Duration::from_millis(1500)).to_string(),
        "1s 500ms"
    );
    assert_eq!(
        duration(Duration::from_nanos(1_500)).to_string(),
        "1us 500ns"
    );
}

#[test]
fn supports_long_units() {
    let opts = DurationOptions::new().long_units();
    assert_eq!(
        humfmt::duration_with(Duration::from_millis(1500), opts).to_string(),
        "1 second 500 milliseconds"
    );
}

#[test]
fn supports_max_unit_override() {
    let opts = DurationOptions::new().max_units(3);
    assert_eq!(
        humfmt::duration_with(Duration::from_secs(3665), opts).to_string(),
        "1h 1m 5s"
    );
}

#[test]
fn supports_max_units_up_to_seven() {
    // 1s 1ms 1us 1ns — four distinct non-zero units below second boundary.
    let value = Duration::from_nanos(1_001_001_001);
    let opts = DurationOptions::new().max_units(7).long_units();
    assert_eq!(
        humfmt::duration_with(value, opts).to_string(),
        "1 second 1 millisecond 1 microsecond 1 nanosecond"
    );
}

#[test]
fn max_units_clamps_to_one_at_minimum() {
    let opts = DurationOptions::new().max_units(0);
    assert_eq!(
        humfmt::duration_with(Duration::from_secs(3661), opts).to_string(),
        "1h"
    );
}

#[test]
fn supports_extension_trait_usage() {
    assert_eq!(
        Duration::from_secs(90).human_duration().to_string(),
        "1m 30s"
    );
}

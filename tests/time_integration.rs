#![cfg(feature = "time")]

use humfmt::{
    time::{self as humtime, TimeHumanize},
    DurationConversionError, DurationOptions,
};

#[test]
fn formats_positive_time_duration() {
    let delta = ::time::Duration::seconds(90);
    assert_eq!(humtime::duration(delta).unwrap().to_string(), "1m 30s");
    assert_eq!(delta.try_human_ago().unwrap().to_string(), "1m 30s ago");
}

#[test]
fn rejects_negative_time_duration() {
    let delta = -::time::Duration::seconds(5);
    assert!(matches!(
        humtime::duration(delta),
        Err(DurationConversionError::NegativeDuration)
    ));
    assert!(matches!(
        humtime::ago(delta),
        Err(DurationConversionError::NegativeDuration)
    ));
    assert!(matches!(
        delta.try_human_duration(),
        Err(DurationConversionError::NegativeDuration)
    ));
}

#[test]
fn supports_custom_options_for_time_duration() {
    let delta = ::time::Duration::milliseconds(1500);
    let out = humtime::ago_with(delta, DurationOptions::new().long_units()).unwrap();
    assert_eq!(out.to_string(), "1 second 500 milliseconds ago");
}

#[test]
fn supports_ago_since_for_offset_datetimes() {
    let then = ::time::OffsetDateTime::from_unix_timestamp(0).unwrap();
    let now = ::time::OffsetDateTime::from_unix_timestamp(3665).unwrap();
    assert_eq!(
        humtime::ago_since(then, now).unwrap().to_string(),
        "1h 1m ago"
    );
}

#[test]
fn supports_ago_since_with_long_options() {
    let then = ::time::OffsetDateTime::from_unix_timestamp(0).unwrap();
    let now = ::time::OffsetDateTime::from_unix_timestamp(3665).unwrap();
    let out =
        humtime::ago_since_with(then, now, DurationOptions::new().long_units().max_units(3)).unwrap();

    assert_eq!(out.to_string(), "1 hour 1 minute 5 seconds ago");
}

#[test]
fn every_entry_point_reports_the_same_error_type() {
    let delta = -::time::Duration::seconds(5);

    // The `*_checked` twins take the same path and report the same type.
    assert!(matches!(
        humtime::duration_checked(delta),
        Err(DurationConversionError::NegativeDuration)
    ));
    assert!(matches!(
        humtime::ago_checked(delta),
        Err(DurationConversionError::NegativeDuration)
    ));
}

#![cfg(feature = "time")]

use humfmt::{
    time::{self as humtime, TimeHumanize},
    DurationOptions, NegativeDurationError,
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
        Err(NegativeDurationError)
    ));
    assert!(matches!(humtime::ago(delta), Err(NegativeDurationError)));
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

#[cfg(feature = "polish")]
#[test]
fn supports_locale_aware_ago_since_for_offset_datetimes() {
    let then = ::time::OffsetDateTime::from_unix_timestamp(0).unwrap();
    let now = ::time::OffsetDateTime::from_unix_timestamp(3665).unwrap();
    let out = humtime::ago_since_with(
        then,
        now,
        DurationOptions::new()
            .locale(humfmt::locale::Polish)
            .long_units()
            .max_units(3),
    )
    .unwrap();

    assert_eq!(out.to_string(), "1 godzina 1 minuta 5 sekund temu");
}

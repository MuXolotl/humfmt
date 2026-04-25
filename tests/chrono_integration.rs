#![cfg(feature = "chrono")]

use humfmt::{
    chrono::{self as humchrono, ChronoHumanize},
    DurationConversionError, DurationOptions, NegativeDurationError,
};

#[test]
fn formats_positive_chrono_timedelta() {
    let delta = ::chrono::TimeDelta::try_seconds(90).unwrap();
    assert_eq!(humchrono::duration(delta).unwrap().to_string(), "1m 30s");
    assert_eq!(delta.try_human_ago().unwrap().to_string(), "1m 30s ago");
}

#[test]
fn rejects_negative_chrono_timedelta() {
    let delta = ::chrono::TimeDelta::try_seconds(-5).unwrap();
    assert!(matches!(
        humchrono::duration(delta),
        Err(NegativeDurationError)
    ));
    assert!(matches!(humchrono::ago(delta), Err(NegativeDurationError)));
}

#[test]
fn supports_custom_options_for_chrono_timedelta() {
    let delta = ::chrono::TimeDelta::milliseconds(1500);
    let out = humchrono::ago_with(delta, DurationOptions::new().long_units()).unwrap();
    assert_eq!(out.to_string(), "1 second 500 milliseconds ago");
}

#[test]
fn supports_ago_since_for_chrono_datetimes() {
    let then = ::chrono::DateTime::from_timestamp(0, 0).unwrap();
    let now = ::chrono::DateTime::from_timestamp(3665, 0).unwrap();
    assert_eq!(
        humchrono::ago_since(then, now).unwrap().to_string(),
        "1h 1m ago"
    );
}

#[cfg(feature = "russian")]
#[test]
fn supports_locale_aware_ago_since_for_chrono_datetimes() {
    let then = ::chrono::DateTime::from_timestamp(0, 0).unwrap();
    let now = ::chrono::DateTime::from_timestamp(3665, 0).unwrap();
    let out = humchrono::ago_since_with(
        then,
        now,
        DurationOptions::new()
            .locale(humfmt::locale::Russian)
            .long_units()
            .max_units(3),
    )
    .unwrap();

    assert_eq!(out.to_string(), "1 час 1 минута 5 секунд назад");
}

#[test]
fn checked_api_distinguishes_negative_duration_errors() {
    let delta = ::chrono::TimeDelta::try_seconds(-5).unwrap();
    assert!(matches!(
        humchrono::duration_checked(delta),
        Err(DurationConversionError::NegativeDuration)
    ));
    assert!(matches!(
        humchrono::ago_checked(delta),
        Err(DurationConversionError::NegativeDuration)
    ));
}

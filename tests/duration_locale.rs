use core::time::Duration;

use humfmt::{
    ago_with, duration_with,
    locale::{CustomLocale, DurationUnit},
    DurationOptions,
};

fn custom_duration_unit(unit: DurationUnit, count: u128, long: bool) -> &'static str {
    if !long {
        return match unit {
            DurationUnit::Minute => "m",
            DurationUnit::Second => "s",
            _ => "?",
        };
    }

    match unit {
        DurationUnit::Minute if count == 1 => "tick",
        DurationUnit::Minute => "ticks",
        DurationUnit::Second if count == 1 => "tock",
        DurationUnit::Second => "tocks",
        _ => "units",
    }
}

#[test]
fn supports_custom_duration_units_and_ago_word() {
    let locale = CustomLocale::english()
        .duration_unit_fn(custom_duration_unit)
        .ago_word("back");
    let opts = DurationOptions::new().locale(locale).long_units();

    assert_eq!(
        duration_with(Duration::from_secs(90), opts).to_string(),
        "1 tick 30 tocks"
    );
    assert_eq!(
        ago_with(Duration::from_secs(90), opts).to_string(),
        "1 tick 30 tocks back"
    );
}

#[cfg(feature = "russian")]
#[test]
fn formats_russian_duration_and_relative_time() {
    let opts = DurationOptions::new()
        .locale(humfmt::locale::Russian)
        .long_units()
        .max_units(3);

    assert_eq!(
        duration_with(Duration::from_secs(3665), opts).to_string(),
        "1 час 1 минута 5 секунд"
    );
    assert_eq!(
        ago_with(Duration::from_secs(90), opts).to_string(),
        "1 минута 30 секунд назад"
    );
}

#[cfg(feature = "polish")]
#[test]
fn formats_polish_duration_and_relative_time() {
    let opts = DurationOptions::new()
        .locale(humfmt::locale::Polish)
        .long_units()
        .max_units(3);

    assert_eq!(
        duration_with(Duration::from_secs(3665), opts).to_string(),
        "1 godzina 1 minuta 5 sekund"
    );
    assert_eq!(
        ago_with(Duration::from_secs(90), opts).to_string(),
        "1 minuta 30 sekund temu"
    );
}

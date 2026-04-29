use super::DurationUnit;

pub(crate) const MAX_COMPACT_SUFFIX_INDEX: usize = 6;

pub(crate) const SHORT_SUFFIXES: [&str; 12] = [
    "", " tys.", " mln", " mld", " bln", " bld", " tln", "", "", "", "", "",
];

pub(crate) const LONG_SUFFIXES: [&str; 12] = [
    "",
    " tysięcy",
    " milionów",
    " miliardów",
    " bilionów",
    " biliardów",
    " trylionów",
    "",
    "",
    "",
    "",
    "",
];

/// Built-in Polish locale pack.
///
/// Enabled with the `polish` feature flag.
///
/// This locale uses:
/// - decimal separator `,`
/// - grouping separator space (`' '`)
/// - list conjunction `"i"`
/// - serial comma disabled by default
/// - relative-time word `"temu"`
#[derive(Copy, Clone, Debug, Default)]
pub struct Polish;

impl super::Locale for Polish {
    fn compact_suffix(&self, idx: usize, long: bool) -> &'static str {
        let suffixes = if long {
            &LONG_SUFFIXES
        } else {
            &SHORT_SUFFIXES
        };

        if idx < suffixes.len() {
            suffixes[idx]
        } else {
            ""
        }
    }

    fn compact_suffix_for(&self, idx: usize, scaled: f64, long: bool) -> &'static str {
        compact_suffix_for(idx, scaled, long)
    }

    fn max_compact_suffix_index(&self) -> usize {
        MAX_COMPACT_SUFFIX_INDEX
    }

    fn decimal_separator(&self) -> char {
        ','
    }

    fn group_separator(&self) -> char {
        ' '
    }

    fn and_word(&self) -> &'static str {
        "i"
    }

    fn ago_word(&self) -> &'static str {
        "temu"
    }

    fn duration_unit(&self, unit: DurationUnit, count: u128, long: bool) -> &'static str {
        duration_unit(unit, count, long)
    }

    fn ordinal_suffix(&self, n: u128) -> &'static str {
        ordinal_suffix(n)
    }
}

pub(crate) fn compact_suffix_for(idx: usize, scaled: f64, long: bool) -> &'static str {
    if !long {
        if idx < SHORT_SUFFIXES.len() {
            return SHORT_SUFFIXES[idx];
        }
        return "";
    }

    match idx {
        1 => plural_form_scaled(scaled, " tysiąc", " tysiące", " tysięcy", " tysiąca"),
        2 => plural_form_scaled(scaled, " milion", " miliony", " milionów", " miliona"),
        3 => plural_form_scaled(scaled, " miliard", " miliardy", " miliardów", " miliarda"),
        4 => plural_form_scaled(scaled, " bilion", " biliony", " bilionów", " biliona"),
        5 => plural_form_scaled(scaled, " biliard", " biliardy", " biliardów", " biliarda"),
        6 => plural_form_scaled(scaled, " trylion", " tryliony", " trylionów", " tryliona"),
        _ => "",
    }
}

pub(crate) fn ordinal_suffix(_n: u128) -> &'static str {
    "."
}

pub(crate) fn duration_unit(unit: DurationUnit, count: u128, long: bool) -> &'static str {
    if !long {
        return match unit {
            DurationUnit::Day => "d",
            DurationUnit::Hour => "godz.",
            DurationUnit::Minute => "min",
            DurationUnit::Second => "s",
            DurationUnit::Millisecond => "ms",
            DurationUnit::Microsecond => "us",
            DurationUnit::Nanosecond => "ns",
        };
    }

    // Polish plural rules (CLDR-style):
    // - "one": only 1
    // - "few": integers ending in 2..4, excluding 12..14
    // - "many": all other integers
    match unit {
        DurationUnit::Day => plural_form_int(count, "dzień", "dni", "dni"),
        DurationUnit::Hour => plural_form_int(count, "godzina", "godziny", "godzin"),
        DurationUnit::Minute => plural_form_int(count, "minuta", "minuty", "minut"),
        DurationUnit::Second => plural_form_int(count, "sekunda", "sekundy", "sekund"),
        DurationUnit::Millisecond => {
            plural_form_int(count, "milisekunda", "milisekundy", "milisekund")
        }
        DurationUnit::Microsecond => {
            plural_form_int(count, "mikrosekunda", "mikrosekundy", "mikrosekund")
        }
        DurationUnit::Nanosecond => {
            plural_form_int(count, "nanosekunda", "nanosekundy", "nanosekund")
        }
    }
}

#[inline]
fn plural_form_scaled(
    value: f64,
    one: &'static str,
    few: &'static str,
    many: &'static str,
    fraction: &'static str,
) -> &'static str {
    if !is_integer(value) {
        return fraction;
    }

    if value < 0.0 || value > (u128::MAX as f64) {
        return many;
    }

    plural_form_int(value as u128, one, few, many)
}

#[inline]
fn plural_form_int(
    n: u128,
    one: &'static str,
    few: &'static str,
    many: &'static str,
) -> &'static str {
    if n == 1 {
        return one;
    }

    let last_two = n % 100;
    let last = n % 10;

    if (2..=4).contains(&last) && !(12..=14).contains(&last_two) {
        few
    } else {
        many
    }
}

#[inline]
fn is_integer(value: f64) -> bool {
    value == (value as u128) as f64
}

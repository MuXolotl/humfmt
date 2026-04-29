use super::DurationUnit;

pub(crate) const MAX_COMPACT_SUFFIX_INDEX: usize = 11;

pub(crate) const SHORT_SUFFIXES: [&str; 12] = [
    "",
    " тыс.",
    " млн",
    " млрд",
    " трлн",
    " квадрлн",
    " квинтлн",
    " секстлн",
    " септлн",
    " октлн",
    " нониллн",
    " дециллн",
];

pub(crate) const LONG_SUFFIXES: [&str; 12] = [
    "",
    " тысяч",
    " миллионов",
    " миллиардов",
    " триллионов",
    " квадриллионов",
    " квинтиллионов",
    " секстиллионов",
    " септиллионов",
    " октиллионов",
    " нониллионов",
    " дециллионов",
];

/// Built-in Russian locale pack.
///
/// Enabled with the `russian` feature flag.
///
/// This locale uses:
/// - decimal separator `,`
/// - grouping separator space (`' '`)
/// - list conjunction `"и"`
/// - serial comma disabled by default
/// - relative-time word `"назад"`
#[derive(Copy, Clone, Debug, Default)]
pub struct Russian;

impl super::Locale for Russian {
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
        "и"
    }

    fn ago_word(&self) -> &'static str {
        "назад"
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
        1 => plural_form_scaled(scaled, " тысяча", " тысячи", " тысяч"),
        2 => plural_form_scaled(scaled, " миллион", " миллиона", " миллионов"),
        3 => plural_form_scaled(scaled, " миллиард", " миллиарда", " миллиардов"),
        4 => plural_form_scaled(scaled, " триллион", " триллиона", " триллионов"),
        5 => plural_form_scaled(scaled, " квадриллион", " квадриллиона", " квадриллионов"),
        6 => plural_form_scaled(scaled, " квинтиллион", " квинтиллиона", " квинтиллионов"),
        7 => plural_form_scaled(scaled, " секстиллион", " секстиллиона", " секстиллионов"),
        8 => plural_form_scaled(scaled, " септиллион", " септиллиона", " септиллионов"),
        9 => plural_form_scaled(scaled, " октиллион", " октиллиона", " октиллионов"),
        10 => plural_form_scaled(scaled, " нониллион", " нониллиона", " нониллионов"),
        11 => plural_form_scaled(scaled, " дециллион", " дециллиона", " дециллионов"),
        _ => "",
    }
}

pub(crate) fn ordinal_suffix(_n: u128) -> &'static str {
    "-й"
}

pub(crate) fn duration_unit(unit: DurationUnit, count: u128, long: bool) -> &'static str {
    if !long {
        return match unit {
            DurationUnit::Day => "д",
            DurationUnit::Hour => "ч",
            DurationUnit::Minute => "м",
            DurationUnit::Second => "с",
            DurationUnit::Millisecond => "мс",
            DurationUnit::Microsecond => "мкс",
            DurationUnit::Nanosecond => "нс",
        };
    }

    match unit {
        DurationUnit::Day => plural_form_int(count, "день", "дня", "дней"),
        DurationUnit::Hour => plural_form_int(count, "час", "часа", "часов"),
        DurationUnit::Minute => plural_form_int(count, "минута", "минуты", "минут"),
        DurationUnit::Second => plural_form_int(count, "секунда", "секунды", "секунд"),
        DurationUnit::Millisecond => {
            plural_form_int(count, "миллисекунда", "миллисекунды", "миллисекунд")
        }
        DurationUnit::Microsecond => {
            plural_form_int(count, "микросекунда", "микросекунды", "микросекунд")
        }
        DurationUnit::Nanosecond => {
            plural_form_int(count, "наносекунда", "наносекунды", "наносекунд")
        }
    }
}

#[inline]
fn plural_form_scaled(
    value: f64,
    one: &'static str,
    few: &'static str,
    many: &'static str,
) -> &'static str {
    if !is_integer(value) {
        // In this simplified model, fractional values use the "few" form:
        // e.g. "1,5 миллиона".
        return few;
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
    let last_two = n % 100;

    if (11..=14).contains(&last_two) {
        return many;
    }

    match n % 10 {
        1 => one,
        2..=4 => few,
        _ => many,
    }
}

#[inline]
fn is_integer(value: f64) -> bool {
    value == (value as u128) as f64
}

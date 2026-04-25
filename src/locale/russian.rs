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

    fn ordinal_suffix(&self, _n: u128) -> &'static str {
        ordinal_suffix(0)
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
        1 => plural_form(scaled, " тысяча", " тысячи", " тысяч"),
        2 => plural_form(scaled, " миллион", " миллиона", " миллионов"),
        3 => plural_form(scaled, " миллиард", " миллиарда", " миллиардов"),
        4 => plural_form(scaled, " триллион", " триллиона", " триллионов"),
        5 => plural_form(scaled, " квадриллион", " квадриллиона", " квадриллионов"),
        6 => plural_form(scaled, " квинтиллион", " квинтиллиона", " квинтиллионов"),
        7 => plural_form(scaled, " секстиллион", " секстиллиона", " секстиллионов"),
        8 => plural_form(scaled, " септиллион", " септиллиона", " септиллионов"),
        9 => plural_form(scaled, " октиллион", " октиллиона", " октиллионов"),
        10 => plural_form(scaled, " нониллион", " нониллиона", " нониллионов"),
        11 => plural_form(scaled, " дециллион", " дециллиона", " дециллионов"),
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
        DurationUnit::Day => plural_form(count as f64, "день", "дня", "дней"),
        DurationUnit::Hour => plural_form(count as f64, "час", "часа", "часов"),
        DurationUnit::Minute => plural_form(count as f64, "минута", "минуты", "минут"),
        DurationUnit::Second => plural_form(count as f64, "секунда", "секунды", "секунд"),
        DurationUnit::Millisecond => {
            plural_form(count as f64, "миллисекунда", "миллисекунды", "миллисекунд")
        }
        DurationUnit::Microsecond => {
            plural_form(count as f64, "микросекунда", "микросекунды", "микросекунд")
        }
        DurationUnit::Nanosecond => {
            plural_form(count as f64, "наносекунда", "наносекунды", "наносекунд")
        }
    }
}

fn plural_form(
    value: f64,
    one: &'static str,
    few: &'static str,
    many: &'static str,
) -> &'static str {
    if !is_integer(value) {
        return few;
    }

    let value = value as u128;
    let last_two = value % 100;

    if (11..=14).contains(&last_two) {
        return many;
    }

    match value % 10 {
        1 => one,
        2..=4 => few,
        _ => many,
    }
}

fn is_integer(value: f64) -> bool {
    value == (value as u128) as f64
}

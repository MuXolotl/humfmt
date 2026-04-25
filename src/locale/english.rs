use super::DurationUnit;

pub(crate) const MAX_COMPACT_SUFFIX_INDEX: usize = 11;

pub(crate) const SHORT_SUFFIXES: [&str; 12] = [
    "", "K", "M", "B", "T", "Qa", "Qi", "Sx", "Sp", "Oc", "No", "Dc",
];

pub(crate) const LONG_SUFFIXES: [&str; 12] = [
    "",
    " thousand",
    " million",
    " billion",
    " trillion",
    " quadrillion",
    " quintillion",
    " sextillion",
    " septillion",
    " octillion",
    " nonillion",
    " decillion",
];

#[derive(Copy, Clone, Debug, Default)]
pub struct English;

impl super::Locale for English {
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

    fn max_compact_suffix_index(&self) -> usize {
        MAX_COMPACT_SUFFIX_INDEX
    }

    fn and_word(&self) -> &'static str {
        "and"
    }

    fn ago_word(&self) -> &'static str {
        "ago"
    }

    fn duration_unit(&self, unit: DurationUnit, count: u128, long: bool) -> &'static str {
        duration_unit(unit, count, long)
    }

    fn ordinal_suffix(&self, n: u128) -> &'static str {
        ordinal_suffix(n)
    }
}

pub(crate) fn duration_unit(unit: DurationUnit, count: u128, long: bool) -> &'static str {
    match (unit, long) {
        (DurationUnit::Day, false) => "d",
        (DurationUnit::Hour, false) => "h",
        (DurationUnit::Minute, false) => "m",
        (DurationUnit::Second, false) => "s",
        (DurationUnit::Millisecond, false) => "ms",
        (DurationUnit::Microsecond, false) => "us",
        (DurationUnit::Nanosecond, false) => "ns",
        (DurationUnit::Day, true) if count == 1 => "day",
        (DurationUnit::Hour, true) if count == 1 => "hour",
        (DurationUnit::Minute, true) if count == 1 => "minute",
        (DurationUnit::Second, true) if count == 1 => "second",
        (DurationUnit::Millisecond, true) if count == 1 => "millisecond",
        (DurationUnit::Microsecond, true) if count == 1 => "microsecond",
        (DurationUnit::Nanosecond, true) if count == 1 => "nanosecond",
        (DurationUnit::Day, true) => "days",
        (DurationUnit::Hour, true) => "hours",
        (DurationUnit::Minute, true) => "minutes",
        (DurationUnit::Second, true) => "seconds",
        (DurationUnit::Millisecond, true) => "milliseconds",
        (DurationUnit::Microsecond, true) => "microseconds",
        (DurationUnit::Nanosecond, true) => "nanoseconds",
    }
}

pub(crate) fn ordinal_suffix(n: u128) -> &'static str {
    match n % 10 {
        1 if n % 100 != 11 => "st",
        2 if n % 100 != 12 => "nd",
        3 if n % 100 != 13 => "rd",
        _ => "th",
    }
}

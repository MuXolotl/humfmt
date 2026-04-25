#[derive(Copy, Clone, Debug)]
pub enum DurationUnit {
    Day,
    Hour,
    Minute,
    Second,
    Millisecond,
    Microsecond,
    Nanosecond,
}

pub trait Locale: Copy + Clone + Default {
    fn compact_suffix(&self, idx: usize, long: bool) -> &'static str;

    fn compact_suffix_for(&self, idx: usize, scaled: f64, long: bool) -> &'static str {
        let _ = scaled;
        self.compact_suffix(idx, long)
    }

    fn max_compact_suffix_index(&self) -> usize {
        11
    }

    fn decimal_separator(&self) -> char {
        '.'
    }

    fn group_separator(&self) -> char {
        ','
    }

    fn duration_unit(&self, unit: DurationUnit, count: u128, long: bool) -> &'static str {
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

    fn and_word(&self) -> &'static str;
    fn ago_word(&self) -> &'static str;
    fn ordinal_suffix(&self, n: u128) -> &'static str;
}

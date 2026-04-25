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
        1 => plural_form(scaled, " tysiąc", " tysiące", " tysięcy", " tysiąca"),
        2 => plural_form(scaled, " milion", " miliony", " milionów", " miliona"),
        3 => plural_form(scaled, " miliard", " miliardy", " miliardów", " miliarda"),
        4 => plural_form(scaled, " bilion", " biliony", " bilionów", " biliona"),
        5 => plural_form(scaled, " biliard", " biliardy", " biliardów", " biliarda"),
        6 => plural_form(scaled, " trylion", " tryliony", " trylionów", " tryliona"),
        _ => "",
    }
}

pub(crate) fn ordinal_suffix(_n: u128) -> &'static str {
    "."
}

fn plural_form(
    value: f64,
    one: &'static str,
    few: &'static str,
    many: &'static str,
    fraction: &'static str,
) -> &'static str {
    if !is_integer(value) {
        return fraction;
    }

    let value = value as u128;
    let last_two = value % 100;

    if (12..=14).contains(&last_two) {
        return many;
    }

    match value % 10 {
        1 if last_two != 11 => one,
        2..=4 => few,
        _ => many,
    }
}

fn is_integer(value: f64) -> bool {
    value == (value as u128) as f64
}

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

    fn ordinal_suffix(&self, n: u128) -> &'static str {
        ordinal_suffix(n)
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

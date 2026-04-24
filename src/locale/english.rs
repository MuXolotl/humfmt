#[derive(Copy, Clone, Debug, Default)]
pub struct English;

impl super::Locale for English {
    fn compact_suffix(&self, idx: usize, long: bool) -> &'static str {
        match (idx, long) {
            (1, false) => "K",
            (2, false) => "M",
            (3, false) => "B",
            (4, false) => "T",

            (1, true) => " thousand",
            (2, true) => " million",
            (3, true) => " billion",
            (4, true) => " trillion",

            _ => "",
        }
    }

    fn and_word(&self) -> &'static str {
        "and"
    }

    fn ago_word(&self) -> &'static str {
        "ago"
    }

    fn ordinal_suffix(&self, n: u128) -> &'static str {
        match n % 10 {
            1 if n % 100 != 11 => "st",
            2 if n % 100 != 12 => "nd",
            3 if n % 100 != 13 => "rd",
            _ => "th",
        }
    }
}
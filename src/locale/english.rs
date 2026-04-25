#[derive(Copy, Clone, Debug, Default)]
pub struct English;

impl super::Locale for English {
    fn compact_suffix(&self, idx: usize, long: bool) -> &'static str {
        match (idx, long) {
            (1, false) => "K",
            (2, false) => "M",
            (3, false) => "B",
            (4, false) => "T",
            (5, false) => "Qa",
            (6, false) => "Qi",
            (7, false) => "Sx",
            (8, false) => "Sp",
            (9, false) => "Oc",
            (10, false) => "No",
            (11, false) => "Dc",

            (1, true) => " thousand",
            (2, true) => " million",
            (3, true) => " billion",
            (4, true) => " trillion",
            (5, true) => " quadrillion",
            (6, true) => " quintillion",
            (7, true) => " sextillion",
            (8, true) => " septillion",
            (9, true) => " octillion",
            (10, true) => " nonillion",
            (11, true) => " decillion",

            _ => "",
        }
    }

    fn max_compact_suffix_index(&self) -> usize {
        11
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

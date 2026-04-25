#[derive(Copy, Clone, Debug, Default)]
pub struct Russian;

impl super::Locale for Russian {
    fn compact_suffix(&self, idx: usize, long: bool) -> &'static str {
        match (idx, long) {
            (1, false) => " тыс.",
            (2, false) => " млн",
            (3, false) => " млрд",
            (4, false) => " трлн",
            (5, false) => " квадрлн",
            (6, false) => " квинтлн",
            (7, false) => " секстлн",
            (8, false) => " септлн",
            (9, false) => " октлн",
            (10, false) => " нониллн",
            (11, false) => " дециллн",

            (1, true) => " тысяч",
            (2, true) => " миллионов",
            (3, true) => " миллиардов",
            (4, true) => " триллионов",
            (5, true) => " квадриллионов",
            (6, true) => " квинтиллионов",
            (7, true) => " секстиллионов",
            (8, true) => " септиллионов",
            (9, true) => " октиллионов",
            (10, true) => " нониллионов",
            (11, true) => " дециллионов",

            _ => "",
        }
    }

    fn compact_suffix_for(&self, idx: usize, scaled: f64, long: bool) -> &'static str {
        if !long {
            return self.compact_suffix(idx, false);
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

    fn max_compact_suffix_index(&self) -> usize {
        11
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

    fn ordinal_suffix(&self, _n: u128) -> &'static str {
        "-й"
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

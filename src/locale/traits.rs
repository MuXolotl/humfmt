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

    fn and_word(&self) -> &'static str;
    fn ago_word(&self) -> &'static str;
    fn ordinal_suffix(&self, n: u128) -> &'static str;
}

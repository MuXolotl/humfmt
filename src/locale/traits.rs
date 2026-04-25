pub trait Locale: Copy + Clone + Default {
    fn compact_suffix(&self, idx: usize, long: bool) -> &'static str;

    fn max_compact_suffix_index(&self) -> usize {
        11
    }

    fn and_word(&self) -> &'static str;
    fn ago_word(&self) -> &'static str;
    fn ordinal_suffix(&self, n: u128) -> &'static str;
}

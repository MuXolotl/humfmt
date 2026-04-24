pub trait Locale: Copy + Clone + Default {
    fn compact_suffix(&self, idx: usize, long: bool) -> &'static str;
    fn and_word(&self) -> &'static str;
    fn ago_word(&self) -> &'static str;
    fn ordinal_suffix(&self, n: u128) -> &'static str;
}

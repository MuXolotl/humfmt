#![cfg(feature = "russian")]

use humfmt::{locale::Russian, number_with, ordinal_with, NumberOptions};

#[test]
fn formats_short_compact_numbers_in_russian() {
    let opts = NumberOptions::new().locale(Russian);
    assert_eq!(number_with(15_320, opts).to_string(), "15,3 тыс.");
    assert_eq!(number_with(1_500_000, opts).to_string(), "1,5 млн");
}

#[test]
fn formats_long_compact_numbers_with_russian_inflection() {
    let opts = NumberOptions::new().locale(Russian).long_units();
    assert_eq!(number_with(1_000, opts).to_string(), "1 тысяча");
    assert_eq!(number_with(2_000, opts).to_string(), "2 тысячи");
    assert_eq!(number_with(5_000, opts).to_string(), "5 тысяч");
    assert_eq!(number_with(1_500_000, opts).to_string(), "1,5 миллиона");
}

#[test]
fn uses_russian_decimal_separator() {
    let opts = NumberOptions::new().locale(Russian).precision(2);
    assert_eq!(number_with(12.34, opts).to_string(), "12,34");
}

#[test]
fn supports_russian_ordinals() {
    assert_eq!(ordinal_with(21, Russian).to_string(), "21-й");
}

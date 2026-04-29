#![cfg(feature = "polish")]

use humfmt::{locale::Polish, number_with, ordinal_with, NumberOptions};

#[test]
fn formats_short_compact_numbers_in_polish() {
    let opts = NumberOptions::new().locale(Polish);
    assert_eq!(number_with(15_320, opts).to_string(), "15,3 tys.");
    assert_eq!(number_with(1_500_000, opts).to_string(), "1,5 mln");
    assert_eq!(number_with(2_000_000_000_u64, opts).to_string(), "2 mld");
}

#[test]
fn formats_long_compact_numbers_with_polish_inflection() {
    let opts = NumberOptions::new().locale(Polish).long_units();
    assert_eq!(number_with(1_000, opts).to_string(), "1 tysiąc");
    assert_eq!(number_with(2_000, opts).to_string(), "2 tysiące");
    assert_eq!(number_with(5_000, opts).to_string(), "5 tysięcy");
    assert_eq!(number_with(1_500_000, opts).to_string(), "1,5 miliona");
    assert_eq!(number_with(2_000_000, opts).to_string(), "2 miliony");
    assert_eq!(number_with(5_000_000, opts).to_string(), "5 milionów");
}

#[test]
fn polish_long_plural_regressions_for_12_14_21_22_25() {
    let opts = NumberOptions::new().locale(Polish).long_units();

    // 12..=14 always use the many form.
    assert_eq!(number_with(12_000, opts).to_string(), "12 tysięcy");
    assert_eq!(number_with(14_000, opts).to_string(), "14 tysięcy");

    // 21 is NOT "one" in Polish; it uses the many form.
    assert_eq!(number_with(21_000, opts).to_string(), "21 tysięcy");

    // 22 ends in 2 and is not in 12..=14 => few form.
    assert_eq!(number_with(22_000, opts).to_string(), "22 tysiące");

    // 25 => many form.
    assert_eq!(number_with(25_000, opts).to_string(), "25 tysięcy");

    // Same rule set for other magnitudes.
    assert_eq!(number_with(21_000_000, opts).to_string(), "21 milionów");
    assert_eq!(number_with(22_000_000, opts).to_string(), "22 miliony");
    assert_eq!(number_with(25_000_000, opts).to_string(), "25 milionów");
}

#[test]
fn uses_polish_decimal_separator() {
    let opts = NumberOptions::new().locale(Polish).precision(2);
    assert_eq!(number_with(12.34, opts).to_string(), "12,34");
}

#[test]
fn supports_polish_ordinals() {
    assert_eq!(ordinal_with(21, Polish).to_string(), "21.");
}

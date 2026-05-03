use humfmt::{locale::CustomLocale, number_with, ordinal_with, NumberOptions};

fn custom_ordinal(_: u128) -> &'static str {
    "o"
}

fn shard_suffix(idx: usize, scaled: f64, long: bool) -> &'static str {
    match (idx, long) {
        (1, false) => "sh",
        (1, true) if scaled == 1.0 => " shard",
        (1, true) => " shards",
        _ => "",
    }
}

#[test]
fn formats_numbers_with_custom_suffixes_and_separators() {
    let locale = CustomLocale::english()
        .short_suffix(1, "k")
        .long_suffix(1, " thousand-ish")
        .separators(',', '_');

    assert_eq!(
        number_with(15_320, NumberOptions::new().locale(locale)).to_string(),
        "15,3k"
    );
    assert_eq!(
        number_with(15_320, NumberOptions::new().locale(locale).long_units()).to_string(),
        "15,3 thousand-ish"
    );
}

#[test]
fn supports_custom_ordinal_suffixes() {
    let locale = CustomLocale::english().ordinal_suffix_fn(custom_ordinal);
    assert_eq!(ordinal_with(7, locale).to_string(), "7o");
}

#[test]
fn supports_custom_value_aware_compact_suffixes() {
    let locale = CustomLocale::english().compact_suffix_fn(shard_suffix);
    let opts = NumberOptions::new().locale(locale).long_units();

    assert_eq!(number_with(1_000, opts).to_string(), "1 shard");
    assert_eq!(number_with(2_000, opts).to_string(), "2 shards");
}

#[test]
fn can_cap_compact_scaling_with_custom_locale_limits() {
    // Capping the maximum suffix index to 1 (thousands).
    // 12,345,678 will be rendered as 12345.7K instead of 12.3M
    let locale = CustomLocale::english().max_compact_suffix_index(1);

    let opts = NumberOptions::new().locale(locale).precision(1);

    assert_eq!(number_with(12_345_678, opts).to_string(), "12345.7K");
}

#[cfg(feature = "russian")]
#[test]
fn can_customize_from_russian_preset() {
    let locale = CustomLocale::russian().short_suffix(1, " тыс");
    let opts = NumberOptions::new().locale(locale);
    assert_eq!(number_with(15_320, opts).to_string(), "15,3 тыс");
}

#[cfg(feature = "polish")]
#[test]
fn can_customize_from_polish_preset() {
    let locale = CustomLocale::polish().short_suffix(1, " k");
    let opts = NumberOptions::new().locale(locale);
    assert_eq!(number_with(15_320, opts).to_string(), "15,3 k");
}

use humfmt::{list, list_with, locale::CustomLocale, ListOptions};

#[test]
fn formats_empty_lists() {
    assert_eq!(list::<&str>(&[]).to_string(), "");
}

#[test]
fn formats_single_item_lists() {
    assert_eq!(list(&["red"]).to_string(), "red");
}

#[test]
fn formats_two_item_lists() {
    assert_eq!(list(&["red", "green"]).to_string(), "red and green");
}

#[test]
fn formats_three_item_lists_with_english_serial_comma() {
    assert_eq!(
        list(&["red", "green", "blue"]).to_string(),
        "red, green, and blue"
    );
}

#[test]
fn can_disable_serial_comma() {
    let out = list_with(
        &["red", "green", "blue"],
        ListOptions::new().no_serial_comma(),
    );

    assert_eq!(out.to_string(), "red, green and blue");
}

#[test]
fn supports_non_string_display_items() {
    assert_eq!(list(&[1, 2, 3]).to_string(), "1, 2, and 3");
}

#[cfg(feature = "russian")]
#[test]
fn uses_russian_conjunction_without_serial_comma() {
    let out = list_with(
        &["яблоки", "груши", "сливы"],
        ListOptions::new().locale(humfmt::locale::Russian),
    );

    assert_eq!(out.to_string(), "яблоки, груши и сливы");
}

#[cfg(feature = "polish")]
#[test]
fn uses_polish_conjunction_without_serial_comma() {
    let out = list_with(
        &["jabłka", "gruszki", "śliwki"],
        ListOptions::new().locale(humfmt::locale::Polish),
    );

    assert_eq!(out.to_string(), "jabłka, gruszki i śliwki");
}

#[test]
fn supports_custom_conjunction_and_serial_comma_style() {
    let locale = CustomLocale::english().and_word("plus").serial_comma(false);
    let out = list_with(&["red", "green", "blue"], ListOptions::new().locale(locale));

    assert_eq!(out.to_string(), "red, green plus blue");
}

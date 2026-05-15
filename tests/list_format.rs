use humfmt::{list, list_with, ListOptions};

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
fn formats_three_item_lists_with_serial_comma() {
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

#[test]
fn supports_custom_conjunction() {
    let out = list_with(
        &["red", "green", "blue"],
        ListOptions::new().conjunction("plus").no_serial_comma(),
    );

    assert_eq!(out.to_string(), "red, green plus blue");
}

#[test]
fn supports_custom_separator() {
    let out = list_with(
        &["red", "green", "blue"],
        ListOptions::new().separator(" | ").conjunction("&"),
    );

    assert_eq!(out.to_string(), "red | green & blue");
}

#[test]
fn serial_comma_is_ignored_for_non_comma_separators() {
    // Serial comma is a comma-specific stylistic rule. If the user overrides
    // the list separator away from commas, injecting a literal comma becomes
    // surprising.
    let out = list_with(
        &["red", "green", "blue"],
        ListOptions::new()
            .separator(" | ")
            .conjunction("&")
            .serial_comma_enabled(true),
    );

    assert_eq!(out.to_string(), "red | green & blue");
}

#[test]
fn supports_explicit_serial_comma_boolean_setter() {
    let with = list_with(
        &["red", "green", "blue"],
        ListOptions::new().serial_comma_enabled(true),
    );
    let without = list_with(
        &["red", "green", "blue"],
        ListOptions::new().serial_comma_enabled(false),
    );

    assert_eq!(with.to_string(), "red, green, and blue");
    assert_eq!(without.to_string(), "red, green and blue");
}

#[test]
fn conjunction_alone_keeps_default_serial_comma() {
    // After replacing locales with direct fields, conjunction() no longer
    // implicitly affects serial_comma. Verify the default is preserved.
    let out = list_with(
        &["red", "green", "blue"],
        ListOptions::new().conjunction("plus"),
    );

    assert_eq!(out.to_string(), "red, green, plus blue");
}

mod display;
mod format;
mod options;

pub use display::ListDisplay;
pub use options::ListOptions;

/// Creates a natural-language list formatter using the default locale.
///
/// # Examples
///
/// ```rust
/// let out = humfmt::list(&["red", "green", "blue"]);
/// assert_eq!(out.to_string(), "red, green, and blue");
/// ```
pub fn list<T: core::fmt::Display>(items: &[T]) -> ListDisplay<'_, T> {
    ListDisplay::new(items, ListOptions::new())
}

/// Creates a natural-language list formatter with custom options.
///
/// # Examples
///
/// ```rust
/// use humfmt::{list_with, ListOptions};
///
/// let out = list_with(&["red", "green", "blue"], ListOptions::new().no_serial_comma());
/// assert_eq!(out.to_string(), "red, green and blue");
/// ```
pub fn list_with<T: core::fmt::Display, L: crate::locale::Locale>(
    items: &[T],
    options: ListOptions<L>,
) -> ListDisplay<'_, T, L> {
    ListDisplay::new(items, options)
}

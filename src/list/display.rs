use core::fmt;

use crate::locale::{English, Locale};

use super::{format::format_list, ListOptions};

/// `Display` wrapper for natural-language list formatting.
///
/// Instances of this type are created via [`crate::list`] and [`crate::list_with`].
///
/// Example:
///
/// ```rust
/// use humfmt::list;
///
/// assert_eq!(list(&["red", "green", "blue"]).to_string(), "red, green, and blue");
/// ```
#[derive(Copy, Clone)]
pub struct ListDisplay<'a, T, L: Locale = English> {
    items: &'a [T],
    options: ListOptions<L>,
}

impl<'a, T, L: Locale> ListDisplay<'a, T, L> {
    pub(crate) fn new(items: &'a [T], options: ListOptions<L>) -> Self {
        Self { items, options }
    }
}

impl<T: fmt::Display, L: Locale> fmt::Display for ListDisplay<'_, T, L> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        format_list(f, self.items, &self.options)
    }
}

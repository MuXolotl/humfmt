use core::fmt;

use crate::common::fmt::{fmt_with_padding, Render};

use super::{format::format_list, ListOptions};

/// `Display` wrapper for natural-language list formatting.
///
/// Instances of this type are created via [`crate::list()`] and [`crate::list_with`].
///
/// # Examples
///
/// ```rust
/// use humfmt::list;
///
/// assert_eq!(list(&["red", "green", "blue"]).to_string(), "red, green, and blue");
/// ```
#[derive(Copy, Clone)]
pub struct ListDisplay<'a, T> {
    items: &'a [T],
    options: ListOptions,
}

impl<'a, T> ListDisplay<'a, T> {
    pub(crate) fn new(items: &'a [T], options: ListOptions) -> Self {
        Self { items, options }
    }
}

impl<T: fmt::Display> Render for ListDisplay<'_, T> {
    fn render<W: fmt::Write + ?Sized>(&self, f: &mut W) -> fmt::Result {
        format_list(f, self.items, &self.options)
    }
}

impl<T: fmt::Display> fmt::Display for ListDisplay<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt_with_padding(self, f)
    }
}

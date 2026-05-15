//! Natural-language list formatting.
//!
//! # Quick start
//!
//! ```rust
//! use humfmt::{list, list_with, ListOptions};
//!
//! // Default: Oxford comma
//! assert_eq!(list(&["red", "green", "blue"]).to_string(), "red, green, and blue");
//!
//! // No serial comma
//! let no_oxford = list_with(
//!     &["red", "green", "blue"],
//!     ListOptions::new().no_serial_comma(),
//! );
//! assert_eq!(no_oxford.to_string(), "red, green and blue");
//! ```
//!
//! # Edge case behaviour
//!
//! | Input | Default output | Notes |
//! |---:|---|---|
//! | `[]` | `""` | Empty list |
//! | `["red"]` | `"red"` | Single item |
//! | `["red", "green"]` | `"red and green"` | Two items, no comma |
//! | `["red", "green", "blue"]` | `"red, green, and blue"` | Three items, serial comma |
//!
//! # Serial comma and non-comma separators
//!
//! The serial comma (Oxford comma) is only meaningful for comma-style separators.
//! If you override the list separator to something non-comma-like (e.g. `" | "`),
//! `humfmt` will not inject a literal comma before the final conjunction even if
//! serial comma is enabled. This keeps the output predictable.

mod display;
mod format;
mod options;

pub use display::ListDisplay;
pub use options::ListOptions;

/// Creates a natural-language list formatter using default options.
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
/// let out = list_with(
///     &["red", "green", "blue"],
///     ListOptions::new()
///         .serial_comma_enabled(false)
///         .conjunction("plus"),
/// );
/// assert_eq!(out.to_string(), "red, green plus blue");
/// ```
pub fn list_with<T: core::fmt::Display>(items: &[T], options: ListOptions) -> ListDisplay<'_, T> {
    ListDisplay::new(items, options)
}

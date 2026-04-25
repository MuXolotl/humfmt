//! Built-in locales and locale customization tools.
//!
//! The top-level helpers default to English. Switch locale-aware APIs such as
//! `number_with`, `duration_with`, `ago_with`, `list_with`, and `ordinal_with`
//! to another locale when you want localized separators, conjunctions, or unit
//! wording.
//!
//! # Examples
//!
//! ```rust
//! use humfmt::{list_with, number_with, NumberOptions, ListOptions};
//! use humfmt::locale::{CustomLocale, English};
//!
//! let english = number_with(15_320, NumberOptions::new().locale(English));
//! assert_eq!(english.to_string(), "15.3K");
//!
//! let custom = CustomLocale::english().and_word("plus").serial_comma(false);
//! let out = list_with(&["red", "green", "blue"], ListOptions::new().locale(custom));
//! assert_eq!(out.to_string(), "red, green plus blue");
//! ```

mod custom;
mod english;
#[cfg(feature = "polish")]
mod polish;
#[cfg(feature = "russian")]
mod russian;
mod traits;

pub use custom::{CompactSuffixFn, CustomLocale, DurationUnitFn, OrdinalSuffixFn};
pub use english::English;
#[cfg(feature = "polish")]
#[cfg_attr(docsrs, doc(cfg(feature = "polish")))]
pub use polish::Polish;
#[cfg(feature = "russian")]
#[cfg_attr(docsrs, doc(cfg(feature = "russian")))]
pub use russian::Russian;
pub use traits::{DurationUnit, Locale};

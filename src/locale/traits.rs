//! Locale abstraction.
//!
//! Locales control:
//! - compact-number suffixes (`K` / `thousand` / inflected forms)
//! - decimal and grouping separators
//! - list formatting style (`and` word, serial comma, list separator)
//! - duration unit labels (`h` vs `hour`, pluralization)
//! - relative-time wording (`ago` word)
//! - ordinal suffixes (`st` / `.` / `-й`)
//!
//! The crate includes built-in locale packs (English by default, plus optional
//! Russian/Polish behind feature flags), and also provides [`crate::locale::CustomLocale`]
//! for ad hoc customization.
//!
//! You can also implement this trait for your own locale type. Keep in mind that
//! `Locale` requires `Copy + Clone + Default` to keep formatter options cheap.
//!
//! # Implementing a minimal locale
//!
//! ```rust
//! use humfmt::locale::{DurationUnit, Locale};
//!
//! #[derive(Copy, Clone, Debug, Default)]
//! struct Pirate;
//!
//! impl Locale for Pirate {
//!     fn compact_suffix(&self, idx: usize, long: bool) -> &'static str {
//!         let _ = long;
//!         match idx {
//!             0 => "",
//!             1 => "k",
//!             2 => "m",
//!             _ => "",
//!         }
//!     }
//!
//!     fn and_word(&self) -> &'static str {
//!         "arr"
//!     }
//!
//!     fn ago_word(&self) -> &'static str {
//!         "back"
//!     }
//!
//!     fn ordinal_suffix(&self, _n: u128) -> &'static str {
//!         "th"
//!     }
//!
//!     fn duration_unit(&self, unit: DurationUnit, count: u128, long: bool) -> &'static str {
//!         let _ = count;
//!         match (unit, long) {
//!             (DurationUnit::Second, false) => "s",
//!             (DurationUnit::Second, true) => "second",
//!             _ => "?",
//!         }
//!     }
//! }
//! ```
#[derive(Copy, Clone, Debug)]
/// Duration unit kind used by locale-aware duration and relative-time formatting.
///
/// Locales receive a `DurationUnit` plus a `count` and a `long` flag and are
/// expected to return an appropriate unit label.
///
/// This enum intentionally matches the unit set used by `humfmt`'s duration
/// formatter (days down to nanoseconds).
pub enum DurationUnit {
    /// 24-hour day.
    Day,
    /// 60-minute hour.
    Hour,
    /// 60-second minute.
    Minute,
    /// Base unit: seconds.
    Second,
    /// 1/1000 of a second.
    Millisecond,
    /// 1/1_000_000 of a second.
    Microsecond,
    /// 1/1_000_000_000 of a second.
    Nanosecond,
}

/// Locale customization trait.
///
/// This trait is intentionally small and uses `&'static str` outputs to keep
/// formatting allocation-free and `no_std` friendly.
///
/// Most users should use built-in locale packs or [`crate::locale::CustomLocale`]
/// rather than implementing `Locale` directly.
pub trait Locale: Copy + Clone + Default {
    /// Resolves the compact-number suffix for a given magnitude index.
    ///
    /// Index `0` means "no suffix". Index `1` is thousand, `2` is million, and so on.
    ///
    /// If `long` is `true`, the suffix is expected to be a long-form label
    /// (e.g. `" thousand"`). Long-form suffixes should include any leading
    /// whitespace you want preserved in the final output.
    fn compact_suffix(&self, idx: usize, long: bool) -> &'static str;

    /// Value-aware compact suffix resolver.
    ///
    /// This is primarily useful for languages that need inflection based on the
    /// rendered value (e.g. "1 million" vs "2 millions", or more complex rules).
    ///
    /// Default implementation ignores `scaled` and falls back to [`Self::compact_suffix`].
    #[inline]
    fn compact_suffix_for(&self, idx: usize, scaled: f64, long: bool) -> &'static str {
        let _ = scaled;
        self.compact_suffix(idx, long)
    }

    /// Maximum suffix index used for compact-number scaling.
    ///
    /// This allows locale packs to intentionally cap supported magnitudes.
    /// Example: a locale may provide suffixes only up to "trillion".
    #[inline]
    fn max_compact_suffix_index(&self) -> usize {
        11
    }

    /// Decimal separator used for numeric output.
    ///
    /// Example: `'.'` for English, `','` for many European locales.
    #[inline]
    fn decimal_separator(&self) -> char {
        '.'
    }

    /// Group separator used for digit grouping (e.g. `1,234,567`).
    #[inline]
    fn group_separator(&self) -> char {
        ','
    }

    /// Default serial-comma preference for list formatting.
    ///
    /// English typically uses a serial comma (Oxford comma) by default; many
    /// languages do not.
    #[inline]
    fn serial_comma(&self) -> bool {
        false
    }

    /// Separator placed between list items (excluding the final conjunction).
    ///
    /// Default is `", "`.
    #[inline]
    fn list_separator(&self) -> &'static str {
        ", "
    }

    /// Resolves the duration unit label for the given unit kind and count.
    ///
    /// - `count` is the unit quantity (used for pluralization/inflection).
    /// - `long` selects between compact (`"h"`) and long-form (`"hour"`) labels.
    ///
    /// Default implementation provides English labels.
    #[inline]
    fn duration_unit(&self, unit: DurationUnit, count: u128, long: bool) -> &'static str {
        match (unit, long) {
            (DurationUnit::Day, false) => "d",
            (DurationUnit::Hour, false) => "h",
            (DurationUnit::Minute, false) => "m",
            (DurationUnit::Second, false) => "s",
            (DurationUnit::Millisecond, false) => "ms",
            (DurationUnit::Microsecond, false) => "us",
            (DurationUnit::Nanosecond, false) => "ns",
            (DurationUnit::Day, true) if count == 1 => "day",
            (DurationUnit::Hour, true) if count == 1 => "hour",
            (DurationUnit::Minute, true) if count == 1 => "minute",
            (DurationUnit::Second, true) if count == 1 => "second",
            (DurationUnit::Millisecond, true) if count == 1 => "millisecond",
            (DurationUnit::Microsecond, true) if count == 1 => "microsecond",
            (DurationUnit::Nanosecond, true) if count == 1 => "nanosecond",
            (DurationUnit::Day, true) => "days",
            (DurationUnit::Hour, true) => "hours",
            (DurationUnit::Minute, true) => "minutes",
            (DurationUnit::Second, true) => "seconds",
            (DurationUnit::Millisecond, true) => "milliseconds",
            (DurationUnit::Microsecond, true) => "microseconds",
            (DurationUnit::Nanosecond, true) => "nanoseconds",
        }
    }

    /// Conjunction word used for list formatting (English: `"and"`).
    fn and_word(&self) -> &'static str;

    /// Word appended for relative-time formatting (English: `"ago"`).
    fn ago_word(&self) -> &'static str;

    /// Ordinal suffix resolver.
    ///
    /// Examples:
    /// - English: `1 -> "st"`, `2 -> "nd"`, `11 -> "th"`
    /// - Polish: `21 -> "."`
    /// - Russian: `21 -> "-й"`
    fn ordinal_suffix(&self, n: u128) -> &'static str;
}

use crate::locale::{English, Locale};

/// Builder-style configuration for compact number formatting.
///
/// This type is designed to be:
/// - cheap to copy (`Copy`)
/// - easy to chain (builder methods return `Self`)
/// - predictable (values are clamped to small, fixed ranges)
///
/// # Examples
///
/// ```rust
/// use humfmt::NumberOptions;
///
/// let opts = NumberOptions::new()
///     .precision(2)
///     .long_units();
///
/// assert_eq!(humfmt::number_with(15_320, opts).to_string(), "15.32 thousand");
/// ```
#[derive(Copy, Clone, Debug)]
pub struct NumberOptions<L: Locale = English> {
    precision: u8,
    long_units: bool,
    separators: bool,
    locale: L,
}

impl NumberOptions<English> {
    /// Creates default English formatting options.
    ///
    /// Defaults:
    /// - precision: `1`
    /// - long units: `false` (short suffixes like `K`, `M`)
    /// - separators: `false` (no digit grouping by default)
    /// - locale: `English`
    #[inline]
    pub fn new() -> Self {
        Self {
            precision: 1,
            long_units: false,
            separators: false,
            locale: English,
        }
    }
}

impl<L: Locale> Default for NumberOptions<L> {
    #[inline]
    fn default() -> Self {
        Self {
            precision: 1,
            long_units: false,
            separators: false,
            locale: L::default(),
        }
    }
}

impl<L: Locale> NumberOptions<L> {
    /// Sets decimal precision for compact values.
    ///
    /// Precision is clamped to `0..=6`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use humfmt::NumberOptions;
    ///
    /// let opts = NumberOptions::new().precision(2);
    /// assert_eq!(humfmt::number_with(15_320, opts).to_string(), "15.32K");
    /// ```
    #[inline]
    pub fn precision(mut self, n: u8) -> Self {
        self.precision = n.min(6);
        self
    }

    /// Uses long suffixes like `" thousand"` instead of short suffixes like `"K"`.
    ///
    /// Long-form suffixes are locale-controlled and may include inflection rules.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use humfmt::NumberOptions;
    ///
    /// let opts = NumberOptions::new().long_units();
    /// assert_eq!(humfmt::number_with(15_320, opts).to_string(), "15.3 thousand");
    /// ```
    #[inline]
    pub fn long_units(mut self) -> Self {
        self.long_units = true;
        self
    }

    /// Enables or disables digit grouping separators for unscaled output.
    ///
    /// Separators are applied only when the output is *not* compacted (i.e. when
    /// the suffix index is `0`). The actual separator characters come from the
    /// active locale.
    ///
    /// If you want to force unscaled rendering for values that would otherwise
    /// be compacted (like `12_345 -> 12.3K`), use a locale with
    /// `max_compact_suffix_index(0)` (e.g. via [`crate::locale::CustomLocale`]).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use humfmt::{number_with, NumberOptions};
    /// use humfmt::locale::CustomLocale;
    ///
    /// let locale = CustomLocale::english().max_compact_suffix_index(0);
    /// let opts = NumberOptions::new().locale(locale).separators(true);
    ///
    /// assert_eq!(number_with(12_345, opts).to_string(), "12,345");
    /// ```
    #[inline]
    pub fn separators(mut self, yes: bool) -> Self {
        self.separators = yes;
        self
    }

    /// Switches the active locale.
    ///
    /// This affects:
    /// - decimal and grouping separators
    /// - compact suffixes and inflection rules
    /// - maximum scaling index (via `Locale::max_compact_suffix_index`)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use humfmt::{number_with, NumberOptions};
    /// use humfmt::locale::English;
    ///
    /// let opts = NumberOptions::new().locale(English);
    /// assert_eq!(number_with(15_320, opts).to_string(), "15.3K");
    /// ```
    #[inline]
    pub fn locale<N: Locale>(self, locale: N) -> NumberOptions<N> {
        NumberOptions {
            precision: self.precision,
            long_units: self.long_units,
            separators: self.separators,
            locale,
        }
    }

    pub(crate) fn precision_value(&self) -> u8 {
        self.precision
    }

    pub(crate) fn long_units_value(&self) -> bool {
        self.long_units
    }

    pub(crate) fn separators_value(&self) -> bool {
        self.separators
    }

    pub(crate) fn locale_ref(&self) -> &L {
        &self.locale
    }
}

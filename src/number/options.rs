use crate::locale::{English, Locale};

/// Builder-style configuration for compact number formatting.
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
    pub(crate) precision: u8,
    pub(crate) long_units: bool,
    pub(crate) separators: bool,
    pub(crate) fixed_precision: bool,
    pub(crate) locale: L,
}

impl NumberOptions<English> {
    /// Creates default English formatting options.
    ///
    /// Defaults:
    /// - precision: `1`
    /// - long units: `false` (short suffixes like `K`, `M`)
    /// - separators: `false` (no digit grouping)
    /// - fixed precision: `false` (trailing zeros are trimmed)
    /// - locale: `English`
    #[inline]
    pub fn new() -> Self {
        Self {
            precision: 1,
            long_units: false,
            separators: false,
            fixed_precision: false,
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
            fixed_precision: false,
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
    /// Separators apply only when the value is not compacted (suffix index `0`).
    /// Separator characters come from the active locale.
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

    /// Controls whether trailing fractional zeros are preserved.
    ///
    /// - `false` (default): trailing zeros are trimmed — `1.50K` becomes `1.5K`
    /// - `true`: trailing zeros are kept — `1.50K` stays `1.50K`
    ///
    /// This is useful when you need consistent column widths in tables or logs.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use humfmt::NumberOptions;
    ///
    /// let trimmed = NumberOptions::new().precision(2);
    /// assert_eq!(humfmt::number_with(1_500, trimmed).to_string(), "1.5K");
    ///
    /// let fixed = NumberOptions::new().precision(2).fixed_precision(true);
    /// assert_eq!(humfmt::number_with(1_500, fixed).to_string(), "1.50K");
    /// ```
    #[inline]
    pub fn fixed_precision(mut self, yes: bool) -> Self {
        self.fixed_precision = yes;
        self
    }

    /// Switches the active locale.
    ///
    /// Affects decimal and grouping separators, compact suffixes, inflection
    /// rules, and maximum scaling index.
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
            fixed_precision: self.fixed_precision,
            locale,
        }
    }
}

use crate::locale::{English, Locale};

/// Builder-style configuration for percentage formatting.
///
/// # Examples
///
/// ```rust
/// use humfmt::PercentOptions;
///
/// let opts = PercentOptions::new().precision(2);
/// assert_eq!(humfmt::percent_with(0.4236, opts).to_string(), "42.36%");
/// ```
#[derive(Copy, Clone, Debug)]
pub struct PercentOptions<L: Locale = English> {
    pub(crate) precision: u8,
    pub(crate) fixed_precision: bool,
    pub(crate) locale: L,
}

impl PercentOptions<English> {
    /// Creates default percentage formatting options.
    ///
    /// Defaults:
    /// - precision: `1`
    /// - fixed precision: `false` (trailing zeros are trimmed)
    /// - locale: `English`
    #[inline]
    pub fn new() -> Self {
        Self {
            precision: 1,
            fixed_precision: false,
            locale: English,
        }
    }
}

impl<L: Locale> Default for PercentOptions<L> {
    #[inline]
    fn default() -> Self {
        Self {
            precision: 1,
            fixed_precision: false,
            locale: L::default(),
        }
    }
}

impl<L: Locale> PercentOptions<L> {
    /// Sets decimal precision for the percentage value.
    ///
    /// Precision is clamped to `0..=6`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use humfmt::PercentOptions;
    ///
    /// let opts = PercentOptions::new().precision(2);
    /// assert_eq!(humfmt::percent_with(0.4236, opts).to_string(), "42.36%");
    /// ```
    #[inline]
    pub fn precision(mut self, n: u8) -> Self {
        self.precision = n.min(6);
        self
    }

    /// Controls whether trailing fractional zeros are preserved.
    ///
    /// - `false` (default): trailing zeros are trimmed — `42.50%` becomes `42.5%`
    /// - `true`: trailing zeros are kept — `42.50%` stays `42.50%`
    ///
    /// Useful for consistent column widths in tables or dashboards.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use humfmt::PercentOptions;
    ///
    /// let trimmed = PercentOptions::new().precision(2);
    /// assert_eq!(humfmt::percent_with(0.425, trimmed).to_string(), "42.5%");
    ///
    /// let fixed = PercentOptions::new().precision(2).fixed_precision(true);
    /// assert_eq!(humfmt::percent_with(0.425, fixed).to_string(), "42.50%");
    /// ```
    #[inline]
    pub fn fixed_precision(mut self, yes: bool) -> Self {
        self.fixed_precision = yes;
        self
    }

    /// Switches the active locale.
    ///
    /// Affects the decimal separator used in the output.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use humfmt::{percent_with, PercentOptions};
    /// use humfmt::locale::CustomLocale;
    ///
    /// let locale = CustomLocale::english().decimal_separator(',');
    /// let opts = PercentOptions::new().precision(1).locale(locale);
    /// assert_eq!(percent_with(0.423, opts).to_string(), "42,3%");
    /// ```
    #[inline]
    pub fn locale<N: Locale>(self, locale: N) -> PercentOptions<N> {
        PercentOptions {
            precision: self.precision,
            fixed_precision: self.fixed_precision,
            locale,
        }
    }
}

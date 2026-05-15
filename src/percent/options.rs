/// Builder-style configuration for percentage formatting.
///
/// # Quick reference
///
/// | Method | Default | Effect |
/// |---|---|---|
/// | [`precision(n)`] | `1` | Decimal places for the percentage value |
/// | [`force_sign(bool)`] | `false` | `0.42` -> `"+42%"` |
/// | [`fixed_precision(bool)`] | `false` | `"42.5%"` -> `"42.50%"` |
/// | [`decimal_separator(c)`] | `'.'` | Decimal separator character |
///
/// [`precision(n)`]: PercentOptions::precision
/// [`force_sign(bool)`]: PercentOptions::force_sign
/// [`fixed_precision(bool)`]: PercentOptions::fixed_precision
/// [`decimal_separator(c)`]: PercentOptions::decimal_separator
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
pub struct PercentOptions {
    pub(crate) precision: u8,
    pub(crate) force_sign: bool,
    pub(crate) fixed_precision: bool,
    pub(crate) decimal_separator: char,
}

impl PercentOptions {
    /// Creates default percentage formatting options.
    ///
    /// Defaults:
    /// - precision: `1`
    /// - force sign: `false`
    /// - fixed precision: `false` (trailing zeros are trimmed)
    /// - decimal separator: `'.'`
    #[inline]
    pub const fn new() -> Self {
        Self {
            precision: 1,
            force_sign: false,
            fixed_precision: false,
            decimal_separator: '.',
        }
    }

    /// Sets decimal precision for the percentage value.
    ///
    /// Precision is clamped to `0..=6`.
    ///
    /// # Behaviour table
    ///
    /// | Input | `precision(0)` | `precision(1)` (default) | `precision(2)` |
    /// |---:|---|---|---|
    /// | `0.423` | `"42%"` | `"42.3%"` | `"42.3%"` (trimmed) |
    /// | `0.4236` | `"42%"` | `"42.4%"` | `"42.36%"` |
    /// | `0.425` | `"43%"` | `"42.5%"` | `"42.5%"` (trimmed) |
    /// | `0.5` | `"50%"` | `"50%"` | `"50%"` (trimmed) |
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
    pub const fn precision(mut self, n: u8) -> Self {
        let n = if n > 6 { 6 } else { n };
        self.precision = n;
        self
    }

    /// Forces the output of a `+` sign for strictly positive values.
    ///
    /// Values that round to exactly zero output `0%` without a sign.
    /// Useful for deltas and change indicators.
    ///
    /// # Behaviour table
    ///
    /// | Input | `force_sign(false)` (default) | `force_sign(true)` |
    /// |---:|---|---|
    /// | `0.42` | `"42%"` | `"+42%"` |
    /// | `0.0` | `"0%"` | `"0%"` (no sign on zero) |
    /// | `-0.42` | `"-42%"` | `"-42%"` (negatives unchanged) |
    /// | `0.0004` (rounds to 0) | `"0%"` | `"0%"` (no sign on rounded-zero) |
    ///
    /// # Examples
    ///
    /// ```rust
    /// use humfmt::PercentOptions;
    ///
    /// let opts = PercentOptions::new().force_sign(true);
    /// assert_eq!(humfmt::percent_with(0.42_f64, opts).to_string(), "+42%");
    /// assert_eq!(humfmt::percent_with(-0.42_f64, opts).to_string(), "-42%");
    /// assert_eq!(humfmt::percent_with(0.0_f64, opts).to_string(), "0%");
    /// ```
    #[inline]
    pub const fn force_sign(mut self, yes: bool) -> Self {
        self.force_sign = yes;
        self
    }

    /// Controls whether trailing fractional zeros are preserved.
    ///
    /// - `false` (default): trailing zeros are trimmed (`42.50%` -> `42.5%`).
    /// - `true`: trailing zeros are kept (`42.50%` stays `42.50%`).
    ///
    /// Useful for consistent column widths in tables or dashboards.
    ///
    /// # Behaviour table
    ///
    /// | Input | `precision(2)` trimmed | `precision(2)` fixed |
    /// |---:|---|---|
    /// | `0.5` | `"50%"` | `"50.00%"` |
    /// | `0.425` | `"42.5%"` | `"42.50%"` |
    /// | `0.4236` | `"42.36%"` | `"42.36%"` |
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
    pub const fn fixed_precision(mut self, yes: bool) -> Self {
        self.fixed_precision = yes;
        self
    }

    /// Overrides the decimal separator.
    ///
    /// Default is `'.'`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use humfmt::PercentOptions;
    ///
    /// let opts = PercentOptions::new().precision(1).decimal_separator(',');
    /// assert_eq!(humfmt::percent_with(0.423_f64, opts).to_string(), "42,3%");
    /// ```
    #[inline]
    pub const fn decimal_separator(mut self, sep: char) -> Self {
        self.decimal_separator = sep;
        self
    }
}

impl Default for PercentOptions {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

use crate::RoundingMode;

/// Builder-style configuration for percentage formatting.
///
/// | Method | Default | Effect |
/// |---|---|---|
/// | [`precision(n)`] | `1` | Decimal places |
/// | [`rounding(mode)`] | `HalfUp` | `HalfUp`, `Floor`, `Ceil` |
/// | [`force_sign(bool)`] | `false` | `0.42` -> `"+42%"` |
/// | [`fixed_precision(bool)`] | `false` | `"42.5%"` -> `"42.50%"` |
/// | [`decimal_separator(c)`] | `'.'` | Decimal point character |
///
/// [`precision(n)`]: PercentOptions::precision
/// [`rounding(mode)`]: PercentOptions::rounding
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
    pub(crate) rounding: RoundingMode,
    pub(crate) force_sign: bool,
    pub(crate) fixed_precision: bool,
    pub(crate) decimal_separator: char,
}

impl PercentOptions {
    /// Creates default percentage formatting options.
    ///
    /// Defaults: precision `1`, rounding `HalfUp`, no forced sign,
    /// trailing zeros trimmed, decimal separator `'.'`.
    #[inline]
    pub const fn new() -> Self {
        Self {
            precision: 1,
            rounding: RoundingMode::HalfUp,
            force_sign: false,
            fixed_precision: false,
            decimal_separator: '.',
        }
    }

    /// Sets the number of decimal places in the output.
    ///
    /// Clamped to `0..=6`. Trailing zeros are trimmed unless
    /// [`fixed_precision`](PercentOptions::fixed_precision) is enabled.
    ///
    /// | Input | `precision(0)` | `precision(1)` (default) | `precision(2)` |
    /// |---:|---|---|---|
    /// | `0.423` | `"42%"` | `"42.3%"` | `"42.3%"` (trimmed) |
    /// | `0.4236` | `"42%"` | `"42.4%"` | `"42.36%"` |
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
        self.precision = if n > 6 { 6 } else { n };
        self
    }

    /// Sets the rounding direction.
    ///
    /// - `HalfUp` (default): ties round away from zero.
    /// - `Floor`: always towards negative infinity.
    /// - `Ceil`: always towards positive infinity.
    ///
    /// | Input | `HalfUp` | `Floor` | `Ceil` |
    /// |---:|---|---|---|
    /// | `0.425` | `"42.5%"` | `"42.5%"` | `"42.5%"` |
    /// | `0.4251` | `"42.5%"` | `"42.5%"` | `"42.6%"` |
    /// | `-0.425` | `"-42.5%"` | `"-42.5%"` | `"-42.5%"` |
    ///
    /// # Examples
    ///
    /// ```rust
    /// use humfmt::{PercentOptions, RoundingMode};
    ///
    /// let opts = PercentOptions::new().precision(0).rounding(RoundingMode::Floor);
    /// assert_eq!(humfmt::percent_with(0.429_f64, opts).to_string(), "42%");
    /// ```
    #[inline]
    pub const fn rounding(mut self, mode: RoundingMode) -> Self {
        self.rounding = mode;
        self
    }

    /// Forces a `+` sign for positive non-zero values.
    ///
    /// Values that round to zero always output `0%` with no sign.
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

    /// Keeps trailing fractional zeros for consistent column widths.
    ///
    /// | Input | trimmed (default) | fixed |
    /// |---:|---|---|
    /// | `0.5` with `precision(2)` | `"50%"` | `"50.00%"` |
    /// | `0.425` with `precision(2)` | `"42.5%"` | `"42.50%"` |
    ///
    /// # Examples
    ///
    /// ```rust
    /// use humfmt::PercentOptions;
    ///
    /// let opts = PercentOptions::new().precision(2).fixed_precision(true);
    /// assert_eq!(humfmt::percent_with(0.5, opts).to_string(), "50.00%");
    /// ```
    #[inline]
    pub const fn fixed_precision(mut self, yes: bool) -> Self {
        self.fixed_precision = yes;
        self
    }

    /// Overrides the decimal separator character (default `'.'`).
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

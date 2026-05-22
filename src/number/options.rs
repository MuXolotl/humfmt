use crate::RoundingMode;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(crate) enum Precision {
    Decimals(u8),
    Significant(u8),
}

/// Builder-style configuration for compact number formatting.
///
/// # Quick reference
///
/// | Method | Default | Effect |
/// |---|---|---|
/// | [`precision(n)`] | `1` | Decimal places for the scaled fractional part |
/// | [`significant_digits(n)`] | `none` | Total significant digits (overrides precision) |
/// | [`compact(bool)`] | `true` | `1500` -> `"1.5K"` vs `"1500"` |
/// | [`force_sign(bool)`] | `false` | `1500` -> `"+1.5K"` |
/// | [`rounding(mode)`] | `HalfUp` | `HalfUp`, `Floor`, `Ceil` |
/// | [`long_units()`] | `false` | `"15.3K"` -> `"15.3 thousand"` |
/// | [`separators(bool)`] | `false` | `"1234"` -> `"1,234"` (when uncompacted) |
/// | [`fixed_precision(bool)`] | `false` | `"1.5K"` -> `"1.50K"` |
/// | [`decimal_separator(c)`] | `'.'` | Decimal separator character |
/// | [`group_separator(c)`] | `','` | Digit grouping separator character |
///
/// [`precision(n)`]: NumberOptions::precision
/// [`significant_digits(n)`]: NumberOptions::significant_digits
/// [`compact(bool)`]: NumberOptions::compact
/// [`force_sign(bool)`]: NumberOptions::force_sign
/// [`rounding(mode)`]: NumberOptions::rounding
/// [`long_units()`]: NumberOptions::long_units
/// [`separators(bool)`]: NumberOptions::separators
/// [`fixed_precision(bool)`]: NumberOptions::fixed_precision
/// [`decimal_separator(c)`]: NumberOptions::decimal_separator
/// [`group_separator(c)`]: NumberOptions::group_separator
///
/// # Examples
///
/// ```rust
/// use humfmt::NumberOptions;
///
/// let opts = NumberOptions::new().precision(2).long_units();
/// assert_eq!(humfmt::number_with(15_320, opts).to_string(), "15.32 thousand");
/// ```
#[derive(Copy, Clone, Debug)]
pub struct NumberOptions {
    pub(crate) precision: Precision,
    pub(crate) compact: bool,
    pub(crate) force_sign: bool,
    pub(crate) rounding: RoundingMode,
    pub(crate) long_units: bool,
    pub(crate) separators: bool,
    pub(crate) fixed_precision: bool,
    pub(crate) decimal_separator: char,
    pub(crate) group_separator: char,
}

impl NumberOptions {
    /// Creates default formatting options.
    ///
    /// | Option | Default |
    /// |---|---|
    /// | precision | `1` |
    /// | compact | `true` |
    /// | force sign | `false` |
    /// | rounding | `HalfUp` |
    /// | long units | `false` (short suffixes: `K`, `M`, ...) |
    /// | separators | `false` (no digit grouping) |
    /// | fixed precision | `false` (trailing zeros trimmed) |
    /// | decimal separator | `'.'` |
    /// | group separator | `','` |
    #[inline]
    pub const fn new() -> Self {
        Self {
            precision: Precision::Decimals(1),
            compact: true,
            force_sign: false,
            rounding: RoundingMode::HalfUp,
            long_units: false,
            separators: false,
            fixed_precision: false,
            decimal_separator: '.',
            group_separator: ',',
        }
    }

    /// Sets the number of decimal places shown in the scaled fractional part.
    ///
    /// Precision is clamped to `0..=6`.
    ///
    /// Trailing zeros are trimmed by default. Use [`fixed_precision(true)`] to
    /// keep them for consistent column widths.
    ///
    /// [`fixed_precision(true)`]: NumberOptions::fixed_precision
    ///
    /// # Behaviour table
    ///
    /// | Input | `precision(0)` | `precision(1)` (default) | `precision(2)` |
    /// |---:|---|---|---|
    /// | `1_400` | `"1K"` | `"1.4K"` | `"1.4K"` (trimmed) |
    /// | `1_500` | `"2K"` | `"1.5K"` | `"1.5K"` (trimmed) |
    /// | `15_320` | `"15K"` | `"15.3K"` | `"15.32K"` |
    /// | `999_950` | `"1M"` | `"1M"` | `"1M"` (rescaled after rounding) |
    ///
    /// # Examples
    ///
    /// ```rust
    /// use humfmt::NumberOptions;
    ///
    /// assert_eq!(humfmt::number_with(15_320, NumberOptions::new().precision(0)).to_string(), "15K");
    /// assert_eq!(humfmt::number_with(15_320, NumberOptions::new().precision(2)).to_string(), "15.32K");
    /// ```
    #[inline]
    pub const fn precision(mut self, n: u8) -> Self {
        let n = if n > 6 { 6 } else { n };
        self.precision = Precision::Decimals(n);
        self
    }

    /// Sets the total number of significant digits to display.
    ///
    /// The requested value is clamped to `1..=39`, matching the maximum number
    /// of decimal digits in a `u128`.
    ///
    /// Important: fractional output still follows the formatter-wide precision
    /// cap of 6 decimal places. This means high significant-digit requests are
    /// most useful for large uncompacted integer output, while compact decimal
    /// output will never print more than 6 digits after the decimal separator.
    ///
    /// # Behaviour table
    ///
    /// | Input | `significant_digits(3)` | Notes |
    /// |---:|---|---|
    /// | `1_234` | `"1.23K"` | `1`, `2`, `3` are the 3 significant digits |
    /// | `12_345` | `"12.3K"` | `1`, `2`, `3` are the 3 significant digits |
    /// | `123_456` | `"123K"` | `1`, `2`, `3` are the 3 significant digits |
    /// | `1_234` (unscaled) | `"1230"` | Unscaled integer is rounded directly |
    ///
    /// With [`fixed_precision(true)`](NumberOptions::fixed_precision), trailing
    /// zeros are padded:
    ///
    /// | Input | `significant_digits(3)` + `fixed_precision` |
    /// |---:|---|
    /// | `1` | `"1.00"` |
    /// | `10` | `"10.0"` |
    /// | `100` | `"100"` |
    ///
    /// # Examples
    ///
    /// ```rust
    /// use humfmt::NumberOptions;
    ///
    /// let opts = NumberOptions::new().significant_digits(3);
    /// assert_eq!(humfmt::number_with(1234, opts).to_string(), "1.23K");
    /// assert_eq!(humfmt::number_with(12345, opts).to_string(), "12.3K");
    /// ```
    ///
    /// High significant-digit requests are clamped but still respect the
    /// 6-decimal-place fractional output cap:
    ///
    /// ```rust
    /// use humfmt::NumberOptions;
    ///
    /// let opts = NumberOptions::new().significant_digits(39);
    /// assert_eq!(humfmt::number_with(1_234_567, opts).to_string(), "1.234567M");
    /// ```
    #[inline]
    pub const fn significant_digits(mut self, n: u8) -> Self {
        let n = if n < 1 {
            1
        } else if n > 39 {
            39
        } else {
            n
        };

        self.precision = Precision::Significant(n);
        self
    }

    /// Controls whether the number should be compacted using magnitude suffixes.
    ///
    /// - `true` (default): Values >= 1,000 are compacted (`1500` -> `"1.5K"`).
    /// - `false`: Values are rendered fully unscaled (`1500` -> `"1500"`).
    ///
    /// Disabling compaction is extremely useful when combined with
    /// [`separators(true)`] to output fully formatted large numbers like
    /// `"1,234,567"`.
    ///
    /// [`separators(true)`]: NumberOptions::separators
    ///
    /// # Behaviour table
    ///
    /// | Input | `compact(true)` (default) | `compact(false)` |
    /// |---:|---|---|
    /// | `999` | `"999"` | `"999"` |
    /// | `1500` | `"1.5K"` | `"1500"` |
    /// | `1_500_000` | `"1.5M"` | `"1500000"` |
    /// | `1_500_000.5` (f64) | `"1.5M"` | `"1500000.5"` |
    ///
    /// # Examples
    ///
    /// ```rust
    /// use humfmt::NumberOptions;
    ///
    /// let opts = NumberOptions::new().compact(false).separators(true);
    /// assert_eq!(humfmt::number_with(1_234_567, opts).to_string(), "1,234,567");
    /// ```
    #[inline]
    pub const fn compact(mut self, enabled: bool) -> Self {
        self.compact = enabled;
        self
    }

    /// Forces the output of a `+` sign for strictly positive values.
    ///
    /// Values that round to exactly zero will output `0` without a sign.
    /// Useful for deltas and change indicators.
    ///
    /// # Behaviour table
    ///
    /// | Input | `force_sign(false)` (default) | `force_sign(true)` |
    /// |---:|---|---|
    /// | `1500` | `"1.5K"` | `"+1.5K"` |
    /// | `42` | `"42"` | `"+42"` |
    /// | `0` | `"0"` | `"0"` (no sign on zero) |
    /// | `-1500` | `"-1.5K"` | `"-1.5K"` (negatives unchanged) |
    /// | `0.004` (f64, rounds to 0) | `"0"` | `"0"` (no sign on rounded-zero) |
    ///
    /// # Examples
    ///
    /// ```rust
    /// use humfmt::NumberOptions;
    ///
    /// let opts = NumberOptions::new().force_sign(true);
    /// assert_eq!(humfmt::number_with(1500, opts).to_string(), "+1.5K");
    /// assert_eq!(humfmt::number_with(-1500, opts).to_string(), "-1.5K");
    /// assert_eq!(humfmt::number_with(0, opts).to_string(), "0");
    /// ```
    #[inline]
    pub const fn force_sign(mut self, yes: bool) -> Self {
        self.force_sign = yes;
        self
    }

    /// Sets the rounding direction for values that require precision cutoff.
    ///
    /// - `HalfUp` (default): standard mathematical rounding. Ties round away from zero.
    /// - `Floor`: always round towards negative infinity.
    /// - `Ceil`: always round towards positive infinity.
    ///
    /// Rounding may rescale across a suffix boundary. For example, `999_500`
    /// at `precision(0)` with `HalfUp` rounds to `1000K` which rescales to `1M`.
    ///
    /// # Behaviour table
    ///
    /// | Input | `precision(0)` + `HalfUp` | `Floor` | `Ceil` |
    /// |---:|---|---|---|
    /// | `1_100` | `"1K"` | `"1K"` | `"2K"` |
    /// | `1_500` | `"2K"` | `"1K"` | `"2K"` |
    /// | `1_900` | `"2K"` | `"1K"` | `"2K"` |
    /// | `999_500` | `"1M"` (rescaled) | `"999K"` | `"1M"` (rescaled) |
    /// | `-1_500` | `"-2K"` | `"-2K"` | `"-1K"` |
    ///
    /// # Examples
    ///
    /// ```rust
    /// use humfmt::{NumberOptions, RoundingMode};
    ///
    /// let floor = NumberOptions::new().precision(0).rounding(RoundingMode::Floor);
    /// assert_eq!(humfmt::number_with(1_900, floor).to_string(), "1K");
    /// ```
    #[inline]
    pub const fn rounding(mut self, mode: RoundingMode) -> Self {
        self.rounding = mode;
        self
    }

    /// Uses long-form suffix labels instead of short ones.
    ///
    /// `"K"` -> `" thousand"`, `"M"` -> `" million"`, and so on.
    ///
    /// # Behaviour table
    ///
    /// | Input | Short (default) | Long |
    /// |---:|---|---|
    /// | `999` | `"999"` | `"999"` |
    /// | `1_000` | `"1K"` | `"1 thousand"` |
    /// | `1_500` | `"1.5K"` | `"1.5 thousand"` |
    /// | `1_000_000` | `"1M"` | `"1 million"` |
    ///
    /// # Examples
    ///
    /// ```rust
    /// use humfmt::NumberOptions;
    ///
    /// assert_eq!(humfmt::number_with(15_320, NumberOptions::new().long_units()).to_string(), "15.3 thousand");
    /// ```
    #[inline]
    pub const fn long_units(mut self) -> Self {
        self.long_units = true;
        self
    }

    /// Enables digit grouping separators for unscaled output.
    ///
    /// **Important:** grouping separators apply **only when the value is not
    /// compacted** — that is, when the output has no suffix.
    /// For compacted output like `"15.3K"` the integer part is always small (`15`)
    /// and grouping would never trigger anyway.
    ///
    /// To show grouped digits for large numbers, disable compact scaling
    /// via [`compact(false)`].
    ///
    /// [`compact(false)`]: NumberOptions::compact
    ///
    /// # Behaviour table
    ///
    /// | Input | `separators(false)` | `separators(true)` |
    /// |---:|---|---|
    /// | `999` | `"999"` | `"999"` |
    /// | `1_234` | `"1.2K"` | `"1.2K"` (compacted, grouping has no effect) |
    /// | `1_234` with `compact(false)` | `"1234"` | `"1,234"` |
    /// | `1_234_567` with `compact(false)` | `"1234567"` | `"1,234,567"` |
    /// | `-1_234_567` with `compact(false)` | `"-1234567"` | `"-1,234,567"` |
    ///
    /// # Examples
    ///
    /// ```rust
    /// use humfmt::{number_with, NumberOptions};
    ///
    /// let opts = NumberOptions::new().compact(false).separators(true);
    /// assert_eq!(number_with(1_234_567, opts).to_string(), "1,234,567");
    /// ```
    #[inline]
    pub const fn separators(mut self, yes: bool) -> Self {
        self.separators = yes;
        self
    }

    /// Controls whether trailing fractional zeros are preserved.
    ///
    /// - `false` (default): trailing zeros are trimmed (`"1.50K"` -> `"1.5K"`).
    /// - `true`: trailing zeros are kept (`"1.50K"` stays `"1.50K"`).
    ///
    /// Useful for consistent column widths in tables, logs, and dashboards.
    ///
    /// # Behaviour table
    ///
    /// | Input | `precision(2)` trimmed | `precision(2)` fixed |
    /// |---:|---|---|
    /// | `1_000` | `"1K"` | `"1.00K"` |
    /// | `1_500` | `"1.5K"` | `"1.50K"` |
    /// | `1_540` | `"1.54K"` | `"1.54K"` |
    /// | `1_000_000` | `"1M"` | `"1.00M"` |
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
    /// use humfmt::NumberOptions;
    ///
    /// let opts = NumberOptions::new().precision(2).decimal_separator(',');
    /// assert_eq!(humfmt::number_with(1.5_f64, opts).to_string(), "1,5");
    /// ```
    #[inline]
    pub const fn decimal_separator(mut self, sep: char) -> Self {
        self.decimal_separator = sep;
        self
    }

    /// Overrides the digit grouping separator.
    ///
    /// Default is `','`. Only takes effect when [`separators(true)`] is also set
    /// and the value is uncompacted.
    ///
    /// [`separators(true)`]: NumberOptions::separators
    ///
    /// # Examples
    ///
    /// ```rust
    /// use humfmt::NumberOptions;
    ///
    /// let opts = NumberOptions::new()
    ///     .compact(false)
    ///     .separators(true)
    ///     .group_separator(' ');
    /// assert_eq!(humfmt::number_with(1_234_567, opts).to_string(), "1 234 567");
    /// ```
    #[inline]
    pub const fn group_separator(mut self, sep: char) -> Self {
        self.group_separator = sep;
        self
    }
}

impl Default for NumberOptions {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

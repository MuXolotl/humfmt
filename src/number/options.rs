use crate::locale::{English, Locale};
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
/// | [`compact(bool)`] | `true` | `"1500"` → `"1.5K"` vs `"1500"` |
/// | [`rounding(mode)`] | `HalfUp` | HalfUp, Floor, Ceil behaviour |
/// | [`long_units()`] | `false` | `"15.3K"` → `"15.3 thousand"` |
/// | [`separators(bool)`] | `false` | `"1234"` → `"1,234"` (when unscaled or uncompacted) |
/// | [`fixed_precision(bool)`] | `false` | `"1.5K"` → `"1.50K"` |
/// | [`locale(L)`] | `English` | Separators, suffixes, inflection rules |
///
/// [`precision(n)`]: NumberOptions::precision
/// [`significant_digits(n)`]: NumberOptions::significant_digits
/// [`compact(bool)`]: NumberOptions::compact
/// [`rounding(mode)`]: NumberOptions::rounding
/// [`long_units()`]: NumberOptions::long_units
/// [`separators(bool)`]: NumberOptions::separators
/// [`fixed_precision(bool)`]: NumberOptions::fixed_precision
/// [`locale(L)`]: NumberOptions::locale
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
    pub(crate) precision: Precision,
    pub(crate) compact: bool,
    pub(crate) rounding: RoundingMode,
    pub(crate) long_units: bool,
    pub(crate) separators: bool,
    pub(crate) fixed_precision: bool,
    pub(crate) locale: L,
}

impl NumberOptions<English> {
    /// Creates default English formatting options.
    ///
    /// | Option | Default |
    /// |---|---|
    /// | precision | `1` |
    /// | compact | `true` |
    /// | rounding | `HalfUp` |
    /// | long units | `false` (short suffixes: `K`, `M`, …) |
    /// | separators | `false` (no digit grouping) |
    /// | fixed precision | `false` (trailing zeros trimmed) |
    /// | locale | `English` |
    #[inline]
    pub fn new() -> Self {
        Self {
            precision: Precision::Decimals(1),
            compact: true,
            rounding: RoundingMode::HalfUp,
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
            precision: Precision::Decimals(1),
            compact: true,
            rounding: RoundingMode::HalfUp,
            long_units: false,
            separators: false,
            fixed_precision: false,
            locale: L::default(),
        }
    }
}

impl<L: Locale> NumberOptions<L> {
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
    /// | `1_400` | `"1K"` | `"1.4K"` | `"1.40K"` → `"1.4K"` (trimmed) |
    /// | `1_500` | `"2K"` | `"1.5K"` | `"1.50K"` → `"1.5K"` (trimmed) |
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
    pub fn precision(mut self, n: u8) -> Self {
        self.precision = Precision::Decimals(n.min(6));
        self
    }

    /// Sets the total number of significant digits to display.
    ///
    /// This provides an alternative to fixed decimal places, ensuring that the
    /// output always maintains a stable level of precision regardless of magnitude.
    ///
    /// Clamped to `1..=39` (the maximum digits in a `u128`).
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
    /// # Examples
    ///
    /// ```rust
    /// use humfmt::NumberOptions;
    ///
    /// let opts = NumberOptions::new().significant_digits(3);
    /// assert_eq!(humfmt::number_with(1234, opts).to_string(), "1.23K");
    /// assert_eq!(humfmt::number_with(12345, opts).to_string(), "12.3K");
    /// ```
    #[inline]
    pub fn significant_digits(mut self, n: u8) -> Self {
        self.precision = Precision::Significant(n.clamp(1, 39));
        self
    }

    /// Controls whether the number should be compacted using magnitude suffixes (e.g. `K`, `M`).
    ///
    /// - `true` (default): Values >= 1,000 are compacted (`1500` → `"1.5K"`).
    /// - `false`: Values are rendered fully unscaled (`1500` → `"1500"`).
    ///
    /// Disabling compaction is extremely useful when combined with [`separators(true)`]
    /// to output fully formatted large numbers like `"1,234,567"`.
    ///
    /// [`separators(true)`]: NumberOptions::separators
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
    pub fn compact(mut self, enabled: bool) -> Self {
        self.compact = enabled;
        self
    }

    /// Sets the rounding direction for values that require precision cutoff.
    ///
    /// - `HalfUp` (default): standard mathematical rounding. Ties round away from zero.
    /// - `Floor`: always round towards negative infinity.
    /// - `Ceil`: always round towards positive infinity.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use humfmt::{NumberOptions, RoundingMode};
    ///
    /// let floor = NumberOptions::new().precision(0).rounding(RoundingMode::Floor);
    /// assert_eq!(humfmt::number_with(1_900, floor).to_string(), "1K");
    ///
    /// let ceil = NumberOptions::new().precision(0).rounding(RoundingMode::Ceil);
    /// assert_eq!(humfmt::number_with(1_100, ceil).to_string(), "2K");
    /// ```
    #[inline]
    pub fn rounding(mut self, mode: RoundingMode) -> Self {
        self.rounding = mode;
        self
    }

    /// Uses long-form suffix labels instead of short ones.
    ///
    /// Long suffixes come from the active locale. For English:
    /// `"K"` → `" thousand"`, `"M"` → `" million"`, and so on.
    ///
    /// For non-English locales, long suffixes may also be inflected based on
    /// the rendered value (e.g. Russian: `"2 тысячи"`, `"5 тысяч"`).
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
    /// assert_eq!(humfmt::number_with(1_000_000, NumberOptions::new().long_units()).to_string(), "1 million");
    /// ```
    #[inline]
    pub fn long_units(mut self) -> Self {
        self.long_units = true;
        self
    }

    /// Enables digit grouping separators for unscaled output.
    ///
    /// **Important:** grouping separators apply **only when the value is not
    /// compacted** — that is, when the output has no suffix.
    /// For compacted output like `"15.3K"` the integer part is always small
    /// (`15`) and grouping would never trigger anyway.
    ///
    /// To show grouped digits for large numbers, you should disable compact scaling
    /// via [`compact(false)`].
    ///
    /// [`compact(false)`]: NumberOptions::compact
    ///
    /// Separator characters come from the active locale:
    /// - English: group separator `','`, decimal separator `'.'`
    /// - Russian / Polish: group separator `' '`, decimal separator `','`
    ///
    /// # Behaviour table
    ///
    /// | Input | `separators(false)` | `separators(true)` |
    /// |---:|---|---|
    /// | `999` | `"999"` | `"999"` |
    /// | `1_234` | `"1.2K"` | `"1.2K"` (compacted, grouping has no effect) |
    /// | `1_234` with `compact(false)` | `"1234"` | `"1,234"` |
    /// | `1_234_567` with `compact(false)`| `"1234567"` | `"1,234,567"` |
    ///
    /// # Examples
    ///
    /// ```rust
    /// use humfmt::{number_with, NumberOptions};
    ///
    /// // Disable compact scaling to show grouped digits.
    /// let opts = NumberOptions::new().compact(false).separators(true);
    /// assert_eq!(number_with(1_234_567, opts).to_string(), "1,234,567");
    /// ```
    #[inline]
    pub fn separators(mut self, yes: bool) -> Self {
        self.separators = yes;
        self
    }

    /// Controls whether trailing fractional zeros are preserved.
    ///
    /// - `false` (default): trailing zeros are trimmed — `"1.50K"` → `"1.5K"`
    /// - `true`: trailing zeros are kept — `"1.50K"` stays `"1.50K"`
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
    /// assert_eq!(humfmt::number_with(1_000, trimmed).to_string(), "1K");
    ///
    /// let fixed = NumberOptions::new().precision(2).fixed_precision(true);
    /// assert_eq!(humfmt::number_with(1_500, fixed).to_string(), "1.50K");
    /// assert_eq!(humfmt::number_with(1_000, fixed).to_string(), "1.00K");
    /// ```
    #[inline]
    pub fn fixed_precision(mut self, yes: bool) -> Self {
        self.fixed_precision = yes;
        self
    }

    /// Switches the active locale.
    ///
    /// Locale affects:
    /// - decimal and grouping separator characters
    /// - compact suffix labels (short and long)
    /// - suffix inflection rules (Russian, Polish)
    /// - maximum compact scaling index
    ///
    /// # Examples
    ///
    /// ```rust
    /// use humfmt::{number_with, NumberOptions};
    /// use humfmt::locale::English;
    ///
    /// assert_eq!(number_with(15_320, NumberOptions::new().locale(English)).to_string(), "15.3K");
    /// ```
    #[inline]
    pub fn locale<N: Locale>(self, locale: N) -> NumberOptions<N> {
        NumberOptions {
            precision: self.precision,
            compact: self.compact,
            rounding: self.rounding,
            long_units: self.long_units,
            separators: self.separators,
            fixed_precision: self.fixed_precision,
            locale,
        }
    }
}

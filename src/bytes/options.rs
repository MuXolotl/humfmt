use crate::locale::Locale;
use crate::RoundingMode;

/// Represents a magnitude of bytes.
///
/// Used to clamp or force the output unit in [`BytesOptions`].
/// Depending on the `binary()` setting, `MB` will output either
/// decimal `MB` (1000-based) or binary `MiB` (1024-based).
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ByteUnit {
    /// Bytes (B)
    B = 0,
    /// Kilobytes (KB) or Kibibytes (KiB)
    KB = 1,
    /// Megabytes (MB) or Mebibytes (MiB)
    MB = 2,
    /// Gigabytes (GB) or Gibibytes (GiB)
    GB = 3,
    /// Terabytes (TB) or Tebibytes (TiB)
    TB = 4,
    /// Petabytes (PB) or Pebibytes (PiB)
    PB = 5,
    /// Exabytes (EB) or Exbibytes (EiB)
    EB = 6,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(crate) enum Precision {
    Decimals(u8),
    Significant(u8),
}

/// Builder-style configuration for byte-size formatting.
///
/// # Examples
///
/// ```rust
/// use humfmt::BytesOptions;
///
/// let opts = BytesOptions::new().binary().precision(2);
/// assert_eq!(humfmt::bytes_with(1536_u64, opts).to_string(), "1.5KiB");
/// ```
///
/// ```rust
/// use humfmt::BytesOptions;
///
/// let opts = BytesOptions::new().decimal_separator(',').space(true);
/// assert_eq!(humfmt::bytes_with(1536_u64, opts).to_string(), "1,5 KB");
/// ```
#[derive(Copy, Clone, Debug)]
pub struct BytesOptions {
    pub(crate) precision: Precision,
    pub(crate) rounding: RoundingMode,
    pub(crate) binary: bool,
    pub(crate) long_units: bool,
    pub(crate) decimal_separator: char,
    pub(crate) space: bool,
    pub(crate) fixed_precision: bool,
    pub(crate) min_unit: ByteUnit,
    pub(crate) max_unit: ByteUnit,
}

impl BytesOptions {
    /// Creates default decimal byte formatting options.
    ///
    /// Defaults:
    /// - precision: `1`
    /// - rounding: `HalfUp`
    /// - standard: decimal (SI, 1000-based)
    /// - unit labels: short (`KB`, `MB`, ...)
    /// - decimal separator: `'.'`
    /// - short-unit spacing: disabled
    /// - fixed precision: `false` (trailing zeros are trimmed)
    /// - min unit: `ByteUnit::B`
    /// - max unit: `ByteUnit::EB`
    #[inline]
    pub fn new() -> Self {
        Self {
            precision: Precision::Decimals(1),
            rounding: RoundingMode::HalfUp,
            binary: false,
            long_units: false,
            decimal_separator: '.',
            space: false,
            fixed_precision: false,
            min_unit: ByteUnit::B,
            max_unit: ByteUnit::EB,
        }
    }

    /// Sets decimal precision for scaled values.
    ///
    /// Precision is clamped to `0..=6`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use humfmt::BytesOptions;
    ///
    /// let opts = BytesOptions::new().precision(2);
    /// assert_eq!(humfmt::bytes_with(1536_u64, opts).to_string(), "1.54KB");
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
    /// # Examples
    ///
    /// ```rust
    /// use humfmt::BytesOptions;
    ///
    /// let opts = BytesOptions::new().significant_digits(3);
    /// assert_eq!(humfmt::bytes_with(1536_u64, opts).to_string(), "1.54KB");
    /// ```
    #[inline]
    pub fn significant_digits(mut self, n: u8) -> Self {
        self.precision = Precision::Significant(n.clamp(1, 39));
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
    /// use humfmt::{BytesOptions, RoundingMode};
    ///
    /// let floor = BytesOptions::new().precision(0).rounding(RoundingMode::Floor);
    /// assert_eq!(humfmt::bytes_with(1_900_u64, floor).to_string(), "1KB");
    ///
    /// let ceil = BytesOptions::new().precision(0).rounding(RoundingMode::Ceil);
    /// assert_eq!(humfmt::bytes_with(1_100_u64, ceil).to_string(), "2KB");
    /// ```
    #[inline]
    pub fn rounding(mut self, mode: RoundingMode) -> Self {
        self.rounding = mode;
        self
    }

    /// Uses binary (IEC, 1024-based) units like `KiB` instead of decimal `KB`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use humfmt::BytesOptions;
    ///
    /// let opts = BytesOptions::new().binary();
    /// assert_eq!(humfmt::bytes_with(1024_u64, opts).to_string(), "1KiB");
    /// ```
    #[inline]
    pub fn binary(mut self) -> Self {
        self.binary = true;
        self
    }

    /// Uses long unit labels like `kilobytes` instead of `KB`.
    ///
    /// Long labels always include a separating space. When enabled,
    /// the `space(...)` option has no effect.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use humfmt::BytesOptions;
    ///
    /// let opts = BytesOptions::new().long_units();
    /// assert_eq!(humfmt::bytes_with(1_u64, opts).to_string(), "1 byte");
    /// ```
    #[inline]
    pub fn long_units(mut self) -> Self {
        self.long_units = true;
        self
    }

    /// Controls whether short unit labels are separated from the number by a space.
    ///
    /// - `false` (default): `1.5KB`, `999B`
    /// - `true`: `1.5 KB`, `999 B`
    ///
    /// Has no effect when `long_units` is enabled.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use humfmt::BytesOptions;
    ///
    /// let opts = BytesOptions::new().space(true);
    /// assert_eq!(humfmt::bytes_with(999_u64, opts).to_string(), "999 B");
    /// assert_eq!(humfmt::bytes_with(1536_u64, opts).to_string(), "1.5 KB");
    /// ```
    #[inline]
    pub fn space(mut self, enabled: bool) -> Self {
        self.space = enabled;
        self
    }

    /// Overrides the decimal separator for scaled byte values.
    ///
    /// Affects scaled output like `1.5KB` but not unscaled output like `999B`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use humfmt::BytesOptions;
    ///
    /// let opts = BytesOptions::new().decimal_separator(',');
    /// assert_eq!(humfmt::bytes_with(1536_u64, opts).to_string(), "1,5KB");
    /// ```
    #[inline]
    pub fn decimal_separator(mut self, separator: char) -> Self {
        self.decimal_separator = separator;
        self
    }

    /// Controls whether trailing fractional zeros are preserved.
    ///
    /// - `false` (default): trailing zeros are trimmed — `1.50 KiB` becomes `1.5 KiB`
    /// - `true`: trailing zeros are kept — `1.50 KiB` stays `1.50 KiB`
    ///
    /// Useful when you need consistent column widths in tables or dashboards.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use humfmt::BytesOptions;
    ///
    /// let trimmed = BytesOptions::new().binary().precision(2).space(true);
    /// assert_eq!(humfmt::bytes_with(1536_u64, trimmed).to_string(), "1.5 KiB");
    ///
    /// let fixed = BytesOptions::new().binary().precision(2).space(true).fixed_precision(true);
    /// assert_eq!(humfmt::bytes_with(1536_u64, fixed).to_string(), "1.50 KiB");
    /// ```
    #[inline]
    pub fn fixed_precision(mut self, yes: bool) -> Self {
        self.fixed_precision = yes;
        self
    }

    /// Clamps the minimum unit used for formatting.
    ///
    /// Useful to avoid switching down to Bytes when formatting columns that
    /// should remain in KB or higher.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use humfmt::{BytesOptions, ByteUnit};
    ///
    /// let opts = BytesOptions::new().min_unit(ByteUnit::KB).precision(3);
    /// // 500 bytes is formatted as 0.5 KB
    /// assert_eq!(humfmt::bytes_with(500_u64, opts).to_string(), "0.5KB");
    /// ```
    #[inline]
    pub fn min_unit(mut self, unit: ByteUnit) -> Self {
        self.min_unit = unit;
        self
    }

    /// Clamps the maximum unit used for formatting.
    ///
    /// Useful to avoid switching up to TB or PB when formatting columns that
    /// should remain in GB.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use humfmt::{BytesOptions, ByteUnit};
    ///
    /// let opts = BytesOptions::new().max_unit(ByteUnit::GB);
    /// // 2,000,000,000,000 bytes is formatted as 2000 GB instead of 2 TB
    /// assert_eq!(humfmt::bytes_with(2_000_000_000_000_u64, opts).to_string(), "2000GB");
    /// ```
    #[inline]
    pub fn max_unit(mut self, unit: ByteUnit) -> Self {
        self.max_unit = unit;
        self
    }

    /// Forces the formatter to always use a specific unit.
    ///
    /// This is equivalent to setting both `min_unit` and `max_unit` to the same value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use humfmt::{BytesOptions, ByteUnit};
    ///
    /// let opts = BytesOptions::new().unit(ByteUnit::MB).precision(3);
    /// assert_eq!(humfmt::bytes_with(150_000_u64, opts).to_string(), "0.15MB");
    /// assert_eq!(humfmt::bytes_with(1_500_000_u64, opts).to_string(), "1.5MB");
    /// assert_eq!(humfmt::bytes_with(1_500_000_000_u64, opts).to_string(), "1500MB");
    /// ```
    #[inline]
    pub fn unit(mut self, unit: ByteUnit) -> Self {
        self.min_unit = unit;
        self.max_unit = unit;
        self
    }

    /// Applies the decimal separator from the provided locale.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use humfmt::{bytes_with, BytesOptions};
    /// use humfmt::locale::CustomLocale;
    ///
    /// let locale = CustomLocale::english().decimal_separator(',');
    /// let opts = BytesOptions::new().locale(locale);
    ///
    /// assert_eq!(bytes_with(1536_u64, opts).to_string(), "1,5KB");
    /// ```
    #[inline]
    pub fn locale<L: Locale>(mut self, locale: L) -> Self {
        self.decimal_separator = locale.decimal_separator();
        self
    }
}

impl Default for BytesOptions {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

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
/// # Quick reference
///
/// | Method | Default | Effect |
/// |---|---|---|
/// | [`precision(n)`] | `1` | Decimal places for the scaled fractional part |
/// | [`significant_digits(n)`] | `none` | Total significant digits (overrides precision) |
/// | [`binary()`] | `false` | SI (1000) vs IEC (1024) units |
/// | [`bits(bool)`] | `false` | Multiply by 8, use bit units (`Kb`, `Mb`) |
/// | [`rounding(mode)`] | `HalfUp` | `HalfUp`, `Floor`, `Ceil` |
/// | [`long_units()`] | `false` | `"KB"` -> `" kilobytes"` |
/// | [`space(bool)`] | `false` | `"1.5KB"` -> `"1.5 KB"` |
/// | [`decimal_separator(c)`] | `'.'` | Decimal separator character |
/// | [`fixed_precision(bool)`] | `false` | `"1.5KB"` -> `"1.50KB"` |
/// | [`min_unit(u)`] | `B` | Clamp minimum unit |
/// | [`max_unit(u)`] | `EB` | Clamp maximum unit |
/// | [`unit(u)`] | auto | Force specific unit |
///
/// [`precision(n)`]: BytesOptions::precision
/// [`significant_digits(n)`]: BytesOptions::significant_digits
/// [`binary()`]: BytesOptions::binary
/// [`bits(bool)`]: BytesOptions::bits
/// [`rounding(mode)`]: BytesOptions::rounding
/// [`long_units()`]: BytesOptions::long_units
/// [`space(bool)`]: BytesOptions::space
/// [`decimal_separator(c)`]: BytesOptions::decimal_separator
/// [`fixed_precision(bool)`]: BytesOptions::fixed_precision
/// [`min_unit(u)`]: BytesOptions::min_unit
/// [`max_unit(u)`]: BytesOptions::max_unit
/// [`unit(u)`]: BytesOptions::unit
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
    pub(crate) bits: bool,
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
    pub const fn new() -> Self {
        Self {
            precision: Precision::Decimals(1),
            rounding: RoundingMode::HalfUp,
            binary: false,
            bits: false,
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
    /// # Behaviour table
    ///
    /// | Input | `precision(0)` | `precision(1)` (default) | `precision(2)` |
    /// |---:|---|---|---|
    /// | `1_536` | `"2KB"` | `"1.5KB"` | `"1.54KB"` |
    /// | `999_950` | `"1MB"` | `"1MB"` | `"1MB"` (rescaled) |
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
    pub const fn precision(mut self, n: u8) -> Self {
        let n = if n > 6 { 6 } else { n };
        self.precision = Precision::Decimals(n);
        self
    }

    /// Sets the total number of significant digits to display.
    ///
    /// Clamped to `1..=39` (the maximum digits in a `u128`).
    ///
    /// # Behaviour table
    ///
    /// | Input | `significant_digits(3)` | Notes |
    /// |---:|---|---|
    /// | `1_234` | `"1.23KB"` | `1`, `2`, `3` are the 3 significant digits |
    /// | `12_345` | `"12.3KB"` | `1`, `2`, `3` are the 3 significant digits |
    /// | `123_456` | `"123KB"` | `1`, `2`, `3` are the 3 significant digits |
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

    /// Sets the rounding direction for values that require precision cutoff.
    ///
    /// - `HalfUp` (default): standard mathematical rounding. Ties round away from zero.
    /// - `Floor`: always round towards negative infinity.
    /// - `Ceil`: always round towards positive infinity.
    ///
    /// # Behaviour table
    ///
    /// | Input | `precision(0)` + `HalfUp` | `Floor` | `Ceil` |
    /// |---:|---|---|---|
    /// | `1_500` | `"2KB"` | `"1KB"` | `"2KB"` |
    /// | `-1_500` | `"-2KB"` | `"-2KB"` | `"-1KB"` |
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
    pub const fn rounding(mut self, mode: RoundingMode) -> Self {
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
    pub const fn binary(mut self) -> Self {
        self.binary = true;
        self
    }

    /// Formats the input value as bits rather than bytes.
    ///
    /// Internally multiplies the value by 8 and uses lowercase suffixes (`Kb`, `Mb`).
    ///
    /// Note: for inputs near `u128::MAX`, the multiplication saturates at
    /// `u128::MAX`. This is a documented limit; in practice no real byte count
    /// approaches this magnitude.
    ///
    /// # Behaviour table
    ///
    /// | Input | `bits(false)` (default) | `bits(true)` |
    /// |---:|---|---|
    /// | `1000` | `"1KB"` | `"8Kb"` |
    /// | `1_500_000` | `"1.5MB"` | `"12Mb"` |
    /// | `125` (long) | `"125 bytes"` | `"1 kilobit"` |
    ///
    /// # Examples
    ///
    /// ```rust
    /// use humfmt::BytesOptions;
    ///
    /// let opts = BytesOptions::new().bits(true);
    /// assert_eq!(humfmt::bytes_with(1000_u64, opts).to_string(), "8Kb");
    /// ```
    #[inline]
    pub const fn bits(mut self, enabled: bool) -> Self {
        self.bits = enabled;
        self
    }

    /// Uses long unit labels like `kilobytes` instead of `KB`.
    ///
    /// Long labels always include a separating space. When enabled,
    /// the [`space(...)`](BytesOptions::space) option has no effect.
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
    pub const fn long_units(mut self) -> Self {
        self.long_units = true;
        self
    }

    /// Controls whether short unit labels are separated from the number by a space.
    ///
    /// - `false` (default): `1.5KB`, `999B`
    /// - `true`: `1.5 KB`, `999 B`
    ///
    /// Has no effect when [`long_units`](BytesOptions::long_units) is enabled.
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
    pub const fn space(mut self, enabled: bool) -> Self {
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
    pub const fn decimal_separator(mut self, separator: char) -> Self {
        self.decimal_separator = separator;
        self
    }

    /// Controls whether trailing fractional zeros are preserved.
    ///
    /// - `false` (default): trailing zeros are trimmed (`1.50 KiB` -> `1.5 KiB`).
    /// - `true`: trailing zeros are kept (`1.50 KiB` stays `1.50 KiB`).
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
    pub const fn fixed_precision(mut self, yes: bool) -> Self {
        self.fixed_precision = yes;
        self
    }

    /// Clamps the minimum unit used for formatting.
    ///
    /// Useful to avoid switching down to bytes when formatting columns that
    /// should remain in KB or higher.
    ///
    /// If the natural unit is below `min_unit`, the value is scaled down
    /// (e.g. `500 B` with `min_unit(KB)` becomes `"0.5KB"`).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use humfmt::{BytesOptions, ByteUnit};
    ///
    /// let opts = BytesOptions::new().min_unit(ByteUnit::KB).precision(3);
    /// assert_eq!(humfmt::bytes_with(500_u64, opts).to_string(), "0.5KB");
    /// ```
    #[inline]
    pub const fn min_unit(mut self, unit: ByteUnit) -> Self {
        self.min_unit = unit;
        self
    }

    /// Clamps the maximum unit used for formatting.
    ///
    /// Useful to avoid switching up to TB or PB when formatting columns that
    /// should remain in GB.
    ///
    /// If the natural unit exceeds `max_unit`, the value is left unscaled
    /// (e.g. `2 TB` with `max_unit(GB)` becomes `"2000GB"`).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use humfmt::{BytesOptions, ByteUnit};
    ///
    /// let opts = BytesOptions::new().max_unit(ByteUnit::GB);
    /// assert_eq!(humfmt::bytes_with(2_000_000_000_000_u64, opts).to_string(), "2000GB");
    /// ```
    #[inline]
    pub const fn max_unit(mut self, unit: ByteUnit) -> Self {
        self.max_unit = unit;
        self
    }

    /// Forces the formatter to always use a specific unit.
    ///
    /// Equivalent to setting both `min_unit` and `max_unit` to the same value.
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
    pub const fn unit(mut self, unit: ByteUnit) -> Self {
        self.min_unit = unit;
        self.max_unit = unit;
        self
    }
}

impl Default for BytesOptions {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

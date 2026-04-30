use crate::locale::Locale;

/// Builder-style configuration for byte-size formatting.
///
/// This type is designed to be:
/// - cheap to copy (`Copy`)
/// - easy to chain (builder methods return `Self`)
/// - predictable (values are clamped to small, fixed ranges)
///
/// Locale-awareness for bytes is intentionally minimal:
/// - unit labels are currently English-only (`KB`, `MB`, or `kilobytes`, ...),
/// - the decimal separator for scaled values is configurable and can be taken
///   from a `Locale` (`BytesOptions::locale(...)`).
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
    pub(crate) precision: u8,
    pub(crate) binary: bool,
    pub(crate) long_units: bool,
    pub(crate) decimal_separator: char,
    pub(crate) space: bool,
}

impl BytesOptions {
    /// Creates default decimal byte formatting options.
    ///
    /// Defaults:
    /// - precision: `1`
    /// - standard: decimal (SI, 1000-based)
    /// - unit labels: short (`KB`, `MB`, ...)
    /// - decimal separator: `'.'`
    /// - short-unit spacing: disabled (no space between number and suffix)
    #[inline]
    pub fn new() -> Self {
        Self {
            precision: 1,
            binary: false,
            long_units: false,
            decimal_separator: '.',
            space: false,
        }
    }

    /// Sets decimal precision for scaled values.
    ///
    /// Precision is clamped to `0..=6` to keep formatting cheap and predictable.
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
        self.precision = n.min(6);
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
    /// Long labels include a separating space (e.g. `"1.5 kilobytes"`).
    /// When `long_units` is enabled, the `space(...)` option has no effect.
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
    /// This affects output like:
    /// - `false` (default): `1.5KB`, `1.5KiB`, `999B`
    /// - `true`: `1.5 KB`, `1.5 KiB`, `999 B`
    ///
    /// This option applies only to short unit labels. Long labels always include a space.
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
    /// This affects output like `1.5KB` (scaled) but not unscaled output like `999B`.
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

    /// Applies numeric separators from the provided locale.
    ///
    /// Currently this only affects the decimal separator used by the byte formatter.
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

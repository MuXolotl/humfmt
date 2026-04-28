use crate::locale::{English, Locale};

/// Builder-style configuration for duration and relative-time formatting.
///
/// This options type is shared by:
/// - [`crate::duration_with`]
/// - [`crate::ago_with`]
/// - optional `chrono` / `time` integrations
///
/// # Examples
///
/// ```rust
/// use humfmt::DurationOptions;
///
/// let opts = DurationOptions::new().max_units(3).long_units();
/// assert_eq!(
///     humfmt::duration_with(core::time::Duration::from_secs(3665), opts).to_string(),
///     "1 hour 1 minute 5 seconds"
/// );
/// ```
#[derive(Copy, Clone, Debug)]
pub struct DurationOptions<L: Locale = English> {
    max_units: u8,
    long_units: bool,
    locale: L,
}

impl DurationOptions<English> {
    /// Creates default duration formatting options (English).
    ///
    /// Defaults:
    /// - max units: `2`
    /// - long units: `false` (compact labels like `h`, `m`, `s`)
    /// - locale: `English`
    #[inline]
    pub fn new() -> Self {
        Self {
            max_units: 2,
            long_units: false,
            locale: English,
        }
    }
}

impl<L: Locale> Default for DurationOptions<L> {
    #[inline]
    fn default() -> Self {
        Self {
            max_units: 2,
            long_units: false,
            locale: L::default(),
        }
    }
}

impl<L: Locale> DurationOptions<L> {
    /// Limits how many non-zero units are rendered.
    ///
    /// The value is clamped to `1..=4` to keep output compact and stable.
    /// (Rendering too many units tends to produce long, noisy strings.)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use humfmt::DurationOptions;
    ///
    /// let opts = DurationOptions::new().max_units(1);
    /// assert_eq!(
    ///     humfmt::duration_with(core::time::Duration::from_secs(3665), opts).to_string(),
    ///     "1h"
    /// );
    /// ```
    #[inline]
    pub fn max_units(mut self, n: u8) -> Self {
        self.max_units = n.clamp(1, 4);
        self
    }

    /// Uses long localized unit labels like `"hour"` instead of `"h"`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use humfmt::DurationOptions;
    ///
    /// let opts = DurationOptions::new().long_units();
    /// assert_eq!(
    ///     humfmt::duration_with(core::time::Duration::from_secs(90), opts).to_string(),
    ///     "1 minute 30 seconds"
    /// );
    /// ```
    #[inline]
    pub fn long_units(mut self) -> Self {
        self.long_units = true;
        self
    }

    /// Switches the active locale.
    ///
    /// This affects:
    /// - duration unit labels (short/long)
    /// - pluralization/inflection rules (if implemented by the locale)
    /// - relative-time wording (the "ago" word)
    #[inline]
    pub fn locale<N: Locale>(self, locale: N) -> DurationOptions<N> {
        DurationOptions {
            max_units: self.max_units,
            long_units: self.long_units,
            locale,
        }
    }

    pub(crate) fn max_units_value(&self) -> u8 {
        self.max_units
    }

    pub(crate) fn long_units_value(&self) -> bool {
        self.long_units
    }

    pub(crate) fn locale_ref(&self) -> &L {
        &self.locale
    }
}

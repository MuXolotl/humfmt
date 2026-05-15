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
pub struct DurationOptions {
    pub(crate) max_units: u8,
    pub(crate) long_units: bool,
}

impl DurationOptions {
    /// Creates default duration formatting options.
    ///
    /// Defaults:
    /// - max units: `2`
    /// - long units: `false` (compact labels like `h`, `m`, `s`)
    #[inline]
    pub const fn new() -> Self {
        Self {
            max_units: 2,
            long_units: false,
        }
    }

    /// Limits how many non-zero units are rendered.
    ///
    /// The value is clamped to `1..=7` (the full range of supported units:
    /// days, hours, minutes, seconds, milliseconds, microseconds, nanoseconds).
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
    ///
    /// let opts_full = DurationOptions::new().max_units(7).long_units();
    /// assert_eq!(
    ///     humfmt::duration_with(core::time::Duration::from_nanos(1_001_001_001), opts_full)
    ///         .to_string(),
    ///     "1 second 1 millisecond 1 microsecond 1 nanosecond"
    /// );
    /// ```
    #[inline]
    pub const fn max_units(mut self, n: u8) -> Self {
        let n = if n < 1 {
            1
        } else if n > 7 {
            7
        } else {
            n
        };
        self.max_units = n;
        self
    }

    /// Uses long unit labels like `"hour"` instead of `"h"`.
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
    pub const fn long_units(mut self) -> Self {
        self.long_units = true;
        self
    }
}

impl Default for DurationOptions {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

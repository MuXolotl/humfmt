/// Builder-style configuration for relative-time formatting.
///
/// [`crate::ago_with`], `human_ago_with`, and the `chrono` / `time`
/// `ago_*` adapters take this type. It carries the same two knobs as
/// [`crate::DurationOptions`] today, but relative time owns its options type so
/// it can grow features of its own without disturbing duration formatting. The
/// two types convert into each other with `From`, so one option set can be
/// reused for both formatters.
///
/// # Examples
///
/// ```rust
/// use humfmt::{AgoOptions, DurationOptions};
///
/// let opts = AgoOptions::new().max_units(3).long_units();
/// assert_eq!(
///     humfmt::ago_with(core::time::Duration::from_secs(3665), opts).to_string(),
///     "1 hour 1 minute 5 seconds ago"
/// );
///
/// let shared = DurationOptions::new().max_units(3);
/// let out = humfmt::ago_with(core::time::Duration::from_secs(3665), shared.into());
/// assert_eq!(out.to_string(), "1h 1m 5s ago");
/// ```
#[derive(Copy, Clone, Debug)]
pub struct AgoOptions {
    pub(crate) max_units: u8,
    pub(crate) long_units: bool,
}

impl AgoOptions {
    /// Creates default relative-time options.
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
    /// use humfmt::AgoOptions;
    ///
    /// let opts = AgoOptions::new().max_units(1);
    /// assert_eq!(
    ///     humfmt::ago_with(core::time::Duration::from_secs(3665), opts).to_string(),
    ///     "1h ago"
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
    /// use humfmt::AgoOptions;
    ///
    /// let opts = AgoOptions::new().long_units();
    /// assert_eq!(
    ///     humfmt::ago_with(core::time::Duration::from_secs(90), opts).to_string(),
    ///     "1 minute 30 seconds ago"
    /// );
    /// ```
    #[inline]
    pub const fn long_units(mut self) -> Self {
        self.long_units = true;
        self
    }
}

impl Default for AgoOptions {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl From<crate::DurationOptions> for AgoOptions {
    #[inline]
    fn from(options: crate::DurationOptions) -> Self {
        Self {
            max_units: options.max_units,
            long_units: options.long_units,
        }
    }
}

impl From<AgoOptions> for crate::DurationOptions {
    #[inline]
    fn from(options: AgoOptions) -> Self {
        Self {
            max_units: options.max_units,
            long_units: options.long_units,
        }
    }
}

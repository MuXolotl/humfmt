/// Builder-style configuration for relative-time formatting.
///
/// [`crate::ago_with`], `human_ago_with`, and the `chrono` / `time` `ago_*`
/// adapters take this type. It covers the two knobs of
/// [`crate::DurationOptions`] plus the relative-time specific
/// [`just_now`](AgoOptions::just_now) threshold. The types convert into each
/// other with `From`, so one option set can be reused for both formatters;
/// converting `AgoOptions` into `DurationOptions` drops `just_now`.
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
    pub(crate) just_now: core::time::Duration,
}

impl AgoOptions {
    /// Creates default relative-time options.
    ///
    /// Defaults:
    /// - max units: `2`
    /// - long units: `false` (compact labels like `h`, `m`, `s`)
    /// - just now: `core::time::Duration::ZERO` (the phrase is off)
    #[inline]
    pub const fn new() -> Self {
        Self {
            max_units: 2,
            long_units: false,
            just_now: core::time::Duration::ZERO,
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

    /// Renders durations shorter than `threshold` as `"just now"`.
    ///
    /// The comparison is strict: a duration equal to `threshold` is rendered
    /// normally. The default `core::time::Duration::ZERO` keeps the phrase off,
    /// so `"0s ago"` stays the output unless a threshold is set. The phrase is
    /// written as-is, regardless of [`long_units`](AgoOptions::long_units).
    ///
    /// # Behaviour table
    ///
    /// | Input | Default | `just_now(5s)` |
    /// |---:|---|---|
    /// | `0s` | `"0s ago"` | `"just now"` |
    /// | `3s` | `"3s ago"` | `"just now"` |
    /// | `5s` | `"5s ago"` | `"5s ago"` (threshold is exclusive) |
    /// | `90s` | `"1m 30s ago"` | `"1m 30s ago"` |
    ///
    /// # Examples
    ///
    /// ```rust
    /// use core::time::Duration;
    /// use humfmt::AgoOptions;
    ///
    /// let opts = AgoOptions::new().just_now(Duration::from_secs(5));
    /// assert_eq!(
    ///     humfmt::ago_with(Duration::from_secs(3), opts).to_string(),
    ///     "just now"
    /// );
    /// assert_eq!(
    ///     humfmt::ago_with(Duration::from_secs(5), opts).to_string(),
    ///     "5s ago"
    /// );
    /// ```
    #[inline]
    pub const fn just_now(mut self, threshold: core::time::Duration) -> Self {
        self.just_now = threshold;
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
            just_now: core::time::Duration::ZERO,
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

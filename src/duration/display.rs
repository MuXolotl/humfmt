use core::fmt;

use super::{format::format_duration, DurationOptions};

/// `Display` wrapper for human-readable durations (e.g. `"1h 1m"`).
///
/// Instances of this type are created via [`crate::duration()`] and [`crate::duration_with`].
///
/// This formatter is intentionally compact by default:
/// it renders at most `max_units` non-zero units (default: 2).
#[derive(Copy, Clone, Debug)]
pub struct DurationDisplay {
    value: core::time::Duration,
    options: DurationOptions,
}

impl DurationDisplay {
    pub(crate) fn new(value: core::time::Duration, options: DurationOptions) -> Self {
        Self { value, options }
    }
}

impl fmt::Display for DurationDisplay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        format_duration(f, self.value, &self.options)
    }
}

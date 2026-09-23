use core::fmt;

use crate::common::fmt::{fmt_with_padding, Render};

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

impl Render for DurationDisplay {
    fn render<W: fmt::Write + ?Sized>(&self, f: &mut W) -> fmt::Result {
        format_duration(f, self.value, &self.options)
    }
}

impl fmt::Display for DurationDisplay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt_with_padding(self, f)
    }
}

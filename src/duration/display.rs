use core::fmt;

use crate::common::fmt::StackString;

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
        if f.width().is_none() {
            format_duration(f, self.value, &self.options)
        } else {
            let mut buf = StackString::<128>::new();
            format_duration(&mut buf, self.value, &self.options)?;
            f.pad(buf.as_str())
        }
    }
}

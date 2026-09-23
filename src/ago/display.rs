use core::fmt;

use crate::common::fmt::{fmt_with_padding, Render};
use crate::duration::{format_duration, DurationLike};

use super::AgoOptions;

/// `Display` wrapper for relative time output (e.g. `"1m 30s ago"`).
///
/// Instances of this type are created via [`crate::ago()`] and [`crate::ago_with`].
/// It builds on the duration formatter and appends `" ago"`, or renders
/// `"just now"` when the value is below the configured threshold.
///
/// This type does not allocate on its own; allocation only happens if the caller
/// requests an owned `String` via `.to_string()` / `format!(...)`.
#[derive(Copy, Clone, Debug)]
pub struct AgoDisplay {
    value: core::time::Duration,
    options: AgoOptions,
}

impl AgoDisplay {
    pub(crate) fn new<T: DurationLike>(value: T, options: AgoOptions) -> Self {
        Self {
            value: value.into_duration(),
            options,
        }
    }
}

impl Render for AgoDisplay {
    fn render<W: fmt::Write + ?Sized>(&self, f: &mut W) -> fmt::Result {
        if self.value < self.options.just_now {
            return f.write_str("just now");
        }

        format_duration(
            f,
            self.value,
            self.options.max_units,
            self.options.long_units,
        )?;
        f.write_str(" ago")
    }
}

impl fmt::Display for AgoDisplay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt_with_padding(self, f)
    }
}

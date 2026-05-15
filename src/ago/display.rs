use core::fmt;

use crate::duration::{duration_with, DurationLike, DurationOptions};

/// `Display` wrapper for relative time output (e.g. `"1m 30s ago"`).
///
/// Instances of this type are created via [`crate::ago()`] and [`crate::ago_with`].
/// It builds on the duration formatter and appends `" ago"`.
///
/// This type does not allocate on its own; allocation only happens if the caller
/// requests an owned `String` via `.to_string()` / `format!(...)`.
#[derive(Copy, Clone, Debug)]
pub struct AgoDisplay {
    value: core::time::Duration,
    options: DurationOptions,
}

impl AgoDisplay {
    pub(crate) fn new<T: DurationLike>(value: T, options: DurationOptions) -> Self {
        Self {
            value: value.into_duration(),
            options,
        }
    }
}

impl fmt::Display for AgoDisplay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ago", duration_with(self.value, self.options))
    }
}

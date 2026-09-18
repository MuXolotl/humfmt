use core::fmt;

use crate::common::fmt::{write_padded, Pad, Render};

use super::{format::format_percent, PercentOptions};

/// `Display` wrapper for percentage formatting (e.g. `"42.3%"`).
///
/// Instances of this type are created via [`crate::percent()`] and [`crate::percent_with`].
///
/// This type does not allocate on its own; allocation only happens if the caller
/// requests an owned `String` via `.to_string()` or `format!(...)`.
#[derive(Copy, Clone, Debug)]
pub struct PercentDisplay {
    value: f64,
    options: PercentOptions,
}

impl PercentDisplay {
    pub(crate) fn new(value: f64, options: PercentOptions) -> Self {
        Self { value, options }
    }
}

impl Render for PercentDisplay {
    fn render<W: fmt::Write + ?Sized>(&self, f: &mut W) -> fmt::Result {
        format_percent(f, self.value, &self.options)
    }
}

impl fmt::Display for PercentDisplay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_padded(Pad::of(f), f, self)
    }
}

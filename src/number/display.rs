use core::fmt;

use crate::common::numeric::NumericValue;

use super::{format::format_number, NumberOptions};

/// `Display` wrapper for compact number formatting (e.g. `"15.3K"`).
///
/// Instances of this type are created via [`crate::number`] and [`crate::number_with`].
///
/// This type does not allocate by itself; allocation only happens if the caller
/// requests an owned `String` via `.to_string()` or `format!(...)`.
#[derive(Copy, Clone, Debug)]
pub struct NumberDisplay {
    value: NumericValue,
    options: NumberOptions,
}

impl NumberDisplay {
    pub(crate) fn new(value: NumericValue, options: NumberOptions) -> Self {
        Self { value, options }
    }
}

impl fmt::Display for NumberDisplay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        format_number(f, self.value, &self.options)
    }
}

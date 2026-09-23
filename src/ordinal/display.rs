use core::fmt;

use crate::common::fmt::{fmt_with_padding, Render};

use super::{ordinal_suffix, traits::OrdinalValue};

/// `Display` wrapper for ordinal formatting (e.g. `"21st"`).
///
/// Instances of this type are created via [`crate::ordinal()`].
#[derive(Copy, Clone, Debug)]
pub struct OrdinalDisplay {
    value: OrdinalValue,
}

impl OrdinalDisplay {
    pub(crate) fn new(value: OrdinalValue) -> Self {
        Self { value }
    }
}

impl Render for OrdinalDisplay {
    fn render<W: fmt::Write + ?Sized>(&self, f: &mut W) -> fmt::Result {
        let (prefix, magnitude) = match self.value {
            OrdinalValue::Int(value) if value < 0 => ("-", value.unsigned_abs()),
            OrdinalValue::Int(value) => ("", value as u128),
            OrdinalValue::UInt(value) => ("", value),
        };

        write!(f, "{}{}{}", prefix, magnitude, ordinal_suffix(magnitude))
    }
}

impl fmt::Display for OrdinalDisplay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt_with_padding(self, f)
    }
}

use core::fmt;
use core::fmt::Write;

use crate::common::fmt::{write_u128, StackString};

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

impl fmt::Display for OrdinalDisplay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.width().is_none() {
            write_ordinal(f, self.value)
        } else {
            let mut buf = StackString::<48>::new();
            write_ordinal(&mut buf, self.value)?;
            f.pad(buf.as_str())
        }
    }
}

fn write_ordinal<W: fmt::Write>(f: &mut W, value: OrdinalValue) -> fmt::Result {
    let (negative, magnitude) = match value {
        OrdinalValue::Int(value) if value < 0 => (true, value.unsigned_abs()),
        OrdinalValue::Int(value) => (false, value as u128),
        OrdinalValue::UInt(value) => (false, value),
    };

    if negative {
        f.write_char('-')?;
    }

    write_u128(f, magnitude, false, ',')?;
    f.write_str(ordinal_suffix(magnitude))
}

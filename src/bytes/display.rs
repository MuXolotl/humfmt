use core::fmt;

use super::{format::format_bytes, traits::BytesValue, BytesOptions};

/// `Display` wrapper for human-readable byte sizes (e.g. `"1.5KB"`).
///
/// Instances of this type are created via [`crate::bytes`] and [`crate::bytes_with`].
///
/// This type writes directly into the provided formatter and does not allocate
/// by itself.
#[derive(Copy, Clone, Debug)]
pub struct BytesDisplay {
    value: BytesValue,
    options: BytesOptions,
}

impl BytesDisplay {
    pub(crate) fn new(value: BytesValue, options: BytesOptions) -> Self {
        Self { value, options }
    }
}

impl fmt::Display for BytesDisplay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        format_bytes(f, self.value, &self.options)
    }
}

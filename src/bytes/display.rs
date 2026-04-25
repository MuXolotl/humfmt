use core::fmt;

use super::{format::format_bytes, traits::BytesValue, BytesOptions};

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

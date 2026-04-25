use core::fmt;

use super::{format::format_duration, DurationOptions};

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
        format_duration(f, self.value, &self.options)
    }
}

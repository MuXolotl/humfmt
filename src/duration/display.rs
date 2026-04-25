use core::fmt;

use crate::locale::{English, Locale};

use super::{format::format_duration, DurationOptions};

#[derive(Copy, Clone, Debug)]
pub struct DurationDisplay<L: Locale = English> {
    value: core::time::Duration,
    options: DurationOptions<L>,
}

impl<L: Locale> DurationDisplay<L> {
    pub(crate) fn new(value: core::time::Duration, options: DurationOptions<L>) -> Self {
        Self { value, options }
    }
}

impl<L: Locale> fmt::Display for DurationDisplay<L> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        format_duration(f, self.value, &self.options)
    }
}

use core::fmt;

use crate::duration::{duration_with, DurationLike, DurationOptions};
use crate::locale::{English, Locale};

#[derive(Copy, Clone, Debug)]
pub struct AgoDisplay<L: Locale = English> {
    value: core::time::Duration,
    options: DurationOptions<L>,
}

impl<L: Locale> AgoDisplay<L> {
    pub(crate) fn new<T: DurationLike>(value: T, options: DurationOptions<L>) -> Self {
        Self {
            value: value.into_duration(),
            options,
        }
    }
}

impl<L: Locale> fmt::Display for AgoDisplay<L> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {}",
            duration_with(self.value, self.options),
            self.options.locale_ref().ago_word()
        )
    }
}

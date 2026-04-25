use core::fmt;

use crate::{
    duration::{duration_with, DurationLike, DurationOptions},
    locale::{English, Locale},
};

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
        let locale = English;
        write!(
            f,
            "{} {}",
            duration_with(self.value, self.options),
            locale.ago_word()
        )
    }
}

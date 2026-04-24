use core::fmt;

use crate::common::numeric::NumericValue;
use crate::locale::{English, Locale};

use super::{format::format_number, NumberOptions};

#[derive(Copy, Clone, Debug)]
pub struct NumberDisplay<L: Locale = English> {
    value: NumericValue,
    options: NumberOptions<L>,
}

impl<L: Locale> NumberDisplay<L> {
    pub(crate) fn new(value: NumericValue, options: NumberOptions<L>) -> Self {
        Self { value, options }
    }
}

impl<L: Locale> fmt::Display for NumberDisplay<L> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        format_number(f, self.value, &self.options)
    }
}

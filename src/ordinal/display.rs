use core::fmt;

use crate::locale::{English, Locale};

use super::traits::OrdinalValue;

#[derive(Copy, Clone, Debug)]
pub struct OrdinalDisplay<L: Locale = English> {
    value: OrdinalValue,
    locale: L,
}

impl<L: Locale> OrdinalDisplay<L> {
    pub(crate) fn new(value: OrdinalValue, locale: L) -> Self {
        Self { value, locale }
    }
}

impl<L: Locale> fmt::Display for OrdinalDisplay<L> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (prefix, magnitude) = match self.value {
            OrdinalValue::Int(value) if value < 0 => ("-", value.unsigned_abs()),
            OrdinalValue::Int(value) => ("", value as u128),
            OrdinalValue::UInt(value) => ("", value),
        };

        let suffix = self.locale.ordinal_suffix(magnitude);
        write!(f, "{prefix}{magnitude}{suffix}")
    }
}

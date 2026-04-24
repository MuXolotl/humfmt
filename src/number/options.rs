use crate::locale::{English, Locale};

#[derive(Copy, Clone, Debug)]
pub struct NumberOptions<L: Locale = English> {
    precision: u8,
    long_units: bool,
    separators: bool,
    locale: L,
}

impl<L: Locale> Default for NumberOptions<L> {
    fn default() -> Self {
        Self {
            precision: 1,
            long_units: false,
            separators: false,
            locale: L::default(),
        }
    }
}

impl<L: Locale> NumberOptions<L> {
    pub fn precision(mut self, n: u8) -> Self {
        self.precision = n;
        self
    }

    pub fn long_units(mut self) -> Self {
        self.long_units = true;
        self
    }

    pub fn separators(mut self, yes: bool) -> Self {
        self.separators = yes;
        self
    }

    pub fn locale<N: Locale>(self, locale: N) -> NumberOptions<N> {
        NumberOptions {
            precision: self.precision,
            long_units: self.long_units,
            separators: self.separators,
            locale,
        }
    }

    pub(crate) fn precision_value(&self) -> u8 {
        self.precision
    }

    pub(crate) fn long_units_value(&self) -> bool {
        self.long_units
    }

    pub(crate) fn separators_value(&self) -> bool {
        self.separators
    }

    pub(crate) fn locale_ref(&self) -> &L {
        &self.locale
    }
}

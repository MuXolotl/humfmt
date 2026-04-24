use crate::locale::{English, Locale};

#[derive(Copy, Clone, Debug)]
pub struct NumberOptions<L: Locale = English> {
    precision: u8,
    long_units: bool,
    separators: bool,
    locale: L,
}
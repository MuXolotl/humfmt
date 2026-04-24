use core::fmt;

use crate::common::numeric::NumericValue;

use super::NumberOptions;

pub fn format_number<L: crate::locale::Locale>(
    f: &mut fmt::Formatter<'_>,
    value: NumericValue,
    _options: &NumberOptions<L>,
) -> fmt::Result {
    match value {
        NumericValue::Int(v) => write!(f, "{v}"),
        NumericValue::UInt(v) => write!(f, "{v}"),
        NumericValue::Float(v) => write!(f, "{v}"),
    }
}
use core::fmt;

use crate::locale::Locale;

use super::ListOptions;

pub fn format_list<T: fmt::Display, L: Locale>(
    f: &mut fmt::Formatter<'_>,
    items: &[T],
    options: &ListOptions<L>,
) -> fmt::Result {
    let locale = options.locale_ref();

    match items {
        [] => Ok(()),
        [item] => write!(f, "{item}"),
        [first, second] => write!(f, "{first} {} {second}", locale.and_word()),
        _ => {
            for (idx, item) in items[..items.len() - 1].iter().enumerate() {
                if idx != 0 {
                    write!(f, ", ")?;
                }

                write!(f, "{item}")?;
            }

            if options.serial_comma_value() {
                write!(f, ",")?;
            }

            write!(f, " {} {}", locale.and_word(), &items[items.len() - 1])
        }
    }
}

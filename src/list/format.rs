use core::fmt;

use crate::locale::Locale;

use super::ListOptions;

pub fn format_list<T: fmt::Display, L: Locale>(
    f: &mut fmt::Formatter<'_>,
    items: &[T],
    options: &ListOptions<L>,
) -> fmt::Result {
    let locale = &options.locale;
    let conjunction = options.conjunction.unwrap_or_else(|| locale.and_word());
    let serial_comma = options
        .serial_comma
        .unwrap_or_else(|| locale.serial_comma());

    match items {
        [] => Ok(()),
        [item] => write!(f, "{item}"),
        [first, second] => write!(f, "{first} {conjunction} {second}"),
        _ => {
            let separator = locale.list_separator();
            for (idx, item) in items[..items.len() - 1].iter().enumerate() {
                if idx != 0 {
                    write!(f, "{separator}")?;
                }
                write!(f, "{item}")?;
            }

            if serial_comma {
                write!(f, ",")?;
            }

            write!(f, " {conjunction} {}", &items[items.len() - 1])
        }
    }
}

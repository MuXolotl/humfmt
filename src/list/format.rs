use core::fmt;

use crate::locale::Locale;

use super::ListOptions;

pub fn format_list<T: fmt::Display, L: Locale>(
    f: &mut fmt::Formatter<'_>,
    items: &[T],
    options: &ListOptions<L>,
) -> fmt::Result {
    let locale = options.locale_ref();
    let conjunction = options.conjunction_or(locale.and_word());

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

            if options.serial_comma_value() {
                write!(f, ",")?;
            }

            write!(f, " {conjunction} {}", &items[items.len() - 1])
        }
    }
}

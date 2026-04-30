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

            // Serial comma (Oxford comma) is only meaningful for comma-style separators
            // such as ", ". If the user overrides the separator to something non-comma-like
            // (e.g. " | "), inserting a literal comma becomes surprising.
            //
            // This keeps the list formatter predictable without expanding the public API.
            if serial_comma && is_comma_style_separator(separator) {
                f.write_str(",")?;
            }

            write!(f, " {conjunction} {}", &items[items.len() - 1])
        }
    }
}

#[inline]
fn is_comma_style_separator(separator: &str) -> bool {
    separator.chars().find(|ch| !ch.is_whitespace()) == Some(',')
}

use core::fmt;

use super::ListOptions;

pub fn format_list<T: fmt::Display>(
    f: &mut fmt::Formatter<'_>,
    items: &[T],
    options: &ListOptions,
) -> fmt::Result {
    let conjunction = options.conjunction;
    let separator = options.separator;
    let serial_comma = options.serial_comma;

    match items {
        [] => Ok(()),
        [item] => write!(f, "{item}"),
        [first, second] => write!(f, "{first} {conjunction} {second}"),
        _ => {
            for (idx, item) in items[..items.len() - 1].iter().enumerate() {
                if idx != 0 {
                    write!(f, "{separator}")?;
                }
                write!(f, "{item}")?;
            }

            // Serial comma (Oxford comma) is only meaningful for comma-style
            // separators such as ", ". If the user overrides the separator to
            // something non-comma-like (e.g. " | "), inserting a literal comma
            // becomes surprising.
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

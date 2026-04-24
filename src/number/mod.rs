mod display;
mod format;
mod options;
mod traits;

pub use display::NumberDisplay;
pub use options::NumberOptions;
pub use traits::NumberLike;

use crate::locale::{English, Locale};

pub fn number<T: NumberLike>(value: T) -> NumberDisplay<English> {
    NumberDisplay::new(value.into_numeric(), NumberOptions::<English>::default())
}

pub fn number_with<T: NumberLike, L: Locale>(
    value: T,
    options: NumberOptions<L>,
) -> NumberDisplay<L> {
    NumberDisplay::new(value.into_numeric(), options)
}

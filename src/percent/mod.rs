//! Percentage formatting.
//!
//! Converts a ratio (e.g. `0.423`) into a human-readable percentage string
//! (e.g. `"42.3%"`), with locale-aware decimal separators and configurable
//! precision.
//!
//! Input is expected to be a ratio in the range `0.0..=1.0`, but values
//! outside this range (e.g. `1.5 → "150%"`) are accepted and rendered as-is.
//! Non-finite inputs (`inf`, `-inf`, `NaN`) render with a `%` suffix.

mod display;
mod format;
mod options;
mod traits;

pub use display::PercentDisplay;
pub use options::PercentOptions;
pub use traits::PercentLike;

use crate::locale::{English, Locale};

/// Creates a human-readable percentage formatter using default options.
///
/// The input is a ratio: `1.0` means `100%`, `0.5` means `50%`.
///
/// # Examples
///
/// ```rust
/// assert_eq!(humfmt::percent(0.423).to_string(), "42.3%");
/// assert_eq!(humfmt::percent(1.0).to_string(), "100%");
/// assert_eq!(humfmt::percent(0.0).to_string(), "0%");
/// ```
pub fn percent<T: PercentLike>(value: T) -> PercentDisplay<English> {
    PercentDisplay::new(value.into_percent(), PercentOptions::new())
}

/// Creates a human-readable percentage formatter with custom options.
///
/// # Examples
///
/// ```rust
/// use humfmt::PercentOptions;
///
/// let opts = PercentOptions::new().precision(2);
/// assert_eq!(humfmt::percent_with(0.4236, opts).to_string(), "42.36%");
/// ```
pub fn percent_with<T: PercentLike, L: Locale>(
    value: T,
    options: PercentOptions<L>,
) -> PercentDisplay<L> {
    PercentDisplay::new(value.into_percent(), options)
}

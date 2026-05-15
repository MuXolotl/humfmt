//! Percentage formatting.
//!
//! Converts a ratio (e.g. `0.423`) into a human-readable percentage string
//! (e.g. `"42.3%"`).
//!
//! Input is expected to be a ratio in the range `0.0..=1.0`, but values
//! outside this range (e.g. `1.5 -> "150%"`) are accepted and rendered as-is.
//! Non-finite inputs (`inf`, `-inf`, `NaN`) render with a `%` suffix.
//!
//! # Quick start
//!
//! ```rust
//! use humfmt::{percent, percent_with, PercentOptions};
//!
//! assert_eq!(percent(0.423_f64).to_string(), "42.3%");
//! assert_eq!(percent(1.0_f64).to_string(), "100%");
//! assert_eq!(percent(1.5_f64).to_string(), "150%");
//!
//! let opts = PercentOptions::new().force_sign(true);
//! assert_eq!(percent_with(0.15_f64, opts).to_string(), "+15%");
//! ```
//!
//! # Edge case behaviour
//!
//! | Input | Default output | Notes |
//! |---:|---|---|
//! | `0.0` | `"0%"` | Zero ratio |
//! | `-0.0` | `"0%"` | Negative zero suppressed |
//! | `0.5` | `"50%"` | Half |
//! | `1.0` | `"100%"` | Full |
//! | `1.5` | `"150%"` | Above 100% accepted |
//! | `-0.423` | `"-42.3%"` | Negative accepted |
//! | `-0.0004` | `"0%"` | Rounds to zero, sign suppressed |
//! | `f64::NAN` | `"NaN%"` | Non-finite preserved |
//! | `f64::INFINITY` | `"inf%"` | Non-finite preserved |
//! | `f64::NEG_INFINITY` | `"-inf%"` | Non-finite preserved |

mod display;
mod format;
mod options;
mod traits;

pub use display::PercentDisplay;
pub use options::PercentOptions;
pub use traits::PercentLike;

/// Creates a human-readable percentage formatter using default options.
///
/// The input is a ratio: `1.0` means `100%`, `0.5` means `50%`.
///
/// # Examples
///
/// ```rust
/// assert_eq!(humfmt::percent(0.423).to_string(), "42.3%");
/// assert_eq!(humfmt::percent(1.0).to_string(), "100%");
/// ```
pub fn percent<T: PercentLike>(value: T) -> PercentDisplay {
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
pub fn percent_with<T: PercentLike>(value: T, options: PercentOptions) -> PercentDisplay {
    PercentDisplay::new(value.into_percent(), options)
}

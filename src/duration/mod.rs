//! Human-readable duration formatting.
//!
//! This module focuses on compact and long-form duration rendering and shares
//! its locale configuration with relative-time formatting.
//!
//! # Quick start
//!
//! ```rust
//! use core::time::Duration;
//! use humfmt::{duration, duration_with, DurationOptions};
//!
//! assert_eq!(duration(Duration::from_secs(3661)).to_string(), "1h 1m");
//!
//! // Long-form with 3 units
//! let opts = DurationOptions::new().long_units().max_units(3);
//! assert_eq!(
//!     duration_with(Duration::from_secs(3665), opts).to_string(),
//!     "1 hour 1 minute 5 seconds"
//! );
//! ```
//!
//! # Edge case behaviour
//!
//! | Input | Default output | Notes |
//! |---:|---|---|
//! | `0s` | `"0s"` | Zero duration |
//! | `900ms` | `"900ms"` | Below 1s |
//! | `1500ms` | `"1s 500ms"` | Compound |
//! | `90s` | `"1m 30s"` | Two units (default cap) |
//! | `3661s` | `"1h 1m"` | Seconds truncated |
//! | `90061s` | `"1d 1h"` | Days included |
//!
//! # Output control
//!
//! - **`max_units(n)`** — limits how many non-zero units are rendered (default: 2,
//!   max: 7). The formatter renders the largest units first.
//! - **`long_units()`** — switches from compact labels (`h`, `m`, `s`) to long-form
//!   (`hour`, `minute`, `second`).
//! - **Locale** — affects unit labels and pluralization (e.g. Russian: `"1 час"`,
//!   `"2 часа"`, `"5 часов"`).

mod display;
mod format;
mod options;
mod traits;

pub use display::DurationDisplay;
pub use options::DurationOptions;
pub use traits::DurationLike;

use crate::locale::{English, Locale};

/// Creates a human-readable duration formatter using default options.
///
/// # Examples
///
/// ```rust
/// let value = core::time::Duration::from_secs(3661);
/// assert_eq!(humfmt::duration(value).to_string(), "1h 1m");
/// ```
pub fn duration<T: DurationLike>(value: T) -> DurationDisplay<English> {
    DurationDisplay::new(value.into_duration(), DurationOptions::new())
}

/// Creates a human-readable duration formatter with custom options.
///
/// # Examples
///
/// ```rust
/// use humfmt::DurationOptions;
///
/// let value = core::time::Duration::from_millis(1500);
/// let out = humfmt::duration_with(value, DurationOptions::new().long_units());
/// assert_eq!(out.to_string(), "1 second 500 milliseconds");
/// ```
pub fn duration_with<T: DurationLike, L: Locale>(
    value: T,
    options: DurationOptions<L>,
) -> DurationDisplay<L> {
    DurationDisplay::new(value.into_duration(), options)
}

//! Human-readable byte-size formatting.
//!
//! Use this module when you want decimal or binary byte units without pulling
//! formatting logic into application code.
//!
//! # Quick start
//!
//! ```rust
//! use humfmt::{bytes, bytes_with, BytesOptions};
//!
//! // Decimal (SI, 1000-based) — default
//! assert_eq!(bytes(1536_u64).to_string(), "1.5KB");
//!
//! // Binary (IEC, 1024-based)
//! assert_eq!(bytes_with(1024_u64, BytesOptions::new().binary()).to_string(), "1KiB");
//!
//! // Bits mode for network speeds
//! assert_eq!(bytes_with(1000_u64, BytesOptions::new().bits(true)).to_string(), "8Kb");
//! ```
//!
//! # Edge case behaviour
//!
//! | Input | Default output | Notes |
//! |---:|---|---|
//! | `0` | `"0B"` | Zero bytes |
//! | `999` | `"999B"` | Below threshold |
//! | `1000` | `"1KB"` | SI threshold |
//! | `1024` | `"1KB"` (SI) / `"1KiB"` (IEC) | Threshold difference |
//! | `-1536` | `"-1.5KB"` | Negative supported |
//! | `u128::MAX` | `"...EB"` | Largest unit, no overflow |
//! | `999_950` | `"1MB"` | Rounds up across unit boundary |
//!
//! # Rounding
//!
//! All rounding uses the selected [`RoundingMode`](crate::RoundingMode)
//! (default: `HalfUp`). The integer path uses exact `u128` long-division
//! arithmetic. When rounding pushes the scaled integer part to the next
//! threshold (e.g. `999_950` at `precision(0)` → `1000KB`), the formatter
//! rescales to the next unit automatically.

mod display;
mod format;
mod options;
mod traits;

pub use display::BytesDisplay;
pub use options::{ByteUnit, BytesOptions};
pub use traits::BytesLike;

/// Creates a human-readable byte-size formatter using default decimal units.
///
/// # Examples
///
/// ```rust
/// assert_eq!(humfmt::bytes(1536).to_string(), "1.5KB");
/// ```
pub fn bytes<T: BytesLike>(value: T) -> BytesDisplay {
    BytesDisplay::new(value.into_bytes(), BytesOptions::new())
}

/// Creates a human-readable byte-size formatter with custom options.
///
/// # Examples
///
/// ```rust
/// use humfmt::BytesOptions;
///
/// let out = humfmt::bytes_with(1536, BytesOptions::new().binary());
/// assert_eq!(out.to_string(), "1.5KiB");
/// ```
pub fn bytes_with<T: BytesLike>(value: T, options: BytesOptions) -> BytesDisplay {
    BytesDisplay::new(value.into_bytes(), options)
}

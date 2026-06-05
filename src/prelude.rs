//! Common imports for ergonomic `humfmt` usage.
//!
//! The prelude re-exports extension traits and the most commonly needed
//! option types. Import it with `use humfmt::prelude::*` to get everything
//! in one shot without cluttering your use list.
//!
//! # Examples
//!
//! ```rust
//! use humfmt::prelude::*;
//!
//! assert_eq!(1_500_000.human_number().to_string(), "1.5M");
//! assert_eq!(1536_u64.human_bytes_with(BytesOptions::new().binary()).to_string(), "1.5KiB");
//! ```

#[cfg(feature = "chrono")]
pub use crate::chrono::ChronoHumanize;
#[cfg(feature = "time")]
pub use crate::time::TimeHumanize;
pub use crate::traits::Humanize;

// Option types that users reach for alongside the trait methods.
pub use crate::{
    ByteUnit, BytesOptions, DurationOptions, ListOptions, NumberOptions, PercentOptions,
    RoundingMode,
};

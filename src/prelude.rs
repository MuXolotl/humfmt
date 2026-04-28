//! Common imports for ergonomic `humfmt` usage.
//!
//! The prelude is intentionally small and only re-exports extension traits.
//! This lets you opt into the ergonomic `.human_*()` methods without pulling
//! in lots of names.
//!
//! # Examples
//!
//! ```rust
//! use humfmt::prelude::*;
//!
//! assert_eq!(1_500_000.human_number().to_string(), "1.5M");
//! ```

#[cfg(feature = "chrono")]
pub use crate::chrono::ChronoHumanize;
#[cfg(feature = "time")]
pub use crate::time::TimeHumanize;
pub use crate::traits::Humanize;

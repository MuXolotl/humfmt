//! # humfmt
//!
//! Ergonomic human-readable formatting toolkit for Rust.
//!
//! `humfmt` provides lightweight, fluent utilities for rendering machine values
//! into readable strings designed for humans.
//!
//! ## Quick examples
//!
//! ```rust
//! use humfmt::Humanize;
//!
//! assert_eq!(humfmt::bytes(1536).to_string(), "1.5KB");
//! assert_eq!(humfmt::number(15320).to_string(), "15.3K");
//! assert_eq!(1_500_000.human_number().to_string(), "1.5M");
//! assert_eq!(humfmt::ordinal(21).to_string(), "21st");
//! assert_eq!(humfmt::duration(core::time::Duration::from_secs(3661)).to_string(), "1h 1m");
//! assert_eq!(humfmt::ago(core::time::Duration::from_secs(90)).to_string(), "1m 30s ago");
//! ```
//!
//! ## Builder customization
//!
//! ```rust
//! use humfmt::{Humanize, NumberOptions};
//!
//! let out = 15_320.human_number_with(
//!     NumberOptions::new()
//!         .precision(2)
//!         .long_units()
//! );
//!
//! assert_eq!(out.to_string(), "15.32 thousand");
//! ```
//!
//! ## Current modules
//!
//! - byte-size formatting
//! - compact number formatting
//! - ordinal formatting
//! - duration formatting
//! - relative time formatting
//! - configurable precision
//! - locale-aware suffix system
//! - optional Russian locale pack
//! - custom locale builder
//! - optional chrono/time integration
//!
//! More humanizers are planned.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod ago;
pub mod bytes;
#[cfg(feature = "chrono")]
pub mod chrono;
pub mod duration;
mod error;
pub mod locale;
pub mod number;
pub mod ordinal;
pub mod prelude;
#[cfg(feature = "time")]
pub mod time;

mod common;
mod traits;

pub use ago::{ago, ago_with, AgoDisplay};
pub use bytes::{bytes, bytes_with, BytesDisplay, BytesLike, BytesOptions};
pub use duration::{duration, duration_with, DurationDisplay, DurationLike, DurationOptions};
pub use error::NegativeDurationError;
pub use number::{number, number_with, NumberDisplay, NumberOptions};
pub use ordinal::{ordinal, ordinal_with, OrdinalDisplay, OrdinalLike};
pub use traits::Humanize;

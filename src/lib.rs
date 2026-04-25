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
//! assert_eq!(humfmt::number(15320).to_string(), "15.3K");
//! assert_eq!(1_500_000.human_number().to_string(), "1.5M");
//! assert_eq!(humfmt::ordinal(21).to_string(), "21st");
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
//! - compact number formatting
//! - ordinal formatting
//! - configurable precision
//! - locale-ready suffix system
//!
//! More humanizers (`bytes`, `duration`, `ago`) are planned.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod locale;
pub mod number;
pub mod ordinal;
pub mod prelude;

mod common;
mod traits;

pub use number::{number, number_with, NumberDisplay, NumberOptions};
pub use ordinal::{ordinal, ordinal_with, OrdinalDisplay, OrdinalLike};
pub use traits::Humanize;

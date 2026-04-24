//! # humfmt
//!
//! Ergonomic human-readable formatting toolkit for Rust.
//!
//! This crate is currently under active development.

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod locale;
pub mod number;
pub mod prelude;

mod common;
mod traits;

pub use number::{number, number_with, NumberDisplay, NumberOptions};
pub use traits::Humanize;

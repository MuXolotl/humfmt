//! # humfmt
//!
//! Ergonomic human-readable formatting toolkit for Rust.
//!
//! This crate is currently under active development.

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod number;
pub mod locale;
pub mod prelude;

mod traits;
mod common;

pub use number::{number, number_with, NumberDisplay, NumberOptions};
pub use traits::Humanize;
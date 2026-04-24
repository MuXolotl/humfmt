#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

//! # humfmt
//!
//! Ergonomic human-readable formatting toolkit for Rust.
//!
//! This crate is currently under active development.

pub mod locale;
pub mod prelude;

mod number;
mod bytes;
mod duration;
mod timeago;
mod ordinal;
mod list;
mod traits;
mod common;

pub use traits::Humanize;
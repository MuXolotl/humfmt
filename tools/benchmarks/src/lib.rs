//! Benchmark harness crate for comparing `humfmt` against other crates.
//!
//! This crate is intentionally separate from `humfmt` itself, so the main
//! library stays dependency-light and CI-friendly.

#![forbid(unsafe_code)]
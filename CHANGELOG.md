# Changelog

All notable changes to `humfmt` will be documented in this file.

The format is based on Keep a Changelog and this project adheres to Semantic Versioning.

---

## [Unreleased]

### Added
- `NumberOptions::fixed_precision(bool)` — opt-in mode that preserves trailing fractional zeros for consistent column widths (e.g. `1.50K` instead of `1.5K`).
- `BytesOptions::fixed_precision(bool)` — same opt-in mode for byte-size output (e.g. `1.50 KiB` instead of `1.5 KiB`).
- MSRV CI job that compiles and tests the library on Rust 1.70 on every push and pull request.
- `common::numeric::is_integer_f64` — shared, correct, `no_std`-compatible helper for checking whether a `f64` value has no fractional part. Replaces the broken `value == (value as u128) as f64` pattern that was copy-pasted across locale modules. Conditionally compiled under `#[cfg(any(feature = "russian", feature = "polish"))]` to avoid `dead_code` warnings in bare builds.
- Unit tests for `is_integer_f64` covering whole numbers, fractional numbers, non-finite values, and negative whole numbers (the previously broken case).
- `DurationOptions::max_units` now accepts values up to `7` (previously clamped to `4`), enabling callers to render all supported time units down to nanoseconds.
- New integration tests for `max_units(7)` and the minimum clamp behaviour (`max_units(0)` → `1`).
- Byte-size formatting now supports a locale-aware decimal separator:
  - `BytesOptions::decimal_separator(char)` overrides the separator for scaled output (e.g. `1,5KB`).
  - `BytesOptions::locale(locale)` copies the decimal separator from any `Locale`.
- `BytesOptions::space(bool)` to optionally insert a space before short unit labels (e.g. `1.5 KB`).

### Fixed
- `common::numeric::is_integer_f64` — previously only compiled under `#[cfg(any(feature = "russian", feature = "polish"))]`. The inner test module always compiled the logic unconditionally, so correctness was verified regardless of active features.
- `is_integer_f64` in `russian.rs` and `polish.rs`: the old `value == (value as u128) as f64` check incorrectly returned `false` for negative whole floats (e.g. `-1.0`, `-42.0`) because casting a negative `f64` to `u128` saturates to `0` on stable Rust. This caused wrong grammatical form selection for negative scaled values in long-form output. Fixed with `value.is_finite() && value % 1.0 == 0.0`.
- Polish long-form plural selection is now CLDR-aligned:
  - `one` is used only for `1`
  - `few` is used for integers ending with `2..=4` excluding `12..=14`
  - all other integers use the `many` form
  This affects both long compact-number suffixes and long-form duration unit labels.
- Restored `polish::ordinal_suffix` and `russian::ordinal_suffix` helpers used by `CustomLocale::{polish,russian}()` presets.
- Russian and Polish duration unit selection now uses integer counts directly (no `u128 -> f64` casts), preserving correctness for very large durations.
- List formatting no longer injects a literal comma for serial-comma output when the list separator is not comma-style (e.g. custom separators like `" | "`).

### Changed
- MSRV raised from **1.67** to **1.70** (released June 2023). This aligns with `criterion 0.5` which already required 1.70, eliminating a silent mismatch where the declared MSRV was lower than what dev-dependencies actually needed.
- Float compact-number scaling (`normalize_scaled`) rewritten from an O(n) loop to an O(1) IEEE 754 exponent-based approach, consistent with the integer path which already used `ilog10`. Division loop is fully removed. The implementation uses a precomputed `POW1000_F64` table instead of `f64::powi` to remain `no_std` compatible.
- `src/bytes/format.rs` — six flat label arrays (`DECIMAL_SHORT`, `BINARY_SHORT`, `DECIMAL_LONG_SINGULAR`, etc.) replaced with two arrays of `UnitLabels` structs. Each struct groups short label, long singular, and long plural for one unit tier. Eliminates the DRY violation and makes adding or auditing labels a single-place change.
- `Locale::duration_unit` default implementation in `traits.rs` now delegates to `english::duration_unit` instead of duplicating the same match expression verbatim. Single source of truth; adding a new time unit no longer requires updating two places.
- `DurationOptions::max_units` clamp widened from `1..=4` to `1..=7`. Existing code using values `1..=4` is unaffected. Values `5..=7` previously silently clamped to `4`; they now work as documented.
- Internal `Options` types (`BytesOptions`, `NumberOptions`, `DurationOptions`, `ListOptions`) now expose their fields as `pub(crate)` directly instead of going through `_value()`-suffixed getter methods. The public builder API is unchanged. This removes a layer of noise with no runtime cost and eliminates naming conflicts between builder methods and internal accessors.
- `proptest` invariant `duration_output_respects_max_units` updated to reflect the new `clamp(1, 7)` bound.
- Float compact-number rounding no longer relies on std-only float math APIs, preserving stable `no_std` builds.
- Float formatting now uses a smaller fixed stack buffer (64 bytes) with a safe fallback, and localized numeric rendering preserves any exponent suffix if it appears.
- Small negative floating-point values that round to zero no longer render as `-0`.

---

## [0.3.0] - 2026-04-28

### Added
- Criterion-based benchmark suite covering the core formatter paths
- Proptest-based invariant coverage for number, bytes, duration, relative-time, and list formatting
- `DurationConversionError` plus non-breaking `*_checked` helpers in `humfmt::chrono` and `humfmt::time`
- locale-aware list item separator hook via `Locale::list_separator()` and `CustomLocale::list_separator(...)`
- `ListOptions::serial_comma_enabled(bool)` and `ListOptions::conjunction(...)` for explicit list-style overrides
- additional property tests for suffix monotonicity and locale decimal-separator invariants
- standalone comparison benchmark harness under `tools/benchmarks/`
- comparison harness coverage for bytes, numbers, durations, and relative-time ("ago")
- benchmark report generator (`report` binary) that produces `BENCHMARKS.md`
- capability matrix in `BENCHMARKS.md` to make comparisons interpretable and fair
- combined dark-theme SVG charts under `assets/benchmarks/` (bytes, time, numbers)
- on-demand GitHub Actions workflow to run the comparison harness and upload artifacts
- extensive rustdoc coverage across the public API (options, locale surface, integrations, and display wrappers)
- compile-time API documentation enforcement via `#![deny(missing_docs)]`

### Changed
- Core number and byte formatting paths were refactored to write directly to the `fmt::Formatter` (fewer intermediate allocations)
- Compact integer number scaling now uses O(1) unit selection (`ilog10` + power-of-1000 lookup) instead of a division loop
- Primitive `Sealed` implementations were centralized in `common::sealed` to remove hidden cross-module coupling
- `StackString::as_str()` is now infallible based on an internal UTF-8 invariant
- Benchmark charts were consolidated into fewer SVG files to keep the repository tidy
- README documentation now points to the updated comparison harness workflow and charts
- Small negative floating-point values that round to zero no longer render as `-0`

---

## [0.2.0] - 2026-04-25

### Added
- `bytes()` and `bytes_with(...)` helpers for human-readable byte sizes
- `BytesOptions` and `Humanize::human_bytes()` / `human_bytes_with(...)`
- `ordinal()` and `ordinal_with(...)` helpers for human-readable ordinal formatting
- `Humanize::human_ordinal()` and `human_ordinal_with(...)`
- `duration()` and `duration_with(...)` helpers for compact human-readable durations
- `DurationOptions` and `Humanize::human_duration()` / `human_duration_with(...)`
- `ago()` and `ago_with(...)` helpers for relative time formatting
- `Humanize::human_ago()` and `human_ago_with(...)`
- optional `chrono` and `time` integration layers for signed durations and timestamp-based relative formatting
- optional `Russian` locale pack for compact numbers and ordinals
- optional `Polish` locale pack for compact numbers and ordinals
- `CustomLocale` builder API for suffix, separator, and ordinal customization
- `list()` and `list_with(...)` helpers for natural-language list formatting
- `ListOptions` for locale-aware conjunction and serial-comma control
- locale-aware duration and relative-time formatting
- `chrono::ago_since_with(...)` and `time::ago_since_with(...)` helpers for localized timestamp comparisons
- gated `ChronoHumanize` / `TimeHumanize` prelude exports when integration features are enabled

### Changed
- localized compact number separators now come from the active locale
- `DurationOptions` now carries locale selection for duration and relative-time output
- `CustomLocale` can now override default list conjunction style
- expanded rustdoc coverage across modules and configured docs.rs to build with all features

---

## [0.1.1] - 2026-04-25

### Fixed
- Restored the `no_std` / `default-features = false` build path
- Normalized `-0.0` formatting to `0`
- Preserved non-finite float rendering (`inf`, `-inf`, `NaN`)

### Changed
- Synced README and roadmap with the published crate state
- Tightened CI to exercise `--all-features` and minimal-feature builds
- Narrowed crate metadata to the features that actually exist today
- Expanded compact number suffix coverage beyond trillion
- Documented the 0.1.x feature-flag compatibility story and added CI coverage for placeholder flags

---

## [0.1.0] - 2026-04-25

### Added
- Initial implementation of compact number formatter
- Builder-style `NumberOptions` API
- `Humanize` extension trait for ergonomic usage
- Support for short and long unit suffixes (K / million style)
- Locale abstraction layer (foundation for future languages)
- Thousand separators support
- Precision control for formatted output
- Integration test suite for numeric formatting
- Smoke example demonstrating real-world usage
- Doctests for all public API entry points

### Notes
- This is the initial public-style release of the formatter core.
- API is still considered early-stage and may evolve before 1.0.
- Future work will introduce byte formatting, duration formatting, and locale expansions.

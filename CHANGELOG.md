# Changelog

All notable changes to `humfmt` will be documented in this file.

The format is based on Keep a Changelog and this project adheres to Semantic Versioning.

---

## [Unreleased]

### Added
- Criterion-based benchmark suite covering the core formatter paths
- Proptest-based invariant coverage for number, bytes, duration, relative-time, and list formatting
- `DurationConversionError` plus non-breaking `*_checked` helpers in `humfmt::chrono` and `humfmt::time`
- locale-aware list item separator hook via `Locale::list_separator()` and `CustomLocale::list_separator(...)`
- `ListOptions::serial_comma_enabled(bool)` and `ListOptions::conjunction(...)` for explicit list-style overrides
- additional property tests for suffix monotonicity and locale decimal-separator invariants
- benchmark coverage for list conjunction override options path
- standalone comparison benchmark harness under `tools/benchmarks/`
- comparison harness coverage for bytes, numbers, durations, and relative-time ("ago")
- benchmark report generator (`report` binary) that produces `BENCHMARKS.md`
- capability matrix in `BENCHMARKS.md` to make comparisons interpretable and fair
- combined dark-theme SVG charts under `assets/benchmarks/` (bytes, time, numbers)
- on-demand GitHub Actions workflow to run the comparison harness and upload artifacts

### Changed
- Core number and byte formatting paths were refactored to write directly to the `fmt::Formatter` (fewer intermediate allocations)
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

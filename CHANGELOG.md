# Changelog

All notable changes to `humfmt` will be documented in this file.

The format is based on Keep a Changelog and this project adheres to Semantic Versioning.

---

## [Unreleased]

### Added
- `bytes()` and `bytes_with(...)` helpers for human-readable byte sizes
- `BytesOptions` and `Humanize::human_bytes()` / `human_bytes_with(...)`
- `ordinal()` and `ordinal_with(...)` helpers for human-readable ordinal formatting
- `Humanize::human_ordinal()` and `Humanize::human_ordinal_with(...)`
- `duration()` and `duration_with(...)` helpers for compact human-readable durations
- `DurationOptions` and `Humanize::human_duration()` / `human_duration_with(...)`
- `ago()` and `ago_with(...)` helpers for relative time formatting
- `Humanize::human_ago()` and `human_ago_with(...)`
- optional `chrono` and `time` integration layers for signed durations and timestamp-based relative formatting

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

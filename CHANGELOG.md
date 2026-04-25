# Changelog

All notable changes to `humfmt` will be documented in this file.

The format is based on Keep a Changelog and this project adheres to Semantic Versioning.

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
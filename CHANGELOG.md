# Changelog

All notable changes to `humfmt` will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

### Added
- Percentage formatter: `percent(0.423) → "42.3%"`.
  - Free functions `percent(value)` and `percent_with(value, options)`.
  - `PercentDisplay<L>` implements `Display` with zero intermediate allocation.
  - `PercentOptions<L>` builder with `precision`, `fixed_precision`, and `locale`.
  - `PercentLike` sealed trait implemented for `f32` and `f64`.
  - `.human_percent()` / `.human_percent_with()` extension methods on `Humanize`.
  - Input is a ratio (`1.0 = 100%`); values outside `0.0..=1.0` are accepted.
  - Non-finite inputs render with a `%` suffix (`inf%`, `NaN%`).
  - Negative zero is suppressed (`-0.0004 → "0%"`, never `"-0%"`).
  - Locale-aware decimal separator via `.locale(locale)`.
- `NumberOptions::rounding(RoundingMode)` — control rounding logic via HalfUp (default), Floor or Ceil.
- `NumberOptions::compact(bool)` — allows completely disabling magnitude scaling (e.g. `1500` → `"1500"` instead of `"1.5K"`). This works perfectly with `separators(true)` to produce fully formatted large numbers like `"1,234,567"`.
- `NumberOptions`: behaviour tables added to rustdoc for `precision`, `long_units`, `separators`, and `fixed_precision`.
- `number` module: edge case and rounding behaviour table added to module docs.

### Changed
- `NumberOptions::separators()`: digit grouping separators apply only when the value is not compacted (suffix index 0). Documentation clarified, and it can now be effectively used with `compact(false)`.

### Fixed
- `number` formatter float path: removed unreachable fallback (`write_float_direct`) that silently ignored the locale decimal separator on `StackString` overflow. The buffer is always sufficient for `precision <= 6` so the fallback was never reached.
- `round_f64`: corrected comment — the real reason for integer-cast rounding is that `f64::round()` requires `std` or `libm` at MSRV 1.70 in a `no_std` build.
- `is_integer_f64`: removed `#[cfg(any(feature = "russian", feature = "polish"))]` guard that caused `dead_code` warnings in bare `no_std` builds. Replaced with `#[allow(dead_code)]`.

### Removed
- Dead internal code in `common/fmt.rs`: `StackString` methods `truncate`, `ends_with_byte`, `find_byte` and function `trim_ascii_trailing_zeros_and_dot` (unused after float path simplification).

---

## [0.4.0] - 2026-05-01

### Added
- `NumberOptions::fixed_precision(bool)` — opt-in mode that preserves trailing fractional zeros for consistent column widths (e.g. `1.50K` instead of `1.5K`).
- `BytesOptions::fixed_precision(bool)` — same opt-in mode for byte-size output (e.g. `1.50 KiB` instead of `1.5 KiB`).
- `BytesOptions::space(bool)` — optionally insert a space before short unit labels (e.g. `1.5 KB` instead of `1.5KB`).
- `BytesOptions::decimal_separator(char)` — overrides the decimal separator for scaled byte output (e.g. `1,5KB`).
- `BytesOptions::locale(locale)` — copies the decimal separator from any `Locale`.
- `DurationOptions::max_units` now accepts values up to `7` (previously clamped to `4`), enabling callers to render all supported time units down to nanoseconds.
- `common::numeric::is_integer_f64` — shared, correct, `no_std`-compatible helper for checking whether a `f64` value has no fractional part.
- MSRV CI job that compiles the library on Rust 1.70 on every push and pull request.

### Changed
- MSRV raised from **1.67** to **1.70** (released June 2023), aligning with `criterion 0.5` which already required 1.70.
- Float compact-number scaling rewritten from an O(n) loop to an O(1) IEEE 754 exponent-based approach, consistent with the integer path which uses `ilog10`.
- `DurationOptions::max_units` clamp widened from `1..=4` to `1..=7`. Values `5..=7` previously clamped silently to `4`; they now work as documented.
- `Locale::duration_unit` default implementation now delegates to `english::duration_unit` instead of duplicating the match expression.
- Internal `Options` types expose fields as `pub(crate)` directly, removing the `_value()`-suffixed getter layer. Public builder API is unchanged.

### Fixed
- `is_integer_f64` in `russian.rs` and `polish.rs`: the old `value == (value as u128) as f64` check incorrectly returned `false` for negative whole floats (e.g. `-1.0`, `-42.0`) because casting a negative `f64` to `u128` saturates to `0` on stable Rust. Fixed with `value.is_finite() && value % 1.0 == 0.0`.
- Polish long-form plural selection is now CLDR-aligned: `one` for `1`, `few` for integers ending in `2..=4` excluding `12..=14`, `many` for everything else.
- List formatting no longer injects a literal comma for serial-comma output when the list separator is not comma-style (e.g. `" | "`).
- Small negative floating-point values that round to zero no longer render as `-0`.

---

## [0.3.0] - 2026-04-28

### Added
- Standalone comparison benchmark harness under `tools/benchmarks/` covering bytes, numbers, durations, and relative-time.
- Benchmark report generator (`report` binary) that produces `BENCHMARKS.md` and dark-theme SVG charts under `assets/benchmarks/`.
- Capability matrix in `BENCHMARKS.md`.
- On-demand GitHub Actions workflow to run the comparison harness and upload artifacts.
- `DurationConversionError` and non-breaking `*_checked` helpers in `humfmt::chrono` and `humfmt::time`.
- `Locale::list_separator()` and `CustomLocale::list_separator(...)` for locale-aware list item separators.
- `ListOptions::serial_comma_enabled(bool)` and `ListOptions::conjunction(...)`.
- Full rustdoc coverage across the public API, enforced via `#![deny(missing_docs)]`.

### Changed
- Core number and byte formatting paths write directly to `fmt::Formatter`, reducing intermediate allocations.
- Compact integer scaling uses O(1) unit selection (`ilog10` + lookup table) instead of a division loop.
- `Sealed` implementations centralized in `common::sealed`.
- `StackString::as_str()` is now infallible based on an internal UTF-8 invariant.

---

## [0.2.0] - 2026-04-25

### Added
- `bytes()` / `bytes_with()` and `BytesOptions` for human-readable byte sizes.
- `ordinal()` / `ordinal_with()` for locale-aware ordinal formatting.
- `duration()` / `duration_with()` and `DurationOptions` for compact duration output.
- `ago()` / `ago_with()` for relative-time formatting.
- `list()` / `list_with()` and `ListOptions` for natural-language list formatting.
- Optional `chrono` integration: `chrono::TimeDelta` and `chrono::DateTime` adapters, including `ago_since()` / `ago_since_with()` for timestamp-based relative formatting.
- Optional `time` integration: `time::Duration` and `time::OffsetDateTime` adapters, including `ago_since()` / `ago_since_with()`.
- `Russian` locale pack (feature `russian`): compact numbers, ordinals, duration units, list conjunction.
- `Polish` locale pack (feature `polish`): compact numbers, ordinals, duration units, list conjunction.
- `CustomLocale` builder for ad hoc suffix, separator, ordinal, duration-unit, list conjunction, and `ago` word customization.
- `Humanize` extension methods: `human_bytes`, `human_ordinal`, `human_duration`, `human_ago`.
- `ChronoHumanize` / `TimeHumanize` extension traits gated behind feature flags.

### Changed
- Localized compact number separators now come from the active locale.
- `DurationOptions` carries locale selection for duration and relative-time output.
- `CustomLocale` supports overriding the default list conjunction style.
- Expanded rustdoc coverage across modules; docs.rs configured to build with all
  features enabled.

---

## [0.1.1] - 2026-04-25

### Fixed
- Restored the `no_std` / `default-features = false` build path.
- Normalized `-0.0` formatting to `"0"`.
- Preserved non-finite float rendering (`inf`, `-inf`, `NaN`).

### Changed
- Expanded compact number suffix coverage beyond trillion.
- Tightened CI to exercise `--all-features` and minimal-feature builds.

---

## [0.1.0] - 2026-04-25

### Added
- Initial implementation of compact number formatter with `number()` / `number_with()` and `NumberOptions`.
- `Humanize` extension trait for ergonomic `.human_number()` usage.
- Short and long unit suffixes (`K` / `" thousand"` style).
- Locale abstraction layer (`Locale` trait, `English` built-in).
- Digit grouping separators and configurable precision.

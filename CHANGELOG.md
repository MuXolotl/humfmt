# Changelog

All notable changes to `humfmt` will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased → 0.7.0]

### Added

- Fuzz testing harness using `cargo-fuzz` with targets for all major formatters (`number`, `bytes`, `percent`, `duration`, `ordinal`, `list`). The harness can be run manually via GitHub Actions.
- Golden (snapshot) tests (`tests/golden.rs`) to act as a strict regression net for all formatters before the 1.0 release.

### Changed

- Refactored `percent` formatter to reuse shared fractional digit writing logic, removing duplicated `unsafe` blocks and reducing internal code bloat.
- Number formatter suffix range now extends to `Ud` / `undecillion` (`10^36`). This improves compact output for the full `u128` / `i128` range:
  - `u128::MAX` now formats as `"340.3Ud"` instead of a very large `Dc` value.
  - `i128::MIN` now formats as `"-170.1Ud"` instead of a very large `Dc` value.
- This is an intentional output change for values at or above `10^36`.

### Fixed

- Fixed overflow-prone fractional digit extraction in the shared `u128` long-division formatter path. Extreme values near `u128::MAX` now round correctly instead of being distorted by saturating intermediate multiplication.
- Hardened significant-digit rounding for extreme integer values such as `u128::MAX`, `i128::MAX`, and `i128::MIN`.

---

## [0.6.0] - 2026-05-16

This release removes the entire i18n / locale subsystem to refocus the crate as a fast, English-only formatting toolkit. Locale-related complexity was holding back development; this is a deliberate scope reduction.

If you need locale-aware formatting, pin to `0.5.x` and use the `russian` / `polish` feature flags from that release line.

### Removed (BREAKING)

- Entire `humfmt::locale` module:
  - `Locale` trait
  - `English`, `Russian`, `Polish` types
  - `CustomLocale` builder and all its hook function pointers
  - `DurationUnit` enum (was used only by locale callbacks)
- All locale feature flags: `english`, `russian`, `polish`.
- The placeholder `alloc` feature (was unused).
- `ordinal_with` function (collapsed into `ordinal` since there are no locales left to switch).
- `Humanize::human_ordinal_with` method.
- `*Options::locale(...)` methods on `NumberOptions`, `PercentOptions`, `BytesOptions`, `DurationOptions`, `ListOptions`.
- `CustomLocale::list_separator` is replaced by the new `ListOptions::separator`.

### Changed (BREAKING)

- `NumberOptions`, `PercentOptions`, `BytesOptions`, `DurationOptions`, `ListOptions` no longer have a `<L: Locale>` generic parameter.
- `NumberDisplay`, `PercentDisplay`, `BytesDisplay`, `DurationDisplay`, `AgoDisplay`, `OrdinalDisplay`, `ListDisplay` no longer have a `<L: Locale>` generic parameter.
- `chrono::*` and `time::*` integration functions and their corresponding trait methods (`ChronoHumanize`, `TimeHumanize`) no longer have a `<L: Locale>` generic parameter.
- `ListOptions` is restructured: `serial_comma` and `conjunction` are now stored as plain values (with English defaults baked into `new()`), not `Option<...>` overrides on top of locale defaults.

### Added

- `NumberOptions::decimal_separator(char)` and `NumberOptions::group_separator(char)` as direct replacements for the removed `.locale(...)` configuration.
- `PercentOptions::decimal_separator(char)` (same purpose).
- `ListOptions::separator(&'static str)` to override the inter-item separator (replaces `CustomLocale::list_separator`).
- Public `humfmt::ordinal::ordinal_suffix(n: u128) -> &'static str` helper for callers that want just the suffix without the value.
- `numfmt` (kurtlawrence/numfmt) added to the comparison harness as an honest competitor for the `number` formatter.
- New `formatter_benches.rs` groups: `custom_separators`, `uncompacted_grouped`, `custom_conjunction`, `custom_separator`.

### Improved

- All builder methods on `NumberOptions`, `PercentOptions`, `BytesOptions`, `DurationOptions`, `ListOptions` are now `const fn` — usable in `const` contexts.
- `StackString` buffer in the float number path shrunk from 512 to 384 bytes (still fits any non-exponential `f64` with full precision and headroom).
- `unsafe { from_utf8_unchecked }` blocks now have `debug_assert!` guards in debug builds to catch invariant regressions.
- `Humanize` blanket impl is explicitly documented (blanket `impl<T> Humanize for T {}` with per-method `where` bounds — keeps imports tiny while staying type-safe).
- `BENCHMARKS.md` capability matrix updated: dropped `Locale-aware` and `Custom locale builder` rows, added `Custom decimal/group separators` and the `numfmt` column.

### Migration

Before:

```rust
use humfmt::{number_with, NumberOptions};
use humfmt::locale::Russian;

let opts = NumberOptions::new().locale(Russian);
println!("{}", number_with(15_320, opts)); // "15,3 тыс."
```

After (English-only output, but custom separators if you want European-style):

```rust
use humfmt::{number_with, NumberOptions};

let opts = NumberOptions::new()
    .decimal_separator(',')
    .group_separator(' ');
println!("{}", number_with(15_320, opts)); // "15,3K"
```

Locale-aware list formatting:

```rust
// Before
use humfmt::locale::CustomLocale;
let locale = CustomLocale::english().and_word("plus").list_separator(" | ");

// After
use humfmt::ListOptions;
let opts = ListOptions::new().conjunction("plus").separator(" | ");
```

---

## [0.5.2] - 2026-05-04

### Changed
- `README.md` rewritten: added `percent` to Quick Example, `bits` mode, `force_sign`, `significant_digits`, `compact(false)` + `separators`, unit forcing (`ByteUnit`). Added `crates.io` and `docs.rs` badges. Added feature flags table. Simplified and restructured for readability.
- `docs/CRATE.md` rewritten: added `percent` formatter section with defaults table and edge-case table. Updated `NumberOptions` defaults table with `significant_digits`, `compact`, `force_sign`, `rounding`. Added behaviour tables for `significant_digits`, `compact`, `force_sign`, `rounding` in `NumberOptions` rustdoc. Updated `BytesOptions` defaults table with `bits`, `significant_digits`, `rounding`, `unit`, `min_unit`, `max_unit`. Added bits mode, unit forcing, significant digits, and rounding sections to `BytesOptions` docs. Added edge-case behaviour tables for `bytes`, `percent`, `ordinal`, `duration`, and `list`. Added locale features matrix. Added cookbook examples across all formatters.
- `src/rounding.rs`: added doc comment with examples and behaviour table for `RoundingMode` enum.
- `src/number/options.rs`: added behaviour tables to `significant_digits`, `compact`, `force_sign`, and `rounding` methods in rustdoc.
- `src/bytes/options.rs`: added behaviour tables to `bits`, `significant_digits`, and `rounding` methods. Clarified `locale()` method docs to note it currently copies only `decimal_separator`. Added scaling notes to `min_unit` and `max_unit`.
- `src/percent/options.rs`: added quick reference table and behaviour tables to `precision`, `force_sign`, and `fixed_precision` methods.
- `src/bytes/mod.rs`: added edge-case behaviour table and cookbook examples to module docs.
- `src/percent/mod.rs`: added edge-case behaviour table and cookbook examples to module docs.
- `src/ordinal/mod.rs`: added edge-case behaviour table (English/Russian/Polish columns). Documented Russian ordinal gender limitation.
- `src/duration/mod.rs`: added edge-case behaviour table and output control section to module docs.
- `src/ago/mod.rs`: documented limitations — future-time support not yet implemented, no "just now" special case.
- `src/list/mod.rs`: added edge-case behaviour table and serial comma explanation for non-comma separators to module docs.
- `src/common/numeric.rs`: added doc examples to `is_integer_f64`.

---

## [0.5.0] - 2026-05-04

### Added
- Percentage formatter: `percent(0.423) → "42.3%"`.
  - Free functions `percent(value)` and `percent_with(value, options)`.
  - `PercentDisplay<L>` implements `Display` with zero intermediate allocation.
  - `PercentOptions<L>` builder with `precision`, `fixed_precision`, `force_sign`, and `locale`.
  - `PercentLike` sealed trait implemented for `f32` and `f64`.
  - `.human_percent()` / `.human_percent_with()` extension methods on `Humanize`.
  - Input is a ratio (`1.0 = 100%`); values outside `0.0..=1.0` are accepted.
  - Non-finite inputs render with a `%` suffix (`inf%`, `NaN%`).
  - Negative zero is suppressed (`-0.0004 → "0%"`, never `"-0%"`).
  - Locale-aware decimal separator via `.locale(locale)`.
- `NumberOptions::compact(bool)` — allows completely disabling magnitude scaling (e.g. `1500` → `"1500"` instead of `"1.5K"`). This works perfectly with `separators(true)` to produce fully formatted large numbers like `"1,234,567"`.
- `NumberOptions::significant_digits(u8)` — allows formatting values to a fixed number of significant digits instead of a fixed number of decimal places (e.g., `1234` with 3 sig figs outputs `"1.23K"`, or `"1230"` if unscaled).
- `NumberOptions::rounding(RoundingMode)` — control rounding logic via `HalfUp` (default), `Floor` or `Ceil`.
- `NumberOptions::force_sign(bool)` and `PercentOptions::force_sign(bool)` — strictly force a `+` sign for positive values.
- `BytesOptions::rounding(RoundingMode)` and `BytesOptions::significant_digits(u8)` — bringing exact API parity and feature alignment between the `bytes` and `number` formatters.
- `BytesOptions::unit(ByteUnit)`, `min_unit(ByteUnit)`, and `max_unit(ByteUnit)` — allows forcing the output to a specific magnitude (e.g. always `MB`) or clamping the range of automatic scaling.
- `BytesOptions::bits(bool)` — multiplies the input by 8 and formats it using bit units (`b`, `Kb`, `Mb`, etc.). Useful for network speeds and bandwidth formatting.
- `NumberOptions`: behaviour tables added to rustdoc for `precision`, `long_units`, `separators`, and `fixed_precision`.
- `number` module: edge case and rounding behaviour table added to module docs.

### Changed
- `NumberOptions::separators()`: digit grouping separators apply only when the value is not compacted (suffix index 0). Documentation clarified, and it can now be effectively used with `compact(false)`.
- Extracted core sig-figs logic (`compute_sigfigs_u128`) into `common::fmt` to be cleanly shared between formatters.
- Floating point math in `number`, `bytes`, and `percent` formatters has been completely rewritten to use internal custom functions (`f64_log10_floor`, `f64_pow10`) to avoid pulling `libm` or `std` math functions, guaranteeing blazing fast and pure `no_std` execution.

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

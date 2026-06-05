# Changelog

All notable changes to `humfmt` will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [[Unreleased](https://github.com/MuXolotl/humfmt/compare/v0.6.0...HEAD)]

### Added

- Fuzz testing harness (`cargo-fuzz`) with targets for `number`, `bytes`, `percent`, `duration`, `ordinal`, and `list`. Run via GitHub Actions or locally with `cargo +nightly fuzz run <target>`.
- Golden snapshot tests (`tests/golden.rs`) as a strict regression net across all formatters ahead of the 1.0 API freeze.
- `PercentOptions::rounding(RoundingMode)` — brings `percent` into full API parity with `number` and `bytes`. Previously `percent` always used half-up; `Floor` and `Ceil` are now available.
- `ByteUnit` and `RoundingMode` re-exported from `humfmt::prelude` so a single `use humfmt::prelude::*` covers the most common option types.

### Changed

- Number formatter suffix range extended to `Ud` / undecillion (`10^36`), covering the full `u128` / `i128` range without falling back to very large `Dc` values. `u128::MAX` now formats as `"340.3Ud"`, `i128::MIN` as `"-170.1Ud"`. This is an intentional output change for values ≥ `10^36`.
- `percent` formatter refactored to use the same rounding infrastructure as `number` and `bytes`, removing the previously hardcoded half-up path.

### Fixed

- Overflow-safe fractional digit extraction in the `u128` long-division path. Extreme values near `u128::MAX` now round correctly.
- Significant-digit rounding for `u128::MAX`, `i128::MAX`, and `i128::MIN`.

---

## [[0.6.0](https://github.com/MuXolotl/humfmt/compare/v0.5.2...v0.6.0)] - 2026-05-16

This release removes the entire i18n / locale subsystem to refocus the crate as a fast, English-only formatting toolkit. If you need locale-aware output, pin to `0.5.x`.

### Added

- `NumberOptions::decimal_separator(char)` and `group_separator(char)` as direct replacements for the removed `.locale(...)` configuration.
- `PercentOptions::decimal_separator(char)`.
- `ListOptions::separator(&'static str)` — replaces `CustomLocale::list_separator`.
- `humfmt::ordinal::ordinal_suffix(n: u128) -> &'static str` public helper.
- `numfmt` added to the comparison benchmark harness.
- New benchmark groups: `custom_separators`, `uncompacted_grouped`, `custom_conjunction`, `custom_separator`.

### Changed

- All builder methods on `NumberOptions`, `PercentOptions`, `BytesOptions`, `DurationOptions`, and `ListOptions` are now `const fn`.
- `StackString` in the float number path shrunk from 512 to 384 bytes.
- `ListOptions`: `serial_comma` and `conjunction` are now plain values with English defaults baked into `new()`, not `Option<…>` overrides.
- `unsafe { from_utf8_unchecked }` blocks now have `debug_assert!` guards in debug builds.

### Removed

- Entire `humfmt::locale` module: `Locale` trait, `English`, `Russian`, `Polish` types, `CustomLocale` builder, `DurationUnit` enum, and all `*Options::locale(…)` methods.
- Feature flags: `english`, `russian`, `polish`, `alloc`.
- `ordinal_with` function and `Humanize::human_ordinal_with` method.

### Migration

```rust
// Before
use humfmt::{number_with, NumberOptions};
use humfmt::locale::Russian;
let opts = NumberOptions::new().locale(Russian);

// After — English output, European-style separators if needed
use humfmt::{number_with, NumberOptions};
let opts = NumberOptions::new()
    .decimal_separator(',')
    .group_separator(' ');
```

```rust
// Before
use humfmt::locale::CustomLocale;
let locale = CustomLocale::english().and_word("plus").list_separator(" | ");

// After
use humfmt::ListOptions;
let opts = ListOptions::new().conjunction("plus").separator(" | ");
```

---

## [[0.5.2](https://github.com/MuXolotl/humfmt/compare/v0.5.0...v0.5.2)] - 2026-05-04

### Changed

- `README.md` rewritten: added `percent`, `bits` mode, `force_sign`, `significant_digits`, `compact(false)` + `separators`, unit forcing, badges, feature flags table.
- `docs/CRATE.md` rewritten with expanded formatter sections, behaviour tables, edge-case tables, and cookbook examples.
- Rustdoc coverage improved across `number`, `bytes`, `percent`, `duration`, `ago`, `list`, and `ordinal` modules.

---

## [[0.5.0](https://github.com/MuXolotl/humfmt/compare/v0.4.0...v0.5.0)] - 2026-05-04

### Added

- `percent(value)` / `percent_with(value, options)` formatter. Input is a ratio (`1.0 = 100%`); values outside `0.0..=1.0` are accepted. Non-finite inputs render with a `%` suffix. Negative zero is suppressed.
- `NumberOptions::compact(bool)` — disable magnitude scaling. Pairs with `separators(true)` to produce `"1,234,567"` style output.
- `NumberOptions::significant_digits(u8)` — round to N total significant digits.
- `NumberOptions::rounding(RoundingMode)` with `HalfUp` (default), `Floor`, `Ceil`.
- `NumberOptions::force_sign(bool)` and `PercentOptions::force_sign(bool)`.
- `BytesOptions::rounding(RoundingMode)` and `significant_digits(u8)`.
- `BytesOptions::unit(ByteUnit)`, `min_unit(ByteUnit)`, `max_unit(ByteUnit)`.
- `BytesOptions::bits(bool)` — multiply by 8 and use bit units (`Kb`, `Mb`, …).

### Changed

- `NumberOptions::separators()`: grouping separators now only apply when the value is uncompacted (no suffix). Works seamlessly with `compact(false)`.
- Shared sig-figs logic (`compute_sigfigs_u128`) moved to `common::fmt`.
- Float scaling in `number`, `bytes`, and `percent` rewritten to use custom `no_std`-compatible helpers (`f64_log10_floor`, `f64_pow10`), removing any dependency on `libm` or `std` math.

### Fixed

- `number` float path: removed an unreachable fallback that silently dropped the decimal separator on `StackString` overflow.
- `round_f64` comment corrected — integer-cast rounding is required because `f64::round()` needs `std` or `libm` at MSRV 1.70 in `no_std` builds.
- `is_integer_f64`: removed a `#[cfg]` guard that caused `dead_code` warnings in bare `no_std` builds.

### Removed

- Dead internal helpers in `common/fmt.rs`: `StackString::truncate`, `ends_with_byte`, `find_byte`, `trim_ascii_trailing_zeros_and_dot`.

---

## [[0.4.0](https://github.com/MuXolotl/humfmt/compare/v0.3.0...v0.4.0)] - 2026-05-01

### Added

- `NumberOptions::fixed_precision(bool)` — keep trailing fractional zeros.
- `BytesOptions::fixed_precision(bool)` — same for byte output.
- `BytesOptions::space(bool)` — space before short unit labels.
- `BytesOptions::decimal_separator(char)`.
- `DurationOptions::max_units` now accepts `1..=7` (was `1..=4`).
- `common::numeric::is_integer_f64` shared `no_std`-compatible helper.
- MSRV CI job (Rust 1.70).

### Changed

- MSRV raised from 1.67 to **1.70** (June 2023), aligning with `criterion 0.5`.
- Float compact-number scaling rewritten from O(n) loop to O(1) via IEEE 754 exponent.
- `DurationOptions::max_units` clamp widened from `1..=4` to `1..=7`.
- Internal `Options` types expose fields as `pub(crate)` directly, removing the `_value()`-suffixed getter layer.

### Fixed

- `is_integer_f64`: the old `value == (value as u128) as f64` check returned `false` for negative whole floats. Fixed with `value.is_finite() && value % 1.0 == 0.0`.
- Polish plural selection now CLDR-aligned.
- List formatting no longer injects a literal comma when the separator is not comma-style.
- Small negative floats that round to zero no longer render as `-0`.

---

## [[0.3.0](https://github.com/MuXolotl/humfmt/compare/v0.2.0...v0.3.0)] - 2026-04-28

### Added

- Standalone comparison benchmark harness under `tools/benchmarks/`.
- Report generator binary producing `BENCHMARKS.md` and dark-theme SVG charts.
- Capability matrix in `BENCHMARKS.md`.
- On-demand GitHub Actions workflow for the comparison harness.
- `DurationConversionError` and `*_checked` helpers in `humfmt::chrono` and `humfmt::time`.
- `Locale::list_separator()` and `CustomLocale::list_separator(…)`.
- `ListOptions::serial_comma_enabled(bool)` and `ListOptions::conjunction(…)`.
- Full rustdoc coverage enforced via `#![deny(missing_docs)]`.

### Changed

- Core number and byte formatting paths write directly to `fmt::Formatter`, reducing intermediate allocations.
- Compact integer scaling uses O(1) unit selection (`ilog10` + lookup table).
- `Sealed` implementations centralised in `common::sealed`.
- `StackString::as_str()` is now infallible.

---

## [[0.2.0](https://github.com/MuXolotl/humfmt/compare/v0.1.1...v0.2.0)] - 2026-04-25

### Added

- `bytes()` / `bytes_with()` and `BytesOptions` (decimal SI + binary IEC).
- `ordinal()` / `ordinal_with()` with locale-aware markers.
- `duration()` / `duration_with()` and `DurationOptions` with `max_units`.
- `ago()` / `ago_with()` for relative-time formatting.
- `list()` / `list_with()` and `ListOptions` for natural-language lists.
- Optional `chrono` integration: `TimeDelta`, `DateTime`, `ago_since()`.
- Optional `time` integration: `time::Duration`, `OffsetDateTime`, `ago_since()`.
- `Russian` locale pack (feature `russian`).
- `Polish` locale pack (feature `polish`).
- `CustomLocale` builder.
- `Humanize` extension methods: `human_bytes`, `human_ordinal`, `human_duration`, `human_ago`.
- `ChronoHumanize` / `TimeHumanize` extension traits.

### Changed

- Localized compact number separators come from the active locale.
- `DurationOptions` carries locale selection.

---

## [[0.1.1](https://github.com/MuXolotl/humfmt/compare/v0.1.0...v0.1.1)] - 2026-04-25

### Fixed

- Restored the `no_std` / `default-features = false` build path.
- Normalised `-0.0` formatting to `"0"`.
- Preserved non-finite float rendering (`inf`, `-inf`, `NaN`).

### Changed

- Expanded compact number suffix coverage beyond trillion.
- Tightened CI to exercise `--all-features` and minimal-feature builds.

---

## [[0.1.0](https://github.com/MuXolotl/humfmt/compare/c61a85d110b72f2e0da0ced5889ac9814a4f1580...v0.1.0)] - 2026-04-25

### Added

- Initial implementation of compact number formatter with `number()` / `number_with()` and `NumberOptions`.
- `Humanize` extension trait for `.human_number()`.
- Short and long unit suffixes (`K` / `" thousand"` style).
- Locale abstraction layer (`Locale` trait, `English` built-in).
- Digit grouping separators and configurable precision.

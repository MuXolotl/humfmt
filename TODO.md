# TODO

A running list of planned features, known issues, and nice-to-haves.

Contributions are welcome — if you want to work on something, open an issue or a draft PR first so we do not step on each other's toes.

---

| Priority | Formatter |
|-|-|
| 1 | number |
| 2 | bytes |
| 3 | duration + ago |
| 4 | list |
| 5 | ordinal |
| 6 | percentage |
| ... | other |

---

## PLANNED

- [ ] Add future-time support to `ago` — right now it only formats past durations. Should support `"in 5 minutes"` for future timestamps alongside the existing `"5 minutes ago"` style, with a clean locale hook for the "in" word.
- [ ] Add `"just now"` / `"now"` / `"moments ago"` special cases to `ago` — for very small durations (e.g. under a configurable threshold like 5 seconds) it looks odd to print `"0s ago"` when the user probably wants `"just now"`.
- [ ] Significant-digits mode — instead of decimal places, round to N total significant digits. Useful for scientific or telemetry output.
- [ ] Rounding mode control — let the caller choose between default half-up rounding, floor, and ceil. Keep the API simple: one enum or three builder methods.
- [ ] Rate / throughput formatter — `1_200_000 -> "1.2 MB/s"`, `42_000 -> "42K ops/s"`. Should reuse existing byte and number formatting logic rather than duplicating it.
- [ ] Ratio formatter — `0.75 -> "3:4"` or `"75%"` depending on options.
- [ ] Temperature formatter — `36.6 -> "36.6°C"` / `"97.9°F"`, with configurable unit and locale-aware decimal separator. Low priority, but fits the theme.
- [ ] Fraction-aware pluralization — floats like `1.0` and `1.5` should behave correctly across locales. Right now `1.5 тысячи` works but the boundary conditions for e.g. Polish need edge-case testing.
- [ ] Duration formatting: configurable join string between units — let the caller choose between `"1h 2m"` (space-joined, current default), `"1h, 2m"` (comma-joined), or `"1 hour and 2 minutes"` (and-joined in long mode).
- [ ] Byte formatter: allow forcing a specific unit — e.g. always render in MB regardless of value size, useful for dashboards and log lines where consistent column width matters.
- [ ] Byte formatter: clamp min/max unit — stop the formatter from jumping all the way to EB when you want output to stay in MB/GB range.
- [ ] Byte formatter: bits mode — `Kb`, `Mb`, `Gb` using the same 1000-based infrastructure. Useful for network throughput display.
- [ ] Number formatter: always-on grouping separators option — right now `separators(true)` only applies when the value is not compacted. Add a way to show `1,234` instead of `1.2K` when the caller explicitly wants raw grouped output.
- [ ] Number formatter: fully disable compact scaling — a way to say "never compact this number, just group the digits". Currently achievable via `CustomLocale::max_compact_suffix_index(0)` but it is not obvious.
- [ ] List formatter: `"or"` conjunction style — `"red, green, or blue"` alongside the existing `"and"` style.
- [ ] List formatter: better handling of edge cases — single-item and empty-list behavior should be explicitly documented with tests, since they are silent no-ops right now.
- [ ] More locale packs — German, French, and Spanish are the obvious next additions since they cover a large chunk of real-world users. Native speaker review of plural rules is important before publishing these.
- [ ] Locale system: make plural rules easier to reason about and extend — the current `plural_form_int` helpers in Russian and Polish are readable but each locale reimplements the same pattern. A shared helper in `common` would make adding new locales safer.
- [ ] Locale system: allow overriding the `"in"` word cleanly for future-time formatting alongside the existing `ago_word` hook.
- [ ] Add golden output test files per locale — a set of fixed inputs and expected outputs stored in test data files so that locale changes never silently break grammar without a test failure.
- [ ] Cookbook-style documentation on docs.rs — short, focused examples for common scenarios: "how do I format bytes", "how do I add my own locale", "what happens with zero / negative / very large values". No walls of text, just code and a one-line explanation.
- [ ] Edge-case behavior tables in the docs — a quick reference showing what each formatter does with `0`, `i128::MIN`, `u128::MAX`, `f64::NAN`, `f64::INFINITY`, and `Duration::MAX`.
- [ ] More real-world examples — CLI progress output, log lines, dashboard numbers. Helps new users immediately see where the crate fits.
- [ ] Explicit MSRV CI job — done, see 0.4.0. Keep this note as a reminder to re-verify after any MSRV bump.
- [ ] Stable public API snapshot before 1.0 — lock down the formatter surface so downstream crates can rely on it without surprises across minor versions.
- [ ] Final API consistency pass before 1.0 — make sure all formatters follow the same naming patterns and builder method conventions with no small inconsistencies left.
- [ ] Add more comparison crates to the benchmark harness — `readable`, `human-readable`, `fancy-duration`, and `duration-human` are missing and cover overlapping functionality. Honest comparison means including them.
- [ ] Improve benchmark alignment — add explicit scenarios that match common real-world output styles (e.g. binary + `precision(2)` + `space(true)`) so the capability matrix stays fair and easy to interpret.
- [ ] `no_std + alloc` tier — right now it is either full `std` or truly bare `no_std`. An `alloc`-gated tier would let embedded targets with a heap use `.to_string()` without pulling in all of `std`. More useful than it sounds for embedded targets.
- [ ] f64 precision loss in `compact_suffix_for` for values above 2^53 — document as a known limitation. The `as_f64()` conversion on `DecimalParts` loses integer precision for very large magnitudes. For display purposes this is rarely visible, but worth a note in docs.
- [ ] Document the Russian ordinal gender limitation — `ordinal_suffix` for Russian always returns `-й` (masculine). The library has no concept of grammatical gender since it only receives a number. This should be explicitly noted in the Russian locale docs and in the edge-case behavior tables.
- [ ] Investigate `is_comma_style_separator` Unicode edge cases — the current check finds the first non-whitespace character and compares it to `,`. This works for ASCII separators but will not recognize `،` (U+060C Arabic comma) or `、` (U+3001 Ideographic comma). Low priority until non-Latin list separators are needed.
- [ ] Number formatter: `separators(true)` currently applies only at suffix index 0 (unscaled output). Consider whether it should also apply to the integer part of compacted output (e.g. `1,234.5K` for very large scaled values). Document the current behaviour explicitly and decide before 1.0.
- [ ] Percentage formatter: consider accepting integer inputs (e.g. `42_u8` meaning `42%` directly, without the `* 100` ratio convention). Could be a separate `percent_ratio` vs `percent_value` split, or an option flag. Decide before 1.0.
- [ ] Percentage formatter: `+` sign option for positive values — `"+42.3%"` style useful for delta/change displays (e.g. portfolio gains).

---

## MAYBE SOMEDAY (no promises)

- [ ] Optional ICU4X integration — high-fidelity locale behavior (pluralization, list formatting, relative time) backed by Unicode CLDR data. This would be a heavy optional feature, definitely not the default.
- [ ] `serde` feature — serialize and deserialize options structs, useful for config-file driven formatting. Not needed until someone asks for it.
- [ ] `num-bigint` integration — compact formatting of arbitrary-precision integers. Very niche.
- [ ] Human-readable ranges — `"1–5 MB"`, `"~3 hours"` for UI-style approximate output.
- [ ] Currency formatter — lightweight, definitely not trying to compete with full i18n libraries. Only makes sense if locale support grows significantly first.
- [ ] Scientific notation formatter — `1.23e9`, with locale-aware decimal separator and optional compact form.
- [ ] Grammar-aware unit forms — case and gender agreement for languages that need it (e.g. Russian genitive after 2–4). The current approach covers most cases but is not linguistically complete.
- [ ] WASM and embedded target smoke tests in CI.
- [ ] Fuzzing harness for the formatting paths — finding edge cases in the integer math and float rendering code.

---

## DONE

### (Unreleased → 0.5.0)
- [x] ~~Percentage formatter — `0.423 → "42.3%"`, locale-aware decimal separator, configurable precision, fixed_precision, f32/f64 input, Humanize trait methods~~
- [x] ~~Fix `format_float` fallback path: locale decimal separator was ignored when`StackString` overflowed~~
- [x] ~~Fix incorrect `no_std` comment in `round_f64`~~
- [x] ~~Remove feature-gated `#[cfg]` guard from `is_integer_f64`, replace with `#[allow(dead_code)]`~~
- [x] ~~Clarify `NumberOptions::separators()` documentation~~
- [x] ~~Edge-case test coverage for `number` formatter (zero, one, -1, i128::MIN, u128::MAX, precision(0), sign symmetry, separators semantics, small floats, suffix boundaries)~~

### v0.4.0 released
- [x] ~~MSRV raised to 1.70 and enforced by a dedicated CI job~~
- [x] ~~`NumberOptions::fixed_precision(bool)` — opt-in trailing-zero preservation~~
- [x] ~~`BytesOptions::fixed_precision(bool)` — opt-in trailing-zero preservation~~
- [x] ~~Float compact-number scaling rewritten to O(1) via IEEE 754 exponent — consistent with integer `ilog10` path~~
- [x] ~~Byte label arrays refactored from 6 flat arrays to 2 `UnitLabels` struct arrays~~
- [x] ~~Fix `is_integer` in `russian.rs` and `polish.rs` — use `value % 1.0 == 0.0` instead of the broken `value == (value as u128) as f64` cast that saturates for negative floats~~
- [x] ~~Extract the shared `is_integer` helper out of `russian.rs` and `polish.rs` into `common::numeric`~~
- [x] ~~Remove the duplicated English duration unit logic from `Locale::duration_unit` default impl in `traits.rs`~~
- [x] ~~Expand `max_units` clamp from `1..=4` to `1..=7` so callers can render all seven duration units~~
- [x] ~~Rename or remove the `_value()` suffix on `pub(crate)` getter methods across `BytesOptions`, `NumberOptions`, `DurationOptions`, and `ListOptions`~~
- [x] ~~Comparison harness covers `humansize` baseline (SI + aligned IEC + signed)~~
- [x] ~~Comparison harness covers `human-repr` with output examples in `BENCHMARKS.md`~~
- [x] ~~Comparison harness covers `indicatif::HumanBytes` and aligned byte groups~~
- [x] ~~Optional spacing in short byte output via `BytesOptions::space(bool)`~~
- [x] ~~Byte formatter locale-aware decimal separator~~
- [x] ~~Float compact-number formatting stable on `no_std` MSRV~~
- [x] ~~Polish plural rules CLDR-aligned for long-form output~~
- [x] ~~Shrink `StackString<512>` to `StackString<64>`~~
- [x] ~~`ListOptions::serial_comma_enabled(bool)` and `ListOptions::conjunction`~~
- [x] ~~`locale::CustomLocale` list separator hook (`list_separator`)~~
- [x] ~~On-demand GitHub Actions benchmark workflow~~
- [x] ~~CI: test suite + clippy + fmt + feature matrix + `no_std` check~~

### v0.3.0 released
- [x] ~~docs.rs all-features build~~
- [x] ~~`#![deny(missing_docs)]` + full public API rustdoc coverage~~
- [x] ~~Centralized `Sealed` trait infrastructure~~
- [x] ~~O(1) compact integer scaling via `ilog10` / `ilog2`~~
- [x] ~~Property tests with `proptest` (sign symmetry, monotonicity, locale invariants)~~
- [x] ~~Capability matrix in `BENCHMARKS.md`~~
- [x] ~~Auto-generated `BENCHMARKS.md` + dark-theme SVG charts~~
- [x] ~~Standalone comparison benchmark harness (`tools/benchmarks/`)~~
- [x] ~~Criterion benchmark suite for the core formatter paths~~
- [x] ~~`DurationConversionError` + `*_checked` helpers~~

### v0.2.0 released
- [x] ~~Optional `time` integration (`Duration`, `OffsetDateTime`)~~
- [x] ~~Optional `chrono` integration (`TimeDelta`, `DateTime`)~~
- [x] ~~`CustomLocale` builder for ad hoc suffix / separator / hook overrides~~
- [x] ~~English, Russian, and Polish locale packs~~
- [x] ~~Natural-language list formatter~~
- [x] ~~Relative-time ("ago") formatter~~
- [x] ~~Duration formatter with configurable `max_units`~~
- [x] ~~Ordinal formatter~~
- [x] ~~Byte-size formatter (decimal SI + binary IEC)~~

### v0.1.x released
- [x] ~~`no_std` compatible build~~
- [x] ~~Compact number formatter with short and long units~~

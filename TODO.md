# TODO

A running list of planned features, known issues, and nice-to-haves.

Contributions are welcome — if you want to work on something, open an issue or a draft PR first so we do not step on each other's toes.

---

| Priority | Formatter | Progress (Est.) |
|:---:|---|:---:|
| 1 | number | ~95% |
| 2 | bytes | ~70% |
| 3 | duration + ago | ~75% |
| 4 | list | ~85% |
| 5 | percentage | ~85% |
| 6 | ordinal | ~90% |
| ... | other | 0% |

---

## PLANNED

- [ ] Byte formatter: bits mode — `Kb`, `Mb`, `Gb` using the same 1000-based infrastructure. Useful for network throughput display.
- [ ] Number & Percent formatters: `+` sign option for positive values — `"+1.5K"`, `"+42.3%"` style useful for delta/change displays.
- [ ] `BytesOptions::locale()` semantics — currently copies only `decimal_separator` from the locale, ignoring everything else. Either copy all relevant fields or rename to `decimal_separator_from_locale()` to make the limited scope explicit. Decide before 1.0.
- [ ] Add future-time support to `ago` — right now it only formats past durations. Should support `"in 5 minutes"` for future timestamps alongside the existing `"5 minutes ago"` style, with a clean locale hook for the "in" word.
- [ ] Add `"just now"` / `"now"` / `"moments ago"` special cases to `ago` — for very small durations (e.g. under a configurable threshold like 5 seconds) it looks odd to print `"0s ago"` when the user probably wants `"just now"`.
- [ ] Rate / throughput formatter — `1_200_000 -> "1.2 MB/s"`, `42_000 -> "42K ops/s"`. Should reuse existing byte and number formatting logic rather than duplicating it.
- [ ] Ratio formatter — `0.75 -> "3:4"` or `"75%"` depending on options.
- [ ] Duration formatting: configurable join string between units — let the caller choose between `"1h 2m"` (space-joined, current default), `"1h, 2m"` (comma-joined), or `"1 hour and 2 minutes"` (and-joined in long mode).
- [ ] List formatter: `"or"` conjunction style — `"red, green, or blue"` alongside the existing `"and"` style.
- [ ] List formatter: better handling of edge cases — single-item and empty-list behavior should be explicitly documented with tests, since they are silent no-ops right now.
- [ ] More locale packs — German, French, and Spanish are the obvious next additions since they cover a large chunk of real-world users. Native speaker review of plural rules is important before publishing these.
- [ ] Locale system: make plural rules easier to reason about and extend — the current `plural_form_int` helpers in Russian and Polish are readable but each locale reimplements the same pattern. A shared helper in `common` would make adding new locales safer.
- [ ] Locale system: allow overriding the `"in"` word cleanly for future-time formatting alongside the existing `ago_word` hook.
- [ ] Add golden output test files per locale — a set of fixed inputs and expected outputs stored in test data files so that locale changes never silently break grammar without a test failure.
- [ ] Cookbook-style documentation on docs.rs — short, focused examples for common scenarios: "how do I format bytes", "how do I add my own locale", "what happens with zero / negative / very large values". No walls of text, just code and a one-line explanation.
- [ ] Edge-case behavior tables in the docs — a quick reference showing what each formatter does with `0`, `i128::MIN`, `u128::MAX`, `f64::NAN`, `f64::INFINITY`, and `Duration::MAX`. Started for `number`, needs to be done for `bytes`, `duration`, `ago`, `percent`.
- [ ] More real-world examples — CLI progress output, log lines, dashboard numbers. Helps new users immediately see where the crate fits.
- [ ] Stable public API snapshot before 1.0 — lock down the formatter surface so downstream crates can rely on it without surprises across minor versions.
- [ ] Final API consistency pass before 1.0 — make sure all formatters follow the same naming patterns and builder method conventions with no small inconsistencies left.
- [ ] Add more comparison crates to the benchmark harness — `readable`, `human-readable`, `fancy-duration`, and `duration-human` are missing and cover overlapping functionality. Honest comparison means including them.
- [ ] Improve benchmark alignment — add explicit scenarios that match common real-world output styles (e.g. binary + `precision(2)` + `space(true)`) so the capability matrix stays fair and easy to interpret.
- [ ] `no_std + alloc` tier — right now it is either full `std` or truly bare `no_std`. An `alloc`-gated tier would let embedded targets with a heap use `.to_string()` without pulling in all of `std`. More useful than it sounds for embedded targets.
- [ ] f64 precision loss in `compact_suffix_for` for values above 2^53 — document as a known limitation. The `as_f64()` conversion on `DecimalParts` loses integer precision for very large magnitudes. For display purposes this is rarely visible, but worth a note in docs.
- [ ] Document the Russian ordinal gender limitation — `ordinal_suffix` for Russian always returns `-й` (masculine). The library has no concept of grammatical gender since it only receives a number. This should be explicitly noted in the Russian locale docs and in the edge-case behavior tables.
- [ ] Investigate `is_comma_style_separator` Unicode edge cases — the current check finds the first non-whitespace character and compares it to `,`. This works for ASCII separators but will not recognize `،` (U+060C Arabic comma) or `、` (U+3001 Ideographic comma). Low priority until non-Latin list separators are needed.
- [ ] Percentage formatter: consider accepting integer inputs (e.g. `42_u8` meaning `42%` directly, without the `* 100` ratio convention). Could be a separate `percent_ratio` vs `percent_value` split, or an option flag. Decide before 1.0.

---

## MAYBE SOMEDAY (no promises)

- [ ] Optional ICU4X integration — high-fidelity locale behavior (pluralization, list formatting, relative time) backed by Unicode CLDR data. This would be a heavy optional feature, definitely not the default.
- [ ] `serde` feature — serialize and deserialize options structs, useful for config-file driven formatting. Not needed until someone asks for it.
- [ ] `num-bigint` integration — compact formatting of arbitrary-precision integers. Very niche.
- [ ] Human-readable ranges — `"1–5 MB"`, `"~3 hours"` for UI-style approximate output.
- [ ] Temperature formatter — `36.6 -> "36.6°C"` / `"97.9°F"`, with configurable unit and locale-aware decimal separator. Low priority, but fits the theme.
- [ ] Scientific notation formatter — `1.23e9`, with locale-aware decimal separator and optional compact form.
- [ ] Grammar-aware unit forms — case and gender agreement for languages that need it (e.g. Russian genitive after 2–4). The current approach covers most cases but is not linguistically complete.
- [ ] WASM and embedded target smoke tests in CI.
- [ ] Fuzzing harness for the formatting paths — finding edge cases in the integer math and float rendering code.

---

## DONE

### (Unreleased → 0.5.0)
- [x] ~~Byte formatter: unit forcing and min/max clamping.~~
- [x] ~~Significant-digits mode — instead of decimal places, round to N total significant digits.~~
- [x] ~~Rounding mode control (HalfUp, Floor, Ceil)~~
- [x] ~~Fraction-aware pluralization: confirmed Polish and Russian boundaries work perfectly with existing tests~~
- [x] ~~Number formatter: always-on grouping separators option works seamlessly with `compact(false)`~~
- [x] ~~Number formatter: fully disable compact scaling cleanly via `NumberOptions::compact(bool)`~~
- [x] ~~Fix `report.rs`: `io::Error::other`, hardcoded hex colors in raw strings~~
- [x] ~~Expand `compare_numbers` benchmark: allocating_int, allocating_float, reused_buffer, locale overhead groups~~
- [x] ~~Expand `number` tests: small types, usize/isize, f32, very small floats, fixed_precision+long_units, separators+negatives, precision clamping, Russian/Polish inflection, space group separator~~
- [x] ~~Add edge case table to `number` module docs~~
- [x] ~~Add behaviour tables to `NumberOptions` rustdoc~~
- [x] ~~Remove dead code from `common/fmt.rs`: `truncate`, `ends_with_byte`, `find_byte`, `trim_ascii_trailing_zeros_and_dot`~~
- [x] ~~Refactor `number/format.rs`: remove dead fallback path, rename internal functions, extract `write_int_frac` helper, add `debug_assert` and `expect`~~
- [x] ~~Percentage formatter — `0.423 → "42.3%"`, locale-aware decimal separator, configurable precision, fixed_precision, f32/f64 input, Humanize trait methods~~
- [x] ~~Fix `format_float` fallback path: locale decimal separator was ignored when `StackString` overflowed~~
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

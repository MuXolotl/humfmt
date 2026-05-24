# TODO

A running list of planned features, known issues, and nice-to-haves.

Contributions are welcome — if you want to work on something, open an issue or
a draft PR first so we do not step on each other's toes.

---

| Priority | Formatter | Progress (Est.) |
|:---:|---|:---:|
| 1 | number | ~95% |
| 2 | bytes | ~95% |
| 3 | duration + ago | ~75% |
| 4 | list | ~95% |
| 5 | percentage | ~95% |
| 6 | ordinal | ~95% |
| ... | other | 0% |

---

## PLANNED

- [ ] Fuzz testing for all formatters
- [ ] Website with extensive interactive examples (https://muxolotl.github.io/humfmt).
- [ ] Add future-time support to `ago` — currently it only formats past durations. Should support `"in 5 minutes"` for future timestamps alongside the existing `"5 minutes ago"` style.
- [ ] Add `"just now"` / `"now"` / `"moments ago"` special cases to `ago` — for very small durations (e.g. under a configurable threshold like 5 seconds) it looks odd to print `"0s ago"` when the user probably wants `"just now"`.
- [ ] Rate / throughput formatter — `1_200_000 -> "1.2 MB/s"`, `42_000 -> "42K ops/s"`. Should reuse existing byte and number formatting logic rather than duplicating it.
- [ ] Ratio formatter — `0.75 -> "3:4"` or `"75%"` depending on options.
- [ ] Duration formatting: configurable join string between units — let the caller choose between `"1h 2m"` (space-joined, current default), `"1h, 2m"` (comma-joined), or `"1 hour and 2 minutes"` (and-joined in long mode).
- [ ] List formatter: `"or"` conjunction style — `"red, green, or blue"` alongside the existing `"and"` style. (Already trivially possible via `.conjunction("or")`; the question is whether to add a convenience `.disjunction()` method.)
- [ ] Cookbook-style documentation on docs.rs — short, focused examples for common scenarios. No walls of text, just code and a one-line explanation.
- [ ] More real-world examples — CLI progress output, log lines, dashboard numbers. Helps new users immediately see where the crate fits.
- [ ] Stable public API snapshot before 1.0 — lock down the formatter surface so downstream crates can rely on it without surprises across minor versions.
- [ ] Final API consistency pass before 1.0 — make sure all formatters follow the same naming patterns and builder method conventions with no small inconsistencies left.
- [ ] Add more comparison crates to the benchmark harness — `readable`, `human-readable`, `fancy-duration`, `duration-human`, `pretty-num`, `format_num` are missing and cover overlapping functionality.
- [ ] Improve benchmark alignment — add explicit scenarios that match common real-world output styles (e.g. binary + `precision(2)` + `space(true)`) so the capability matrix stays fair and easy to interpret.
- [ ] `no_std + alloc` tier — right now it is either full `std` or truly bare `no_std`. An `alloc`-gated tier would let embedded targets with a heap use `.to_string()` without pulling in all of `std`.
- [ ] f64 precision loss for compact scaling above 2^53 — document as a known limitation. The integer-to-f64 conversion loses precision for very large magnitudes. For display purposes this is rarely visible.
- [ ] Investigate `is_comma_style_separator` Unicode edge cases — currently only ASCII `,` is recognized. Could extend to `،` (U+060C Arabic comma) or `、` (U+3001 Ideographic comma) if a real use case appears.
- [ ] Percentage formatter: consider accepting integer inputs (e.g. `42_u8` meaning `42%` directly, without the `* 100` ratio convention). Could be a separate `percent_value` function or an option flag.
- [ ] Edge-case behaviour tables in the docs — a quick reference showing what each formatter does with `0`, `i128::MIN`, `u128::MAX`, `f64::NAN`, `f64::INFINITY`, and `Duration::MAX`. Started for `number` and `bytes`, needs to be completed for `duration`, `ago`, and refined for `percent` and `list`.
- [ ] Golden output test files — a set of fixed inputs and expected outputs stored as test data so that formatting changes never silently regress without a test failure. Useful as a regression net before 1.0.

---

## MAYBE SOMEDAY (no promises)

- [ ] `serde` feature — serialize and deserialize options structs, useful for config-file driven formatting. Not needed until someone asks for it.
- [ ] Optional `ryu` integration for the float number path — `ryu` is typically 3-5x faster than the standard library float formatter and already implements zero-alloc via `Buffer`. Would be opt-in via a `ryu-float` feature.
- [ ] `width()` / padding support — leverage `f.width()` from `Formatter` so that `format!("{:>10}", humfmt::number(x))` works for table-style output.
- [ ] `#[derive(Humanize)]` proc macro — let users derive convenience methods on their domain types (e.g. `MyMetric(1500).human_number()`).
- [ ] Allocation-tracking benchmarks (via `dhat-rs` or `cap`) — quantify the "zero-alloc" claim with numbers, not just words.
- [ ] Binary size and compile time benchmarks — `cargo bloat --crates`, `cargo build --timings`. humfmt's dependency-free design should shine here.
- [ ] `num-bigint` integration — compact formatting of arbitrary-precision integers. Very niche.
- [ ] Human-readable ranges — `"1–5 MB"`, `"~3 hours"` for UI-style approximate output.
- [ ] Temperature formatter — `36.6 -> "36.6°C"` / `"97.9°F"`, with configurable unit. Low priority, but fits the theme.
- [ ] Scientific notation formatter — `1.23e9`, with optional compact form.
- [ ] WASM and embedded target smoke tests in CI.
- [ ] Fuzzing harness for the formatting paths — finding edge cases in the integer math and float rendering code.

---

## DONE

### (Unreleased → 0.7.0)
...

### v0.6.0 released
- [x] ~~Update `BENCHMARKS.md` capability matrix (drop locale rows, add separator row, add `numfmt` column).~~
- [x] ~~Add `numfmt` to the comparison benchmark harness.~~
- [x] ~~Document the `Humanize` blanket impl pattern explicitly.~~
- [x] ~~Add `humfmt::ordinal::ordinal_suffix(u128) -> &'static str` public helper.~~
- [x] ~~Shrink float-path `StackString` from 512 to 384 bytes.~~
- [x] ~~Make all builder methods `const fn`.~~
- [x] ~~Replace `CustomLocale::list_separator` with `ListOptions::separator(&'static str)`.~~
- [x] ~~Replace locale-driven separator configuration with direct `decimal_separator(char)` / `group_separator(char)` builder methods on `NumberOptions`, `PercentOptions`, `BytesOptions`.~~
- [x] ~~Remove the entire i18n / locale subsystem (`Locale` trait, `English` / `Russian` / `Polish` types, `CustomLocale`, `DurationUnit`, all `*Options::locale(...)` methods, `ordinal_with`, `human_ordinal_with`, `russian` / `polish` / `english` / `alloc` feature flags).~~

### v0.5.0 released
- [x] ~~Number & Percent formatters:~~ `+` ~~sign option for positive values —~~ `"+1.5K"`~~,~~ `"+42.3%"` ~~style useful for delta/change displays.~~
- [x] ~~Byte formatter: bits mode (Kb, Mb, Gb) for bandwidth and throughput display.~~
- [x] ~~Byte formatter: support RoundingMode and significant_digits for API parity.~~
- [x] ~~Byte formatter: unit forcing and min/max clamping.~~
- [x] ~~Significant-digits mode — instead of decimal places, round to N total significant digits.~~
- [x] ~~Rounding mode control (HalfUp, Floor, Ceil)~~
- [x] ~~Number formatter: always-on grouping separators option works seamlessly with `compact(false)`~~
- [x] ~~Number formatter: fully disable compact scaling cleanly via `NumberOptions::compact(bool)`~~
- [x] ~~Percentage formatter — `0.423 → "42.3%"`, configurable precision, fixed_precision, f32/f64 input, Humanize trait methods~~

### v0.4.0 released
- [x] ~~MSRV raised to 1.70 and enforced by a dedicated CI job~~
- [x] ~~`NumberOptions::fixed_precision(bool)` — opt-in trailing-zero preservation~~
- [x] ~~`BytesOptions::fixed_precision(bool)` — opt-in trailing-zero preservation~~
- [x] ~~Float compact-number scaling rewritten to O(1) via IEEE 754 exponent — consistent with integer `ilog10` path~~
- [x] ~~Comparison harness covers `humansize` baseline (SI + aligned IEC + signed)~~
- [x] ~~Comparison harness covers `human-repr` with output examples in `BENCHMARKS.md`~~
- [x] ~~Comparison harness covers `indicatif::HumanBytes` and aligned byte groups~~
- [x] ~~Optional spacing in short byte output via `BytesOptions::space(bool)`~~
- [x] ~~Float compact-number formatting stable on `no_std` MSRV~~
- [x] ~~Shrink `StackString<512>` to `StackString<64>`~~
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
- [x] ~~Natural-language list formatter~~
- [x] ~~Relative-time ("ago") formatter~~
- [x] ~~Duration formatter with configurable `max_units`~~
- [x] ~~Ordinal formatter~~
- [x] ~~Byte-size formatter (decimal SI + binary IEC)~~

### v0.1.x released
- [x] ~~`no_std` compatible build~~
- [x] ~~Compact number formatter with short and long units~~

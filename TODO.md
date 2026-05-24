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

- [ ] Separate `ago` options from duration configuration.
- [ ] Add fuzz testing for all formatters.
- [ ] Build a website with extensive interactive examples ([https://muxolotl.github.io/humfmt](https://muxolotl.github.io/humfmt)).
- [ ] Add future-time support to `ago` — it currently formats only past durations and should also support `"in 5 minutes"` style output for future timestamps alongside the existing `"5 minutes ago"` style.
- [ ] Add `"just now"` / `"now"` / `"moments ago"` special cases to `ago` — for very small durations (for example, under a configurable threshold like 5 seconds), `"0s ago"` looks awkward when the user probably wants `"just now"` instead.
- [ ] Add a rate / throughput formatter — `1_200_000 -> "1.2 MB/s"`, `42_000 -> "42K ops/s"`. This should reuse the existing byte and number formatting logic instead of duplicating it.
- [ ] Add a ratio formatter — `0.75 -> "3:4"` or `"75%"` depending on options.
- [ ] Add configurable join strings for duration formatting — let the caller choose between `"1h 2m"` (space-joined, current default), `"1h, 2m"` (comma-joined), or `"1 hour and 2 minutes"` (and-joined in long mode).
- [ ] Consider an `"or"` conjunction style for the list formatter — `"red, green, or blue"` alongside the existing `"and"` style. This is already trivially possible via `.conjunction("or")`; the question is whether it deserves a convenience `.disjunction()` method.
- [ ] Add cookbook-style documentation on [docs.rs](http://docs.rs) — short, focused examples for common scenarios, with no walls of text.
- [ ] Add more real-world examples — CLI progress output, log lines, and dashboard numbers to help new users immediately see where the crate fits.
- [ ] Create a stable public API snapshot before 1.0 — lock down the formatter surface so downstream crates can rely on it without surprises across minor versions.
- [ ] Do a final formatter API consistency pass before 1.0 — audit `number`, `bytes`, `percent`, `duration`, `ago`, `ordinal`, and `list`, make their public API shapes as consistent as practical `*_with(...)`, dedicated `Options`, `Display`, sealed `*Like`, `Humanize` where appropriate), and keep only deliberate exceptions instead of accidental inconsistencies.
- [ ] Add more comparison crates to the benchmark harness — `readable`, `human-readable`, `fancy-duration`, `duration-human`, `pretty-num`, and `format_num` are still missing and cover overlapping functionality.
- [ ] Improve benchmark alignment — add explicit scenarios that match common real-world output styles (for example, binary + `precision(2)` + `space(true)`) so the capability matrix stays fair and easy to interpret.
- [ ] Add a `no_std + alloc` tier — right now the crate is either full `std` or truly bare `no_std`. An `alloc`-gated tier would let embedded targets with a heap use `.to_string()` without pulling in all of `std`.
- [ ] Document `f64` precision loss for compact scaling above `2^53` — integer-to-`f64` conversion loses precision for very large magnitudes, although this is rarely visible for display purposes.
- [ ] Investigate Unicode edge cases in `is_comma_style_separator` — it currently recognizes only ASCII `,`. It could potentially be extended to `،` (U+060C Arabic comma) or `、` (U+3001 Ideographic comma) if a real use case appears.
- [ ] Consider accepting integer inputs in the percentage formatter — for example, `42_u8` meaning `42%` directly instead of following the current `* 100` ratio convention. This could be a separate `percent_value` function or an option flag.
- [ ] Finish the edge-case behaviour tables in the docs — provide a quick reference showing what each formatter does with `0`, `i128::MIN`, `u128::MAX`, `f64::NAN`, `f64::INFINITY`, and `Duration::MAX`. This has started for `number` and `bytes`, but still needs to be completed for `duration` and `ago`, and refined for `percent` and `list`.
- [ ] Add golden output test files — store fixed inputs and expected outputs as test data so formatting changes cannot silently regress without a test failure. This should act as a regression net before 1.0.

---

## MAYBE SOMEDAY (no promises)

- [ ] Add a `serde` feature — serialize and deserialize options structs for config-file-driven formatting. This is not needed until someone actually asks for it.
- [ ] Add optional `ryu` integration for the float number path — `ryu` is typically 3–5x faster than the standard library float formatter and already implements zero-allocation formatting via `Buffer`. This would be opt-in behind a `ryu-float` feature.
- [ ] Add `width()` / padding support — leverage `f.width()` from `Formatter` so that `format!("{:>10}", humfmt::number(x))` works for table-style output.
- [ ] Add a `#[derive(Humanize)]` proc macro — let users derive convenience methods on their domain types (for example, `MyMetric(1500).human_number()`).
- [ ] Add allocation-tracking benchmarks (for example, via `dhat-rs` or `cap`) — quantify the "zero-alloc" claim with numbers instead of words.
- [ ] Add binary size and compile time benchmarks — `cargo bloat --crates`, `cargo build --timings`. The crate's dependency-free design should shine here.
- [ ] Add `num-bigint` integration — compact formatting for arbitrary-precision integers. This is very niche.
- [ ] Add human-readable ranges — `"1–5 MB"`, `"~3 hours"` for UI-style approximate output.
- [ ] Add a temperature formatter — `36.6 -> "36.6°C"` / `"97.9°F"`, with configurable units. Low priority, but it fits the theme.
- [ ] Add a scientific notation formatter — `1.23e9`, with optional compact output.
- [ ] Add WASM and embedded target smoke tests in CI.
- [ ] Add a dedicated fuzzing harness for the formatting paths — specifically to find edge cases in the integer math and float rendering code.

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
- [x] ~~Number & Percent formatters: `+` sign option for positive values — `"+1.5K"`, `"+42.3%"` style useful for delta/change displays.~~
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

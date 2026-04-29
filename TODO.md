# TODO

A running list of planned features, known issues, and nice-to-haves.

Contributions are welcome — if you want to work on something, open an issue or a draft PR first so we do not step on each other's toes.

---

## PLANNED

- [ ] Fix `is_integer` in `russian.rs` and `polish.rs` — the current `value == (value as u128) as f64` check gives wrong results for negative floats because the cast saturates to zero. Use `value % 1.0 == 0.0` instead, which works on stable `no_std`.
- [ ] Extract the shared `is_integer` helper out of `russian.rs` and `polish.rs` into `common` — it is the same function copy-pasted in two places.
- [ ] Remove the duplicated English duration unit logic from `Locale::duration_unit` default impl in `traits.rs` — it is a word-for-word copy of `english::duration_unit`. The default should just call `english::duration_unit` to avoid two places to update.
- [ ] Expand `max_units` clamp from `1..=4` to `1..=7` so callers can render all seven duration units when they explicitly ask for it.
- [ ] Rename or remove the `_value()` suffix on `pub(crate)` getter methods across `BytesOptions`, `NumberOptions`, `DurationOptions`, and `ListOptions` — the suffix adds noise without benefit, just access the fields or drop the suffix.
- [ ] Add a percentage formatter — `0.423 -> "42.3%"`, `1.0 -> "100%"`, with locale-aware decimal separators and a configurable number of decimal places. Should reuse the existing number formatting infrastructure rather than being its own thing.
- [ ] Add future-time support to `ago` — right now it only formats past durations. Should support `"in 5 minutes"` for future timestamps alongside the existing `"5 minutes ago"` style, with a clean locale hook for the "in" word.
- [ ] Add `"just now"` / `"now"` / `"moments ago"` special cases to `ago` — for very small durations (e.g. under a configurable threshold like 5 seconds) it looks odd to print `"0s ago"` when the user probably wants `"just now"`.
- [ ] Fixed-precision mode — when the caller sets `precision(2)`, optionally preserve trailing zeros so `1.50 KiB` stays `1.50 KiB` instead of being trimmed to `1.5 KiB`. Keep it opt-in so default behavior does not change.
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
- [ ] Explicit MSRV CI job — add a CI step that runs `cargo check` and `cargo test` against exactly Rust 1.67, not just stable, so MSRV regressions are caught automatically.
- [ ] Stable public API snapshot before 1.0 — lock down the formatter surface so downstream crates can rely on it without surprises across minor versions.
- [ ] Final API consistency pass before 1.0 — make sure all formatters follow the same naming patterns and builder method conventions with no small inconsistencies left.
- [ ] Add more comparison crates to the benchmark harness — `readable`, `human-readable`, `fancy-duration`, and `duration-human` are missing and cover overlapping functionality. Honest comparison means including them.
- [ ] Improve benchmark alignment — add explicit scenarios that match common real-world output styles (e.g. binary + `precision(2)` + `space(true)`) so the capability matrix stays fair and easy to interpret.

---

## MAYBE SOMEDAY (no promises)

- [ ] Optional ICU4X integration — high-fidelity locale behavior (pluralization, list formatting, relative time) backed by Unicode CLDR data. This would be a heavy optional feature, definitely not the default.
- [ ] `serde` feature — serialize and deserialize options structs, useful for config-file driven formatting. Not needed until someone asks for it.
- [ ] `num-bigint` integration — compact formatting of arbitrary-precision integers. Very niche.
- [ ] Smart adaptive formatter — `humfmt::auto(value)` that picks the best representation automatically based on the input type and magnitude.
- [ ] Human-readable ranges — `"1–5 MB"`, `"~3 hours"` for UI-style approximate output.
- [ ] Currency formatter — lightweight, definitely not trying to compete with full i18n libraries. Only makes sense if locale support grows significantly first.
- [ ] Scientific notation formatter — `1.23e9`, with locale-aware decimal separator and optional compact form.
- [ ] Word-order templates for locales — so languages like German and Polish can reorder phrases naturally instead of always appending the unit after the number. Complex to design well.
- [ ] Grammar-aware unit forms — case and gender agreement for languages that need it (e.g. Russian genitive after 2–4). The current approach covers most cases but is not linguistically complete.
- [ ] WASM and embedded target smoke tests in CI.
- [ ] Fuzzing harness for the formatting paths — finding edge cases in the integer math and float rendering code.
- [ ] `no_std` + `alloc` (without `std`) tier — right now it is either full `std` or truly bare `no_std`. An `alloc`-gated tier would let embedded targets with a heap use `to_string()` without pulling in all of `std`.

---

## DONE

- [x] ~~Comparison harness covers `humansize` baseline (SI + aligned IEC + signed)~~ (Unreleased)
- [x] ~~Comparison harness covers `human-repr` with output examples in `BENCHMARKS.md`~~ (Unreleased)
- [x] ~~Comparison harness covers `indicatif::HumanBytes` and aligned byte groups~~ (Unreleased)
- [x] ~~Optional spacing in short byte output via `BytesOptions::space(bool)`~~ (Unreleased)
- [x] ~~Byte formatter locale-aware decimal separator~~ (Unreleased)
- [x] ~~Float compact-number formatting stable on `no_std` MSRV~~ (Unreleased)
- [x] ~~Polish plural rules CLDR-aligned for long-form output~~ (Unreleased)
- [x] ~~Shrink `StackString<512>` to `StackString<64>`~~ (Unreleased)
- [x] ~~`ListOptions::serial_comma_enabled(bool)` and `ListOptions::conjunction`~~ (Unreleased)
- [x] ~~`locale::CustomLocale` list separator hook (`list_separator`)~~ (Unreleased)
- [x] ~~On-demand GitHub Actions benchmark workflow~~ (Unreleased)
- [x] ~~CI: test suite + clippy + fmt + feature matrix + `no_std` check~~ (Unreleased)
- [x] ~~docs.rs all-features build~~ (0.3.0)
- [x] ~~`#![deny(missing_docs)]` + full public API rustdoc coverage~~ (0.3.0)
- [x] ~~Centralized `Sealed` trait infrastructure~~ (0.3.0)
- [x] ~~O(1) compact integer scaling via `ilog10` / `ilog2`~~ (0.3.0)
- [x] ~~Property tests with `proptest` (sign symmetry, monotonicity, locale invariants)~~ (0.3.0)
- [x] ~~Capability matrix in `BENCHMARKS.md`~~ (0.3.0)
- [x] ~~Auto-generated `BENCHMARKS.md` + dark-theme SVG charts~~ (0.3.0)
- [x] ~~Standalone comparison benchmark harness (`tools/benchmarks/`)~~ (0.3.0)
- [x] ~~Criterion benchmark suite for the core formatter paths~~ (0.3.0)
- [x] ~~`DurationConversionError` + `*_checked` helpers~~ (0.3.0)
- [x] ~~Optional `time` integration (`Duration`, `OffsetDateTime`)~~ (0.2.0)
- [x] ~~Optional `chrono` integration (`TimeDelta`, `DateTime`)~~ (0.2.0)
- [x] ~~`CustomLocale` builder for ad hoc suffix / separator / hook overrides~~ (0.2.0)
- [x] ~~English, Russian, and Polish locale packs~~ (0.2.0)
- [x] ~~Natural-language list formatter~~ (0.2.0)
- [x] ~~Relative-time ("ago") formatter~~ (0.2.0)
- [x] ~~Duration formatter with configurable `max_units`~~ (0.2.0)
- [x] ~~Ordinal formatter~~ (0.2.0)
- [x] ~~Byte-size formatter (decimal SI + binary IEC)~~ (0.2.0)
- [x] ~~`no_std` compatible build~~ (0.1.1)
- [x] ~~Compact number formatter with short and long units~~ (0.1.0)

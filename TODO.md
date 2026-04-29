# TODO

A running list of things to do, things being worked on, and things that are done. Feel free to pick something up — contributions are welcome!

If you want to work on something, open an issue or a draft PR first so we don't step on each other's toes.

---

## PLANNED

- [ ] Percentage formatter — `0.423 -> "42.3%"`, with locale-aware separators
- [ ] Rate / throughput formatter — `1_200_000 -> "1.2 MB/s"`, `42_000 -> "42K ops/s"`
- [ ] Ratio formatter — `0.75 -> "3:4"` or `"75%"` depending on options
- [ ] Temperature formatter — `36.6 -> "36.6°C"` (low priority, but fits the theme)
- [ ] Expand `max_units` clamp from `1..=4` to `1..=7` so all duration units can be rendered when the caller explicitly asks for it
- [ ] Extract the shared `is_integer` helper out of `russian.rs` / `polish.rs` into `common` — it's the same function copy-pasted in two places right now
- [ ] Fix `is_integer` in `number/format.rs` to use `value.fract() == 0.0` instead of the `as u128` cast, which saturates for negative inputs
- [ ] Expand comparison benchmarks — add more crates and more realistic scenarios so the capability matrix stays honest and up to date
- [ ] More MSRV CI coverage — test against 1.67 explicitly, not just stable
- [ ] Stable public API snapshot — lock down the formatter surface before 1.0 so downstream users can depend on it without surprises
- [ ] Cookbook-style docs — short, practical guides on the docs.rs page: "how do I format bytes?", "how do I add a custom locale?", edge-case tables, that sort of thing. No walls of text.
- [ ] More locale packs — German, French, Spanish are the obvious next ones. Looking for native speakers to help get pluralization right.
- [ ] Fixed-precision mode (preserve trailing zeros) for bytes and numbers — e.g. `1.50 KiB` instead of `1.5 KiB` when `precision(2)` is set
- [ ] Benchmark alignment presets — add explicit comparison scenarios that match common output styles (e.g. binary + `precision(2)` + `space(true)`) to keep tables fair

---

## MAYBE SOMEDAY (no promises)

- [ ] Optional ICU4X integration — for high-fidelity locale behavior (pluralization, list formatting, relative time) backed by Unicode CLDR data. This would be a heavy optional feature, not the default.
- [ ] `serde` feature — serialize / deserialize options structs (useful for config files)
- [ ] `num-bigint` integration — for compact formatting of arbitrary-precision integers
- [ ] WASM / embedded target smoke tests in CI
- [ ] Fuzzing harness for the formatting paths

---

## DONE

- [x] ~~Compact number formatter with short and long units~~
- [x] ~~Byte-size formatter (decimal SI + binary IEC)~~
- [x] ~~Ordinal formatter~~
- [x] ~~Duration formatter with configurable `max_units`~~
- [x] ~~Relative-time ("ago") formatter~~
- [x] ~~Natural-language list formatter~~
- [x] ~~English, Russian, and Polish locale packs~~
- [x] ~~`CustomLocale` builder for ad hoc suffix / separator / hook overrides~~
- [x] ~~Optional `chrono` integration (`TimeDelta`, `DateTime`)~~
- [x] ~~Optional `time` integration (`Duration`, `OffsetDateTime`)~~
- [x] ~~`DurationConversionError` + `*_checked` helpers~~
- [x] ~~`no_std` compatible build~~
- [x] ~~Criterion benchmark suite for the core formatter paths~~
- [x] ~~Standalone comparison benchmark harness (`tools/benchmarks/`)~~
- [x] ~~Auto-generated `BENCHMARKS.md` + dark-theme SVG charts~~
- [x] ~~Capability matrix in `BENCHMARKS.md`~~
- [x] ~~Property tests with `proptest` (sign symmetry, monotonicity, locale invariants)~~
- [x] ~~O(1) compact integer scaling via `ilog10` / `ilog2`~~
- [x] ~~Centralized `Sealed` trait infrastructure~~
- [x] ~~`#![deny(missing_docs)]` + full public API rustdoc coverage~~
- [x] ~~docs.rs all-features build~~
- [x] ~~CI: test suite + clippy + fmt + feature matrix + `no_std` check~~
- [x] ~~On-demand GitHub Actions benchmark workflow~~
- [x] ~~`locale::CustomLocale` list separator hook (`list_separator`)~~
- [x] ~~`ListOptions::serial_comma_enabled(bool)` and `ListOptions::conjunction`~~
- [x] Shrink `StackString<512>` to `StackString<64>` — 512 bytes on the stack for a float that will never exceed ~50 characters is overkill
- [x] Polish plural rules for long-form output are CLDR-aligned (compact-number long suffixes and duration units).
- [x] Float compact-number formatting remains stable `no_std` on MSRV by avoiding std-only or unstable core float math methods.
- [x] Byte formatter locale-awareness — the decimal separator in byte output can respect the active locale like numbers do.
- [x] Optional spacing in short byte output via `BytesOptions::space(bool)` (e.g. `1.5 KB`).
- [x] Comparison harness includes indicatif::HumanBytes and aligned byte benchmark groups (IEC + space).

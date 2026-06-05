# TODO

Planned work and known gaps. See [CHANGELOG.md](./CHANGELOG.md) for what's already done.

---

## Formatters

**number**
- *none*

**bytes**
- *none*

**percent**
- [ ] Integer input support (`42_u8` → `42%` directly, as an alternative to the current ratio convention)

**duration**
- [ ] Configurable unit join style — space (`"1h 2m"`, default), comma (`"1h, 2m"`), or `"and"` (`"1 hour and 2 minutes"`)

**ago**
- [ ] Split `AgoOptions` from `DurationOptions`
- [ ] Future-time support (`"in 5 minutes"` alongside existing `"5 minutes ago"`)
- [ ] `"just now"` threshold — configurable cutoff for very short durations instead of `"0s ago"`

**ordinal**
- *none*

**list**
- [ ] `.disjunction()` shorthand — `"red, green, or blue"` (already possible via `.conjunction("or")`, question is whether a named method is worth it)

**new**
- [ ] Rate / throughput formatter — `1_200_000 -> "1.2 MB/s"`, `42_000 -> "42K ops/s"`

---

## Docs

- [ ] Finish edge-case behaviour tables (`0`, `i128::MIN`, `u128::MAX`, `f64::NAN`, `f64::INFINITY`, `Duration::MAX`) for `duration`, `ago`, `percent`, `list`
- [ ] Cookbook-style examples on docs.rs — focused, no walls of text
- [ ] Real-world examples — CLI progress, log lines, dashboard output
- [ ] Document `f64` precision loss above `2^53` in compact scaling
- [ ] Investigate `is_comma_style_separator` for non-ASCII commas (`،` U+060C, `、` U+3001)
- [ ] API consistency audit before 1.0
- [ ] Public API freeze before 1.0

---

## Infrastructure

- [ ] `no_std + alloc` tier (currently: full `std` or bare `no_std`)
- [ ] `width()` / padding via `fmt::Formatter::width()`
- [ ] Binary size and compile-time benchmarks (`cargo bloat --crates`, `cargo build --timings`)

---

## Benchmarks

- [ ] Add missing crates: `readable`, `human-readable`, `fancy-duration`, `duration-human`, `pretty-num`, `format_num`
- [ ] Improve scenario alignment (e.g. IEC + `precision(2)` + `space(true)`)
- [ ] Allocation-tracking benchmarks (`dhat-rs` or `cap`)

---

## Maybe

- [ ] `ryu` feature flag for float path (`ryu::Buffer`, opt-in behind `ryu-float`)
- [ ] `serde` feature for options structs
- [ ] `#[derive(Humanize)]` proc-macro
- [ ] `num-bigint` integration for arbitrary-precision integers
- [ ] Human-readable ranges (`"1–5 MB"`, `"~3 hours"`)
- [ ] Temperature formatter (`36.6 -> "36.6°C"` / `"97.9°F"`, configurable units)
- [ ] Scientific notation formatter (`1.23e9`, optional compact output)
- [ ] Ratio formatter (`0.75 -> "3:4"` or `"75%"` depending on options)
- [ ] WASM / embedded smoke tests in CI
- [ ] Interactive examples website — <https://muxolotl.github.io/humfmt>

---

## Done (unreleased)

- [x] ~~`PercentOptions::rounding(RoundingMode)`~~
- [x] ~~Common option types re-exported from `humfmt::prelude`~~
- [x] ~~Fuzz targets for all formatters~~
- [x] ~~Golden snapshot tests~~

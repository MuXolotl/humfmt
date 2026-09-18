# TODO

Planned work and known gaps. See [CHANGELOG.md](./CHANGELOG.md) for what's already done.

---

## Formatters

**number**
- *none*

**bytes**
- Short decimal labels are `KB` / `MB` (JEDEC-style capitals), not SI `kB`. Documented as 1000-based. Do not silently switch to `kB` — that would be a breaking output change.

**percent**
- Integer input (`42_u8` → `42%`) is **not** planned. The ratio convention (`1.0` = `100%`) is the API. Mixing both would make `42` vs `0.42` a footgun. If a "already a percent" helper is ever needed, it should be a separately named function (`percent_points` / `from_percent`), not an overload.

**duration**
- [ ] Configurable unit join style — space (`"1h 2m"`, default), comma (`"1h, 2m"`), or `"and"` (`"1 hour and 2 minutes"`)

**ago**
- [ ] Split `AgoOptions` from `DurationOptions` (breaking, 0.7)
- [ ] Future-time support (`"in 5 minutes"` alongside existing `"5 minutes ago"`)
- [ ] `"just now"` threshold — configurable cutoff for very short durations instead of `"0s ago"`

**ordinal**
- *none*

**list**
- `.disjunction()` is YAGNI — `.conjunction("or")` already does `"red, green, or blue"`.

**new**
- Rate / throughput (`"1.2 MB/s"`) — only if a real consumer needs it. Do not add temperature / scientific notation / ranges just to fill a matrix.

---

## Docs

- [x] Edge-case tables for `duration` (`Duration::MAX`)
- [ ] Cookbook-style examples on docs.rs — focused, no walls of text
- [ ] Real-world examples — CLI progress, log lines, dashboard output
- [x] Document `f64` precision loss above `2^53` in compact scaling
- [ ] Investigate `is_comma_style_separator` for non-ASCII commas (`،` U+060C, `、` U+3001)
- [ ] API consistency audit before 1.0
- [ ] Public API freeze before 1.0

---

## Infrastructure

- `no_std + alloc` as a crate feature is unnecessary: formatters are `Display` and never heap-allocate. Users with `alloc` already get `.to_string()`.
- [x] `width()` / padding via `fmt::Formatter::pad()` (string-like default align)
- [ ] Binary size and compile-time measurements (`cargo bloat --crates`, `cargo build --timings`) — report in BENCHMARKS.md when measured, do not guess

---

## Benchmarks

Keep comparisons honest: same input type, same unit system, call setup outside the timed loop, and label semantic mismatches in the report. Do not present humfmt-only groups as competitor wins.

- [ ] Add crates only where output actually overlaps: `readable` / `pretty-num` (grouped digits, not compact `K/M`), `human-duration` / `fancy-duration` (duration semantics differ — align `max_units` or label the difference)
- [ ] Allocation-counting (custom allocator or `dhat`) on the reused-buffer path — expected 0 from humfmt itself
- [ ] Binary size of a hello-world using only `humfmt::bytes` vs `humansize` vs `bytesize`
- [ ] Criterion baselines in CI for regression tracking (store `target/criterion` baselines, not marketing numbers)

---

## Maybe

- [ ] `ryu` feature for the large-uncompacted float fallback only — default path must stay zero-deps
- [ ] `serde` for options structs — only if someone persists them
- [ ] Interactive examples website
- Do **not**: `#[derive(Humanize)]`, `num-bigint`, temperature, scientific notation, ratio `3:4`. Those turn the crate into a kitchen sink.

---

## Done (unreleased)

- [x] `PercentOptions::rounding(RoundingMode)`
- [x] Common option types re-exported from `humfmt::prelude`
- [x] Fuzz targets for all formatters
- [x] Golden snapshot tests
- [x] `Display` width / alignment / fill
- [x] Shared `Precision` + float rounding helper
- [x] Direct digit writing for integers, compact floats, duration, ordinal

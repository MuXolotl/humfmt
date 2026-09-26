# TODO

Planned work and known gaps. Completed items are removed from this file;
[CHANGELOG.md](./CHANGELOG.md) is the single place that lists what shipped.

---

## Formatters

**number**
- Decide `force_sign` for `-0.0` — currently `"0"`, not `"+0"`

**bytes**
- Short decimal labels are `KB` / `MB` (JEDEC-style capitals), not SI `kB`. Documented as 1000-based. Do not silently switch to `kB` — that would be a breaking output change.
- Drop the sign when a value rounds to zero — `bytes_with(-1i64, unit(KB))` gives `"-0KB"`
- Fix `significant_digits` below one unit — `bytes_with(12_345u64, unit(MB).significant_digits(3))` gives `"0.01MB"`, must be `"0.0123MB"`
- Decide `bits(true)` at `u128::MAX` — silent saturation or a fallible API
- Fix the `precision(2)` row for `999_950` in the `BytesOptions` table — says `"1MB"`, actual `"999.95KB"`

**percent**
- Integer input support (`42_u8` → `42%` directly, as an alternative to the current ratio convention)
- Decide the `percent` semantics — round the exact product (`0.15` → `14.9999…%`, `Floor` → `14%`) or the `f64` product (`15.0` → `15%`)

**duration**
- Configurable unit join style — space (`"1h 2m"`, default), comma (`"1h, 2m"`), or `"and"` (`"1 hour and 2 minutes"`)

**ago**
- Future-time support (`"in 5 minutes"` alongside existing `"5 minutes ago"`)

**ordinal**
- Document the negative-input rule and test `-11/-12/-13`, `-111`, `i128::MIN`

**list**
- Render once instead of twice when a width is requested

**new**
- Rate / throughput formatter — `1_200_000 -> "1.2 MB/s"`, `42_000 -> "42K ops/s"`

---

## Docs

- Document that a width specifier renders the value twice
- Fix the "every formatter has a `*_with` variant" claim — `ordinal` has none
- Edge-case behaviour table for `ago` (`0`, `1s`, `Duration::MAX`); `number`, `bytes`, `percent`, `duration` and `list` have one
- Decide the fate of the `humfmt::chrono` / `humfmt::time` `*_checked` functions: since the single error type landed they are name-only twins of `duration`, `ago`, and `ago_since`
- Cookbook-style examples on docs.rs — focused, no walls of text
- Real-world examples — CLI progress, log lines, dashboard output
- Investigate `is_comma_style_separator` for non-ASCII commas (`،` U+060C, `、` U+3001)
- API consistency audit before 1.0
- Public API freeze before 1.0

---

## Infrastructure

- Add a numeric oracle test — exact reference, assert full strings across precision, significant digits, rounding modes and unit options
- Add a differential test: `number_with(v, precision(p).compact(false))` must match `format!("{:.*}", p, v)` for finite `f64`
- Extend fuzz inputs to `u128` and fuzz the option set (`significant_digits`, `min_unit` / `max_unit` / `unit`, `bits`, `rounding`); run on push; assert values, not only absence of panics
- Extend the suffix property tests to `u128` inputs and add the missing `Ud`
- Verify the `time` pin `<0.3.42` on MSRV 1.70 — the MSRV job currently skips it
- Binary size and compile-time measurements (`cargo bloat --crates`, `cargo build --timings`)

---

## Benchmarks

- Add missing crates: `readable`, `human-readable`, `fancy-duration`, `duration-human`, `pretty-num`, `format_num`
- Allocation-counting benchmarks
- Binary size benchmarks
- Compile-time benchmarks
- Criterion baselines for regression tracking
- Add confidence intervals and mark overlapping comparisons

---

## Maybe

- `ryu` feature flag for float path (`ryu::Buffer`, opt-in behind `ryu-float`)
- `serde` feature for options structs
- `#[derive(Humanize)]` proc-macro
- `num-bigint` integration for arbitrary-precision integers
- Human-readable ranges (`"1–5 MB"`, `"~3 hours"`)
- Temperature formatter (`36.6 -> "36.6°C"` / `"97.9°F"`, configurable units)
- Scientific notation formatter (`1.23e9`, optional compact output)
- Ratio formatter (`0.75 -> "3:4"` or `"75%"` depending on options)
- WASM / embedded smoke tests in CI
- Interactive examples website — <https://muxolotl.github.io/humfmt>

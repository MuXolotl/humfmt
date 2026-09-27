# TODO

Planned work and known gaps. Completed items are removed from this file;
[CHANGELOG.md](./CHANGELOG.md) is the single place that lists what shipped.

---

## Formatters

**bytes**
- Short decimal labels are `KB` / `MB` (JEDEC-style capitals), not SI `kB`, and
  they measure 1000-based units. Switching to `kB` would change output, so it
  needs a major release.
- Raw bit counts above `2^125` saturate at `u128::MAX`. Every scaled unit is a
  multiple of eight and stays exact, but the saturation is silent.

**percent**
- Integer input support (`42_u8` -> `42%` directly, as an alternative to the
  ratio convention).

**duration**
- Configurable unit join style: space (`"1h 2m"`, default), comma (`"1h, 2m"`),
  or `"and"` (`"1 hour and 2 minutes"`).

**ago**
- Future-time support (`"in 5 minutes"` alongside the current `"5 minutes ago"`).

**ordinal**
- Test and document the negative-input rule: `-11`, `-12`, `-13`, `-111`,
  `i128::MIN`.

**list**
- Render once instead of twice when a width is requested.

**new**
- Rate and throughput formatter: `1_200_000 -> "1.2 MB/s"`, `42_000 -> "42K ops/s"`.

---

## Docs

- Edge-case behaviour table for `ago` (`0`, `1s`, `Duration::MAX`). `number`,
  `bytes`, `percent`, `duration` and `list` have one.
- Decide what happens to the `humfmt::chrono` / `humfmt::time` `*_checked`
  functions: since the single error type landed they are name-only twins of
  `duration`, `ago`, and `ago_since`.
- Cookbook-style examples on docs.rs, focused rather than exhaustive.
- Real-world examples: CLI progress, log lines, dashboard output.
- Investigate `is_comma_style_separator` for non-ASCII commas (`،` U+060C,
  `、` U+3001).
- API consistency audit and public API freeze before 1.0.

---

## Infrastructure

- Extend the differential and oracle coverage to `bytes` and the integer paths,
  which currently rest on the curated cases in `tests/` alone.
- Extend fuzz inputs to `u128` and fuzz the option set (`significant_digits`,
  `min_unit` / `max_unit` / `unit`, `bits`, `rounding`); run on push; assert
  formatted values, not only absence of panics.
- Extend the suffix property tests to `u128` inputs and add the missing `Ud`.
- Verify the `time` pin `<0.3.42` on MSRV 1.70; the MSRV job currently skips it.
- Binary size and compile-time measurements (`cargo bloat --crates`,
  `cargo build --timings`).

---

## Benchmarks

- Add missing crates: `readable`, `human-readable`, `fancy-duration`,
  `duration-human`, `pretty-num`, `format_num`.
- Allocation counting in the benchmark harness. A zero-allocation test now
  covers the formatters themselves.
- Binary size and compile-time benchmarks.
- Criterion baselines for regression tracking.
- Confidence intervals, with overlapping comparisons marked.

---

## Maybe

- `ryu` feature flag for the float path (`ryu::Buffer`, opt-in behind `ryu-float`)
- `serde` feature for the options structs
- `#[derive(Humanize)]` proc-macro
- `num-bigint` integration for arbitrary-precision integers
- Human-readable ranges (`"1-5 MB"`, `"~3 hours"`)
- Temperature formatter (`36.6 -> "36.6°C"` / `"97.9°F"`, configurable units)
- Scientific notation formatter (`1.23e9`, optional compact output)
- Ratio formatter (`0.75 -> "3:4"` or `"75%"` depending on options)
- WASM and embedded smoke tests in CI
- Interactive examples website: <https://muxolotl.github.io/humfmt>

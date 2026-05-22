# humfmt Benchmarks (Comparison Harness)

This directory contains a standalone benchmark harness used to compare `humfmt`
against other crates with overlapping functionality.

The harness is kept out of the main crate to avoid pulling extra dependencies
into `humfmt` itself.

## Running locally

From the repository root:

```bash
cargo bench --manifest-path tools/benchmarks/Cargo.toml
```

Then generate a repository-friendly summary and dark-theme charts:

```bash
cargo run --release --manifest-path tools/benchmarks/Cargo.toml --bin report
```

Outputs:

- `BENCHMARKS.md`
- `assets/benchmarks/*_dark.svg`

Criterion will also write HTML reports under:

- `tools/benchmarks/target/criterion/`

## Methodology

This harness measures two common usage patterns:

1. Allocating: `to_string()` / `format!(...)` returning an owned `String`
2. Reused buffer: writing `Display` output into a pre-allocated `String`

For honest comparisons, benchmark inputs are fixed and formatter setup is done
outside the timed loops.

## Notes on semantic alignment

Not all crates share the same output semantics (SI vs IEC, fixed decimals vs
trimmed decimals, spacing, compact suffix names, and input types).

To keep comparisons interpretable, the generated report separates benchmark
groups into two categories:

1. **Comparison groups** — scenarios where multiple crates can reasonably
   produce similar output for similar inputs.
2. **humfmt-only groups** — scenarios that measure humfmt feature cost or
   extended input ranges that the compared crates do not support.

Humfmt-only groups are intentionally not presented as competitor comparisons.
They exist to track regression behaviour and make feature costs visible.

## Bytes benchmarks

Byte benchmarks are split into:

- default-style groups close to humfmt defaults,
- aligned IEC + space + precision groups,
- extended `u128` range groups where only humfmt participates,
- negative signed-input groups where only crates with signed support participate.

The generated report includes output example tables so semantic differences are
visible instead of hidden behind timing numbers.

## Number benchmarks

Number benchmarks include:

- mixed `i64` compact-number comparisons,
- `u64` apples-to-apples compact-number comparisons,
- `f64` compact-number comparisons,
- reused-buffer comparisons,
- humfmt-only option-cost groups,
- humfmt-only `u128` extended-range groups.

The comparison groups include `human_format` and `numfmt` where their APIs and
output model overlap with humfmt. The humfmt-only groups cover features such as:

- significant digits,
- long-form suffix labels,
- uncompacted grouped output,
- custom decimal/group separators,
- forced signs,
- floor/ceil rounding modes,
- extended `u128` formatting,
- reused-buffer output for option-heavy scenarios.

## Duration and relative-time benchmarks

Duration and relative-time semantics vary significantly between crates. For
example, some crates render all non-zero units while humfmt caps output via
`DurationOptions::max_units`.

The generated report calls out these differences explicitly instead of treating
all duration strings as identical outputs.

## Partial runs

You can run a single benchmark file while iterating:

```bash
cargo bench --manifest-path tools/benchmarks/Cargo.toml --bench compare_numbers
```

The report generator reads whatever Criterion results exist on disk. If you only
run one benchmark file, unrelated sections in `BENCHMARKS.md` may show stale
values or `N/A`.

For a fresh full report, run:

```bash
cargo bench --manifest-path tools/benchmarks/Cargo.toml
cargo run --release --manifest-path tools/benchmarks/Cargo.toml --bin report
```

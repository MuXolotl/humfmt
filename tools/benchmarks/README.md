# humfmt Benchmarks (Comparison Harness)

This directory contains a standalone benchmark harness used to compare `humfmt`
against other popular crates with overlapping functionality.

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

This harness tries to measure two common usage patterns:

1) Allocating: `to_string()` / `format!(...)` returning an owned `String`
2) Reused buffer: writing `Display` output into a pre-allocated `String`

For honest comparisons, benchmark inputs are fixed and formatter setup is done
outside the timed loops.
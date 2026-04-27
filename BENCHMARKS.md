# Benchmarks

This report is generated from Criterion `median.point_estimate` values.

Regenerate locally:

```bash
cargo bench --manifest-path tools/benchmarks/Cargo.toml
cargo run --release --manifest-path tools/benchmarks/Cargo.toml --bin report
```

Notes:

- Results depend on machine / OS / CPU scaling.
- Some crates differ in formatting semantics (e.g. spaces between number and unit).

## Bytes (allocating, to_string)

| Implementation | Median per-iteration | time per value (approx) | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt | 1.87 µs | 234 ns | 1.00× |
| bytesize | 2.22 µs | 278 ns | 1.19× |
| byte-unit | 8.15 µs | 1.02 µs | 4.36× |
| prettier-bytes | 1.08 µs | 135 ns | 0.58× |

## Bytes (reused buffer, write! into String)

| Implementation | Median per-iteration | time per value (approx) | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt | 1.10 µs | 137 ns | 1.00× |
| bytesize | 1.45 µs | 181 ns | 1.32× |
| byte-unit | 6.97 µs | 871 ns | 6.35× |
| prettier-bytes | 374 ns | 47 ns | 0.34× |

## Numbers (allocating, to_string / format)

| Implementation | Median per-iteration | time per value (approx) | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt | 4.30 µs | 430 ns | 1.00× |
| human_format | 5.76 µs | 576 ns | 1.34× |


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
| bytesize | 2.25 µs | 282 ns | 1.21× |
| byte-unit | 7.94 µs | 992 ns | 4.24× |
| prettier-bytes | 1.10 µs | 137 ns | 0.59× |

## Bytes (reused buffer, write! into String)

| Implementation | Median per-iteration | time per value (approx) | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt | 1.10 µs | 137 ns | 1.00× |
| bytesize | 1.45 µs | 181 ns | 1.32× |
| byte-unit | 7.02 µs | 877 ns | 6.39× |
| prettier-bytes | 374 ns | 47 ns | 0.34× |

## Numbers (allocating, to_string / format)

| Implementation | Median per-iteration | time per value (approx) | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt | 4.32 µs | 432 ns | 1.00× |
| human_format | 5.78 µs | 578 ns | 1.34× |


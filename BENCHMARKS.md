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
| humfmt | 564 ns | 70 ns | 1.00× |
| bytesize | 1.02 µs | 128 ns | 1.81× |
| byte-unit | 4.52 µs | 565 ns | 8.01× |
| prettier-bytes | 315 ns | 39 ns | 0.56× |

## Bytes (reused buffer, write! into String)

| Implementation | Median per-iteration | time per value (approx) | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt | 461 ns | 58 ns | 1.00× |
| bytesize | 886 ns | 111 ns | 1.92× |
| byte-unit | 4.23 µs | 529 ns | 9.17× |
| prettier-bytes | 198 ns | 25 ns | 0.43× |

## Numbers (allocating, to_string / format)

| Implementation | Median per-iteration | time per value (approx) | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt | 2.16 µs | 216 ns | 1.00× |
| human_format | 2.31 µs | 231 ns | 1.07× |


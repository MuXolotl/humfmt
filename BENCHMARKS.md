# Benchmarks

This report is generated from Criterion `median.point_estimate` values.

Regenerate locally:

```bash
cargo bench --manifest-path tools/benchmarks/Cargo.toml
cargo run --release --manifest-path tools/benchmarks/Cargo.toml --bin report
```

---

## Capability Matrix

| Feature | humfmt | bytesize | byte-unit | prettier-bytes | humantime | timeago | human_format |
|---|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
| Byte sizes | yes | yes | yes | yes | no | no | no |
| Compact numbers | yes | no | no | no | no | no | yes |
| Duration formatting | yes | no | no | no | yes | yes | no |
| Relative time (ago) | yes | no | no | no | no | yes | no |
| Ordinals | yes | no | no | no | no | no | no |
| List formatting | yes | no | no | no | no | no | no |
| Signed input (negatives) | yes | no | no | no | — | — | no |
| u128 / i128 range | yes | no | partial | no | — | — | no |
| Float input | yes | no | no | no | — | — | yes |
| Long-form labels | yes | no | yes | no | yes | yes | yes |
| Max-units cap | yes | — | — | — | no | yes | — |
| Binary (IEC) units | yes | yes | yes | yes | — | — | — |
| Configurable precision | yes | no | yes | no | — | — | yes |
| Locale-aware | yes | no | no | no | no | yes | no |
| Custom locale builder | yes | no | no | no | no | no | no |
| no_std compatible | yes | no | no | yes | no | no | no |
| Zero-alloc Display | yes | yes | no | yes | yes | no | no |

---

## Notes

- Results depend on machine / OS / CPU scaling.
- **Bold** = best (lowest) value in column.
- Limitation tags are shown next to each crate name.
- Rows are sorted fastest to slowest within each group.
- Duration semantics can differ between crates (e.g. full-unit rendering vs capped output).
- Some crates return an owned `String` by design; `humfmt` formatters implement `Display`.

---

## Bytes — allocating (`to_string`), u64 inputs

> prettier-bytes and bytesize are **u64-only**. humfmt accepts i8-i128 and u8-u128.

| Implementation | Median per-iteration | time per value | Relative vs humfmt |
|---|---:|---:|---:|
| prettier-bytes  u64 only, fixed 2dp, no negatives | **357 ns** | **45 ns** | 0.59x |
| humfmt  i8-u128, any precision | 605 ns | 76 ns | 1.00x |
| bytesize  u64 only | 1.02 us | 127 ns | 1.68x |
| byte-unit  u64/u128 | 4.80 us | 600 ns | 7.94x |

## Bytes — reused buffer (`write!` into `String`), u64 inputs

| Implementation | Median per-iteration | time per value | Relative vs humfmt |
|---|---:|---:|---:|
| prettier-bytes  u64 only, fixed 2dp, no negatives | **186 ns** | **23 ns** | 0.37x |
| humfmt  i8-u128, any precision | 503 ns | 63 ns | 1.00x |
| bytesize  u64 only | 918 ns | 115 ns | 1.83x |
| byte-unit  u64/u128 | 4.61 us | 576 ns | 9.16x |

## Bytes — extended range (u128 > u64::MAX) — humfmt only

> No other benchmarked crate handles values above `u64::MAX`.

| Scenario | Median per-iteration | Time per value |
|---|---:|---:|
| humfmt/u128_extended | 662 ns | 166 ns |

## Bytes — negative values (i64) — humfmt only

> No other benchmarked crate in this harness supports signed byte inputs.

| Scenario | Median per-iteration | Time per value |
|---|---:|---:|
| humfmt/negative_i64 | 281 ns | 70 ns |

## Numbers — allocating (`to_string`)

> human_format accepts f64 only and returns an owned `String`. humfmt accepts all integer and float primitives.

| Implementation | Median per-iteration | time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  i8-u128 + f32/f64, locale-aware | **2.12 us** | **212 ns** | 1.00x |
| human_format  f64 only, EN only, returns String | 2.22 us | 222 ns | 1.05x |

## Duration formatting — allocating

> humantime renders all non-zero units. humfmt caps at `max_units` (default 2). These produce different output for the same input.

| Implementation | Median per-iteration | time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  short, 2 units (default) | **749 ns** | **94 ns** | 1.00x |
| humantime  EN only, all non-zero units | 824 ns | 103 ns | 1.10x |
| humfmt  short, 3 units | 908 ns | 114 ns | 1.21x |
| humfmt  long labels, 2 units | 998 ns | 125 ns | 1.33x |

## Relative time — allocating

> timeago returns an owned `String` from `convert()`. humfmt implements `Display` and writes directly with no intermediate allocation.

| Implementation | Median per-iteration | time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  short, 2 units (default) | **861 ns** | **108 ns** | 1.00x |
| humfmt  short, 2 units (explicit) | 861 ns | 108 ns | 1.00x |
| timeago  EN, 1 unit (default), returns String | 1.04 us | 130 ns | 1.21x |
| humfmt  long, 2 units | 1.12 us | 140 ns | 1.30x |
| timeago  EN, 2 units, returns String | 1.58 us | 198 ns | 1.84x |


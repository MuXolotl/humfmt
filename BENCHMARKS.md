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
| prettier-bytes  u64 only, fixed 2dp, no negatives | **1.12 us** | **140 ns** | 0.59x |
| humfmt  i8-u128, any precision | 1.90 us | 238 ns | 1.00x |
| bytesize  u64 only | 2.23 us | 278 ns | 1.17x |
| byte-unit  u64/u128 | 8.99 us | 1.12 us | 4.72x |

## Bytes — reused buffer (`write!` into `String`), u64 inputs

| Implementation | Median per-iteration | time per value | Relative vs humfmt |
|---|---:|---:|---:|
| prettier-bytes  u64 only, fixed 2dp, no negatives | **361 ns** | **45 ns** | 0.29x |
| humfmt  i8-u128, any precision | 1.27 us | 158 ns | 1.00x |
| bytesize  u64 only | 1.44 us | 180 ns | 1.14x |
| byte-unit  u64/u128 | 6.99 us | 873 ns | 5.52x |

## Bytes — extended range (u128 > u64::MAX) — humfmt only

> No other benchmarked crate handles values above `u64::MAX`.

| Scenario | Median per-iteration | Time per value |
|---|---:|---:|
| humfmt/u128_extended | 1.66 us | 415 ns |

## Bytes — negative values (i64) — humfmt only

> No other benchmarked crate in this harness supports signed byte inputs.

| Scenario | Median per-iteration | Time per value |
|---|---:|---:|
| humfmt/negative_i64 | 865 ns | 216 ns |

## Numbers — allocating (`to_string`)

> human_format accepts f64 only and returns an owned `String`. humfmt accepts all integer and float primitives.

| Implementation | Median per-iteration | time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  i8-u128 + f32/f64, locale-aware | **4.34 us** | **434 ns** | 1.00x |
| human_format  f64 only, EN only, returns String | 5.69 us | 569 ns | 1.31x |

## Duration formatting — allocating

> humantime renders all non-zero units. humfmt caps at `max_units` (default 2). These produce different output for the same input.

| Implementation | Median per-iteration | time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  short, 2 units (default) | **2.68 us** | **335 ns** | 1.00x |
| humfmt  short, 3 units | 3.21 us | 401 ns | 1.20x |
| humantime  EN only, all non-zero units | 4.13 us | 516 ns | 1.54x |
| humfmt  long labels, 2 units | 4.49 us | 561 ns | 1.67x |

## Relative time — allocating

> timeago returns an owned `String` from `convert()`. humfmt implements `Display` and writes directly with no intermediate allocation.

| Implementation | Median per-iteration | time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  short, 2 units (default) | **2.64 us** | **330 ns** | 1.00x |
| humfmt  short, 2 units (explicit) | 2.70 us | 338 ns | 1.02x |
| humfmt  long, 2 units | 3.72 us | 464 ns | 1.41x |
| timeago  EN, 2 units, returns String | 5.62 us | 702 ns | 2.13x |
| timeago  EN, 1 unit (default), returns String | 6.76 us | 845 ns | 2.56x |


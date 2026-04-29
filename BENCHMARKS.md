# Benchmarks

This report is generated from Criterion `median.point_estimate` values.

Regenerate locally:

```bash
cargo bench --manifest-path tools/benchmarks/Cargo.toml
cargo run --release --manifest-path tools/benchmarks/Cargo.toml --bin report
```

---

## Capability Matrix

| Feature | humfmt | bytesize | byte-unit | prettier-bytes | indicatif (HumanBytes) | humantime | timeago | human_format |
|---|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
| Byte sizes | yes | yes | yes | yes | yes | no | no | no |
| Compact numbers | yes | no | no | no | no | no | no | yes |
| Duration formatting | yes | no | no | no | no | yes | yes | no |
| Relative time (ago) | yes | no | no | no | no | no | yes | no |
| Ordinals | yes | no | no | no | no | no | no | no |
| List formatting | yes | no | no | no | no | no | no | no |
| Signed input (negatives) | yes | no | no | no | no | — | — | no |
| u128 / i128 range | yes | no | partial | no | no | — | — | no |
| Float input | yes | no | no | no | no | — | — | yes |
| Long-form labels | yes | no | yes | no | no | yes | yes | yes |
| Max-units cap | yes | — | — | — | — | no | yes | — |
| Binary (IEC) units | yes | yes | yes | yes | yes | — | — | — |
| Configurable precision | yes | no | yes | no | no | — | — | yes |
| Locale-aware | yes | no | no | no | no | no | yes | no |
| Custom locale builder | yes | no | no | no | no | no | no | no |
| no_std compatible | yes | no | no | yes | no | no | no | no |
| Zero-alloc Display | yes | yes | no | yes | yes | yes | no | no |

---

## Notes

- Results depend on machine / OS / CPU scaling.
- **Bold** = best (lowest) value in column.
- Limitation tags are shown next to each crate name.
- Rows are sorted fastest to slowest within each group.
- Duration semantics can differ between crates (e.g. full-unit rendering vs capped output).
- Some crates return an owned `String` by design; `humfmt` formatters implement `Display`.
- Some groups are explicitly "aligned" to match a common output style (IEC + space, etc.).

---

## Bytes — allocating (`to_string`), u64 inputs

> prettier-bytes and bytesize are **u64-only**. humfmt accepts i8-i128 and u8-u128.

| Implementation | Median per-iteration | time per value | Relative vs humfmt |
|---|---:|---:|---:|
| prettier-bytes  u64 only, fixed 2dp, no negatives | **1.10 us** | **137 ns** | 0.60x |
| humfmt  i8-u128, any precision | 1.83 us | 229 ns | 1.00x |
| bytesize  u64 only | 2.29 us | 286 ns | 1.25x |
| byte-unit  u64/u128 | 8.87 us | 1.11 us | 4.84x |

## Bytes — allocating (`to_string`) — aligned (IEC + space), u64 inputs

> This group aligns formatting style (IEC units + space + 2dp) to compare against indicatif::HumanBytes.

| Implementation | Median per-iteration | time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  u64, IEC, 2dp, space | **1.29 us** | **215 ns** | 1.00x |
| indicatif HumanBytes  u64 only, IEC, fixed 2dp, space | 1.76 us | 294 ns | 1.37x |

## Bytes — reused buffer (`write!` into `String`), u64 inputs

| Implementation | Median per-iteration | time per value | Relative vs humfmt |
|---|---:|---:|---:|
| prettier-bytes  u64 only, fixed 2dp, no negatives | **382 ns** | **48 ns** | 0.36x |
| humfmt  i8-u128, any precision | 1.07 us | 134 ns | 1.00x |
| bytesize  u64 only | 1.41 us | 176 ns | 1.31x |
| byte-unit  u64/u128 | 7.14 us | 893 ns | 6.66x |

## Bytes — reused buffer (`write!` into `String`) — aligned (IEC + space), u64 inputs

| Implementation | Median per-iteration | time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  u64, IEC, 2dp, space | **734 ns** | **122 ns** | 1.00x |
| indicatif HumanBytes  u64 only, IEC, fixed 2dp, space | 1.11 us | 185 ns | 1.52x |

## Bytes — extended range (u128 > u64::MAX) — humfmt only

> No other benchmarked crate handles values above `u64::MAX`.

| Scenario | Median per-iteration | Time per value |
|---|---:|---:|
| humfmt/u128_extended | 1.63 us | 408 ns |

## Bytes — negative values (i64) — humfmt only

> No other benchmarked crate in this harness supports signed byte inputs.

| Scenario | Median per-iteration | Time per value |
|---|---:|---:|
| humfmt/negative_i64 | 884 ns | 221 ns |

## Numbers — allocating (`to_string`)

> human_format accepts f64 only and returns an owned `String`. humfmt accepts all integer and float primitives.

| Implementation | Median per-iteration | time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  i8-u128 + f32/f64, locale-aware | **4.08 us** | **408 ns** | 1.00x |
| human_format  f64 only, EN only, returns String | 6.04 us | 604 ns | 1.48x |

## Duration formatting — allocating

> humantime renders all non-zero units. humfmt caps at `max_units` (default 2). These produce different output for the same input.

| Implementation | Median per-iteration | time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  short, 2 units (default) | **2.15 us** | **268 ns** | 1.00x |
| humfmt  short, 3 units | 2.62 us | 327 ns | 1.22x |
| humantime  EN only, all non-zero units | 3.77 us | 472 ns | 1.76x |
| humfmt  long labels, 2 units | 3.89 us | 486 ns | 1.81x |

## Relative time — allocating

> timeago returns an owned `String` from `convert()`. humfmt implements `Display` and writes directly with no intermediate allocation.

| Implementation | Median per-iteration | time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  short, 2 units (explicit) | **2.64 us** | **329 ns** | 1.00x |
| humfmt  short, 2 units (default) | 2.66 us | 333 ns | 1.01x |
| humfmt  long, 2 units | 3.66 us | 458 ns | 1.39x |
| timeago  EN, 2 units, returns String | 5.68 us | 710 ns | 2.15x |
| timeago  EN, 1 unit (default), returns String | 6.73 us | 841 ns | 2.55x |


# Benchmarks

This report is generated from Criterion `median.point_estimate` values.

Regenerate locally:

```bash
cargo bench --manifest-path tools/benchmarks/Cargo.toml
cargo run --release --manifest-path tools/benchmarks/Cargo.toml --bin report
```

---

## Capability Matrix

| Feature | humfmt | humansize | bytesize | byte-unit | prettier-bytes | indicatif (HumanBytes) | human-repr | humantime | timeago | human_format |
|---|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
| Byte sizes | yes | yes | yes | yes | yes | yes | yes | no | no | no |
| Compact numbers | yes | no | no | no | no | no | yes | no | no | yes |
| Duration formatting | yes | no | no | no | no | no | yes | yes | yes | no |
| Relative time (ago) | yes | no | no | no | no | no | no | no | yes | no |
| Ordinals | yes | no | no | no | no | no | no | no | no | no |
| List formatting | yes | no | no | no | no | no | no | no | no | no |
| Signed input (negatives) | yes | yes | no | no | no | no | yes | — | — | no |
| u128 / i128 range | yes | no | no | partial | no | no | yes | — | — | no |
| Float input | yes | no | no | no | no | no | yes | — | — | yes |
| Long-form labels | yes | yes | no | yes | no | no | no | yes | yes | yes |
| Max-units cap | yes | — | — | — | — | — | — | no | yes | — |
| Binary (IEC) units | yes | yes | yes | yes | yes | yes | yes | — | — | — |
| Configurable precision | yes | yes | no | yes | no | no | no | — | — | yes |
| Locale-aware | yes | no | no | no | no | no | no | no | yes | no |
| Custom locale builder | yes | no | no | no | no | no | no | no | no | no |
| no_std compatible | yes | yes | no | no | yes | no | no | no | no | no |
| Zero-alloc Display | yes | yes | yes | no | yes | yes | yes | yes | no | no |

---

## Notes

- Results depend on machine / OS / CPU scaling.
- **Bold** = best (lowest) value in column.
- Limitation tags are shown next to each crate name.
- Rows are sorted fastest to slowest within each group.
- Duration semantics can differ between crates (e.g. full-unit rendering vs capped output).
- Some crates return an owned `String` by design; `humfmt` formatters implement `Display`.
- Some groups are explicitly "aligned" to match a common output style (IEC + space, etc.).
- Precision semantics differ: some crates keep fixed digits (e.g. `1.50`), while humfmt trims trailing zeros by design.

---

## Byte formatting semantics (examples)

These tables show representative outputs for a few byte values using the same configurations as the benchmarks.

### Default-style configuration

| Bytes | humfmt (SI, precision=2) | humansize (SI, precision=2, no space) | bytesize (SI, default) | byte-unit (`{:#.2}`) | prettier-bytes |
|---:|---|---|---|---|---|
| 1536 | `1.54KB` | `1.54kB` | `1.5 kB` | `1.5 KiB` | `1.54kB` |
| 9876543210 | `9.88GB` | `9.88GB` | `9.9 GB` | `9876543.21 KB` | `9.88GB` |

### Aligned configuration (IEC + space + precision=2)

| Bytes | humfmt (IEC, precision=2, trims) | indicatif HumanBytes | humansize (IEC, fixed 2dp, space) | bytesize (`iec`, `:.2`) | byte-unit (binary, `:.2`) | human-repr (iec+space) |
|---:|---|---|---|---|---|---|
| 1536 | `1.5 KiB` | `1.50 KiB` | `1.50 KiB` | `1.50 KiB` | `1.50 KiB` | `1.5 KiB` |
| 1500 | `1.46 KiB` | `1.46 KiB` | `1.46 KiB` | `1.46 KiB` | `1.46 KiB` | `1.5 KiB` |
| 1514000000 | `1.41 GiB` | `1.41 GiB` | `1.41 GiB` | `1.41 GiB` | `1.41 GiB` | `1.41 GiB` |

---

## Bytes — allocating (`to_string`), u64 inputs

> prettier-bytes, bytesize, humansize, and indicatif are **u64-only** in this harness. humfmt accepts i8-i128 and u8-u128.

| Implementation | Median per-iteration | time per value | Relative vs humfmt |
|---|---:|---:|---:|
| prettier-bytes  u64 only, fixed 2dp, no negatives | **1.10 us** | **137 ns** | 0.56x |
| humfmt  i8-u128, any precision | 1.95 us | 244 ns | 1.00x |
| bytesize  u64 only (SI), default 1dp, space | 2.20 us | 275 ns | 1.13x |
| humansize  u64 only, SI, precision=2, no space | 2.90 us | 363 ns | 1.49x |
| byte-unit  u64 (auto unit), format! uses String | 8.39 us | 1.05 us | 4.30x |

## Bytes — allocating (`to_string`) — aligned (IEC + space + precision=2), u64 inputs

> This group aligns unit system and spacing. Decimal digit policy can still differ (fixed digits vs trimmed zeros).

| Implementation | Median per-iteration | time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  u64, IEC, precision=2, space (trims zeros) | **1.32 us** | **219 ns** | 1.00x |
| byte-unit  u64 only, IEC, fixed 2dp, space | 1.66 us | 277 ns | 1.26x |
| indicatif HumanBytes  u64 only, IEC, fixed 2dp, space | 1.74 us | 291 ns | 1.32x |
| bytesize  u64 only, IEC, fixed 2dp, space | 1.81 us | 301 ns | 1.37x |
| humansize  u64 only, IEC, fixed 2dp, space | 2.04 us | 340 ns | 1.55x |
| human-repr  u64, IEC+space (feature), decimals are algorithmic | 2.96 us | 494 ns | 2.25x |

## Bytes — reused buffer (`write!` into `String`), u64 inputs

| Implementation | Median per-iteration | time per value | Relative vs humfmt |
|---|---:|---:|---:|
| prettier-bytes  u64 only, fixed 2dp, no negatives | **402 ns** | **50 ns** | 0.36x |
| humfmt  i8-u128, any precision | 1.12 us | 140 ns | 1.00x |
| bytesize  u64 only (SI), default 1dp, space | 1.45 us | 181 ns | 1.30x |
| humansize  u64 only, SI, precision=2, no space | 2.23 us | 278 ns | 1.99x |
| byte-unit  u64 (auto unit), write! + Display | 7.06 us | 882 ns | 6.31x |

## Bytes — reused buffer (`write!` into `String`) — aligned (IEC + space + precision=2), u64 inputs

| Implementation | Median per-iteration | time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  u64, IEC, precision=2, space (trims zeros) | **776 ns** | **129 ns** | 1.00x |
| byte-unit  u64 only, IEC, fixed 2dp, space | 1.03 us | 171 ns | 1.33x |
| bytesize  u64 only, IEC, fixed 2dp, space | 1.16 us | 193 ns | 1.49x |
| indicatif HumanBytes  u64 only, IEC, fixed 2dp, space | 1.16 us | 194 ns | 1.50x |
| humansize  u64 only, IEC, fixed 2dp, space | 1.48 us | 247 ns | 1.91x |
| human-repr  u64, IEC+space (feature), decimals are algorithmic | 2.04 us | 341 ns | 2.64x |

## Bytes — extended range (u128 > u64::MAX) — humfmt only

> No other benchmarked crate handles values above `u64::MAX`.

| Scenario | Median per-iteration | Time per value |
|---|---:|---:|
| humfmt/u128_extended | 1.63 us | 408 ns |

## Bytes — negative values (i64)

> bytesize and prettier-bytes do not participate (unsigned-only). This harness includes humfmt and humansize.

| Scenario | Median per-iteration | Time per value |
|---|---:|---:|
| humfmt/negative_i64 | 852 ns | 213 ns |
| humansize/negative_i64 | 1.37 us | 342 ns |

## Numbers — allocating (`to_string`)

> human_format accepts f64 only and returns an owned `String`. humfmt accepts all integer and float primitives.

| Implementation | Median per-iteration | time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  i8-u128 + f32/f64, locale-aware | **4.13 us** | **413 ns** | 1.00x |
| human_format  f64 only, EN only, returns String | 5.74 us | 574 ns | 1.39x |

## Duration formatting — allocating

> humantime renders all non-zero units. humfmt caps at `max_units` (default 2). These produce different output for the same input.

| Implementation | Median per-iteration | time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humantime  EN only, all non-zero units | **2.47 us** | **309 ns** | 0.86x |
| humfmt  short, 2 units (default) | 2.88 us | 360 ns | 1.00x |
| humfmt  short, 3 units | 2.99 us | 374 ns | 1.04x |
| humfmt  long labels, 2 units | 4.31 us | 539 ns | 1.50x |

## Relative time — allocating

> timeago returns an owned `String` from `convert()`. humfmt implements `Display` and writes directly with no intermediate allocation.

| Implementation | Median per-iteration | time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  short, 2 units (explicit) | **2.58 us** | **322 ns** | 1.00x |
| humfmt  short, 2 units (default) | 2.59 us | 324 ns | 1.00x |
| humfmt  long, 2 units | 3.49 us | 436 ns | 1.35x |
| timeago  EN, 2 units, returns String | 5.83 us | 728 ns | 2.26x |
| timeago  EN, 1 unit (default), returns String | 6.94 us | 867 ns | 2.69x |


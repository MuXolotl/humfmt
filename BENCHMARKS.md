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
| prettier-bytes  u64 only, fixed 2dp, no negatives | **499 ns** | **62 ns** | 0.74x |
| humfmt  i8-u128, any precision | 676 ns | 85 ns | 1.00x |
| bytesize  u64 only (SI), default 1dp, space | 1.03 us | 129 ns | 1.52x |
| humansize  u64 only, SI, precision=2, no space | 1.39 us | 173 ns | 2.05x |
| byte-unit  u64 (auto unit), format! uses String | 4.60 us | 574 ns | 6.80x |

## Bytes — allocating (`to_string`) — aligned (IEC + space + precision=2), u64 inputs

> This group aligns unit system and spacing. Decimal digit policy can still differ (fixed digits vs trimmed zeros).

| Implementation | Median per-iteration | time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  u64, IEC, precision=2, space (trims zeros) | **503 ns** | **84 ns** | 1.00x |
| indicatif HumanBytes  u64 only, IEC, fixed 2dp, space | 769 ns | 128 ns | 1.53x |
| bytesize  u64 only, IEC, fixed 2dp, space | 847 ns | 141 ns | 1.68x |
| byte-unit  u64 only, IEC, fixed 2dp, space | 867 ns | 145 ns | 1.72x |
| humansize  u64 only, IEC, fixed 2dp, space | 934 ns | 156 ns | 1.86x |
| human-repr  u64, IEC+space (feature), decimals are algorithmic | 1.04 us | 173 ns | 2.06x |

## Bytes — reused buffer (`write!` into `String`), u64 inputs

| Implementation | Median per-iteration | time per value | Relative vs humfmt |
|---|---:|---:|---:|
| prettier-bytes  u64 only, fixed 2dp, no negatives | **195 ns** | **24 ns** | 0.38x |
| humfmt  i8-u128, any precision | 519 ns | 65 ns | 1.00x |
| bytesize  u64 only (SI), default 1dp, space | 896 ns | 112 ns | 1.73x |
| humansize  u64 only, SI, precision=2, no space | 1.19 us | 149 ns | 2.30x |
| byte-unit  u64 (auto unit), write! + Display | 4.43 us | 553 ns | 8.53x |

## Bytes — reused buffer (`write!` into `String`) — aligned (IEC + space + precision=2), u64 inputs

| Implementation | Median per-iteration | time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  u64, IEC, precision=2, space (trims zeros) | **370 ns** | **62 ns** | 1.00x |
| byte-unit  u64 only, IEC, fixed 2dp, space | 575 ns | 96 ns | 1.55x |
| bytesize  u64 only, IEC, fixed 2dp, space | 622 ns | 104 ns | 1.68x |
| indicatif HumanBytes  u64 only, IEC, fixed 2dp, space | 653 ns | 109 ns | 1.77x |
| humansize  u64 only, IEC, fixed 2dp, space | 821 ns | 137 ns | 2.22x |
| human-repr  u64, IEC+space (feature), decimals are algorithmic | 936 ns | 156 ns | 2.53x |

## Bytes — extended range (u128 > u64::MAX) — humfmt only

> No other benchmarked crate handles values above `u64::MAX`.

| Scenario | Median per-iteration | Time per value |
|---|---:|---:|
| humfmt/u128_extended | 677 ns | 169 ns |

## Bytes — negative values (i64)

> bytesize and prettier-bytes do not participate (unsigned-only). This harness includes humfmt and humansize.

| Scenario | Median per-iteration | Time per value |
|---|---:|---:|
| humfmt/negative_i64 | 322 ns | 81 ns |
| humansize/negative_i64 | 674 ns | 168 ns |

## Numbers — allocating (`to_string`)

> human_format accepts f64 only and returns an owned `String`. humfmt accepts all integer and float primitives.

| Implementation | Median per-iteration | time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  i8-u128 + f32/f64, locale-aware | **2.08 us** | **208 ns** | 1.00x |
| human_format  f64 only, EN only, returns String | 2.33 us | 233 ns | 1.12x |

## Duration formatting — allocating

> humantime renders all non-zero units. humfmt caps at `max_units` (default 2). These produce different output for the same input.

| Implementation | Median per-iteration | time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  short, 2 units (default) | **759 ns** | **95 ns** | 1.00x |
| humantime  EN only, all non-zero units | 840 ns | 105 ns | 1.11x |
| humfmt  short, 3 units | 972 ns | 122 ns | 1.28x |
| humfmt  long labels, 2 units | 1.10 us | 137 ns | 1.45x |

## Relative time — allocating

> timeago returns an owned `String` from `convert()`. humfmt implements `Display` and writes directly with no intermediate allocation.

| Implementation | Median per-iteration | time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  short, 2 units (explicit) | **846 ns** | **106 ns** | 1.00x |
| humfmt  short, 2 units (default) | 863 ns | 108 ns | 1.02x |
| timeago  EN, 1 unit (default), returns String | 1.07 us | 134 ns | 1.27x |
| humfmt  long, 2 units | 1.08 us | 135 ns | 1.28x |
| timeago  EN, 2 units, returns String | 1.66 us | 208 ns | 1.96x |


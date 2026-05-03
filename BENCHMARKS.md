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
| Compact numbers | yes | no | no | no | no | no | no | no | no | yes |
| Duration formatting | yes | no | no | no | no | no | yes | yes | yes | no |
| Relative time (ago) | yes | no | no | no | no | no | no | no | yes | no |
| Ordinals | yes | no | no | no | no | no | no | no | no | no |
| List formatting | yes | no | no | no | no | no | no | no | no | no |
| Percentage | yes | no | no | no | no | no | no | no | no | no |
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
- No crate other than humfmt and human_format produces compact `K/M/B` style number output; human-repr and readable produce grouped digits (`1,000`) instead.

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

> prettier-bytes, bytesize, humansize, and indicatif are **u64-only** in this harness. humfmt accepts i8–i128 and u8–u128.

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| prettier-bytes  u64 only, fixed 2dp, no negatives | **321 ns** | **40 ns** | 0.44x |
| humfmt  i8-u128, any precision | 727 ns | 91 ns | 1.00x |
| bytesize  u64 only (SI), default 1dp, space | 979 ns | 122 ns | 1.35x |
| humansize  u64 only, SI, precision=2, no space | 1.37 us | 171 ns | 1.88x |
| byte-unit  u64 (auto unit), format! uses String | 4.49 us | 562 ns | 6.18x |

## Bytes — allocating (`to_string`) — aligned (IEC + space + precision=2), u64 inputs

> This group aligns unit system and spacing. Decimal digit policy can still differ (fixed digits vs trimmed zeros).

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  u64, IEC, precision=2, space (trims zeros) | **537 ns** | **89 ns** | 1.00x |
| byte-unit  u64 only, IEC, fixed 2dp, space | 725 ns | 121 ns | 1.35x |
| indicatif HumanBytes  u64 only, IEC, fixed 2dp, space | 747 ns | 124 ns | 1.39x |
| bytesize  u64 only, IEC, fixed 2dp, space | 769 ns | 128 ns | 1.43x |
| humansize  u64 only, IEC, fixed 2dp, space | 941 ns | 157 ns | 1.75x |
| human-repr  u64, IEC+space (feature) | 1.06 us | 177 ns | 1.98x |

## Bytes — reused buffer (`write!` into `String`), u64 inputs

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| prettier-bytes  u64 only, fixed 2dp, no negatives | **194 ns** | **24 ns** | 0.31x |
| humfmt  i8-u128, any precision | 634 ns | 79 ns | 1.00x |
| bytesize  u64 only (SI), default 1dp, space | 868 ns | 109 ns | 1.37x |
| humansize  u64 only, SI, precision=2, no space | 1.20 us | 150 ns | 1.89x |
| byte-unit  u64 (auto unit), write! + Display | 4.21 us | 527 ns | 6.65x |

## Bytes — reused buffer (`write!` into `String`) — aligned (IEC + space + precision=2), u64 inputs

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  u64, IEC, precision=2, space (trims zeros) | **458 ns** | **76 ns** | 1.00x |
| byte-unit  u64 only, IEC, fixed 2dp, space | 582 ns | 97 ns | 1.27x |
| bytesize  u64 only, IEC, fixed 2dp, space | 618 ns | 103 ns | 1.35x |
| indicatif HumanBytes  u64 only, IEC, fixed 2dp, space | 647 ns | 108 ns | 1.41x |
| humansize  u64 only, IEC, fixed 2dp, space | 818 ns | 136 ns | 1.79x |
| human-repr  u64, IEC+space (feature) | 934 ns | 156 ns | 2.04x |

## Bytes — extended range (u128 > u64::MAX) — humfmt only

> No other benchmarked crate handles values above `u64::MAX`.

| Scenario | Median per-iteration | Time per value |
|---|---:|---:|
| humfmt/u128_extended | 745 ns | 186 ns |

## Bytes — negative values (i64)

> bytesize and prettier-bytes do not participate (unsigned-only). This harness includes humfmt and humansize.

| Scenario | Median per-iteration | Time per value |
|---|---:|---:|
| humfmt/negative_i64 | 339 ns | 85 ns |
| humansize/negative_i64 | 652 ns | 163 ns |

## Numbers — allocating (`to_string`), mixed i64 inputs

> human_format accepts f64 only and always returns an owned `String`. humfmt accepts all integer and float primitives and implements `Display`.

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  i64, precision=1 (default) | **825 ns** | **83 ns** | 1.00x |
| humfmt  i64, precision=2 | 878 ns | 88 ns | 1.06x |
| human_format  f64 only, EN only, precision=2, returns String | 2.54 us | 254 ns | 3.08x |

## Numbers — allocating (`to_string`), u64 inputs (apples-to-apples)

> human_format receives u64 cast to f64. Both crates produce compact `K/M/B` style output.

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  u64, precision=1 | **671 ns** | **84 ns** | 1.00x |
| humfmt  u64, precision=2 | 703 ns | 88 ns | 1.05x |
| human_format  u64 as f64, precision=2, returns String | 1.78 us | 222 ns | 2.65x |
| human_format  u64 as f64, precision=1, returns String | 1.90 us | 238 ns | 2.83x |

## Numbers — allocating (`to_string`), f64 inputs

> Float path only. human_format accepts f64 natively. human-repr and readable do not produce compact suffixes and are excluded.

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| human_format  f64, precision=2, returns String | **1.58 us** | **198 ns** | 0.96x |
| humfmt  f64, precision=2 | 1.65 us | 206 ns | 1.00x |

## Numbers — reused buffer (`write!` into `String`), u64 inputs

> humfmt writes via `Display` with no intermediate allocation. human_format always allocates a `String`; we `push_str` it into the buffer.

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  u64, precision=1, write! | **580 ns** | **73 ns** | 1.00x |
| humfmt  u64, precision=2, write! | 619 ns | 77 ns | 1.07x |
| human_format  u64 as f64, precision=2, push_str (always allocs) | 1.77 us | 221 ns | 3.04x |

## Numbers — locale overhead (humfmt only)

> Measures the cost of locale-aware formatting. Russian and Polish require plural form selection based on the rendered value.

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  English, short | **670 ns** | **84 ns** | 1.00x |
| humfmt  Polish, short | 731 ns | 91 ns | 1.09x |
| humfmt  English, long | 735 ns | 92 ns | 1.10x |
| humfmt  Russian, short | 780 ns | 98 ns | 1.16x |
| humfmt  Polish, long (plural selection) | 843 ns | 105 ns | 1.26x |
| humfmt  Russian, long (plural selection) | 860 ns | 107 ns | 1.28x |

## Duration formatting — allocating

> humantime renders all non-zero units. humfmt caps at `max_units` (default 2). These produce different output for the same input.

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  short, 2 units (default) | **767 ns** | **96 ns** | 1.00x |
| humantime  EN only, all non-zero units | 848 ns | 106 ns | 1.11x |
| humfmt  short, 3 units | 991 ns | 124 ns | 1.29x |
| humfmt  long labels, 2 units | 1.09 us | 137 ns | 1.43x |

## Relative time — allocating

> timeago returns an owned `String` from `convert()`. humfmt implements `Display` and writes directly with no intermediate allocation.

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  short, 2 units (default) | **918 ns** | **115 ns** | 1.00x |
| humfmt  short, 2 units (explicit) | 924 ns | 115 ns | 1.01x |
| timeago  EN, 1 unit (default), returns String | 1.05 us | 132 ns | 1.15x |
| humfmt  long, 2 units | 1.11 us | 139 ns | 1.21x |
| timeago  EN, 2 units, returns String | 1.67 us | 209 ns | 1.82x |


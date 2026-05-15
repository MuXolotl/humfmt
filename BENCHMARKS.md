# Benchmarks

This report is generated from Criterion `median.point_estimate` values.

Regenerate locally:

```bash
cargo bench --manifest-path tools/benchmarks/Cargo.toml
cargo run --release --manifest-path tools/benchmarks/Cargo.toml --bin report
```

---

## Capability Matrix

| Feature | humfmt | humansize | bytesize | byte-unit | prettier-bytes | indicatif (HumanBytes) | human-repr | humantime | timeago | human_format | numfmt |
|---|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
| Byte sizes | yes | yes | yes | yes | yes | yes | yes | no | no | no | no |
| Compact numbers | yes | no | no | no | no | no | no | no | no | yes | yes |
| Duration formatting | yes | no | no | no | no | no | yes | yes | yes | no | no |
| Relative time (ago) | yes | no | no | no | no | no | no | no | yes | no | no |
| Ordinals | yes | no | no | no | no | no | no | no | no | no | no |
| List formatting | yes | no | no | no | no | no | no | no | no | no | no |
| Percentage | yes | no | no | no | no | no | no | no | no | no | no |
| Signed input (negatives) | yes | yes | no | no | no | no | yes | — | — | no | yes |
| u128 / i128 range | yes | no | no | partial | no | no | yes | — | — | no | no |
| Float input | yes | no | no | no | no | no | yes | — | — | yes | yes |
| Long-form labels | yes | yes | no | yes | no | no | no | yes | yes | yes | no |
| Max-units cap | yes | — | — | — | — | — | — | no | yes | — | — |
| Binary (IEC) units | yes | yes | yes | yes | yes | yes | yes | — | — | — | — |
| Configurable precision | yes | yes | no | yes | no | no | no | — | — | yes | yes |
| Custom decimal/group separators | yes | no | no | no | no | no | no | no | no | yes | yes |
| no_std compatible | yes | yes | no | no | yes | no | no | no | no | no | no |
| Zero-alloc Display | yes | yes | yes | no | yes | yes | yes | yes | no | no | partial |

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
- Compact `K/M/B` style number output is produced by humfmt, human_format, and numfmt; human-repr and readable produce grouped digits (`1,000`) instead.

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
| prettier-bytes  u64 only, fixed 2dp, no negatives | **365 ns** | **46 ns** | 0.47x |
| humfmt  i8-u128, any precision | 778 ns | 97 ns | 1.00x |
| bytesize  u64 only (SI), default 1dp, space | 1.05 us | 131 ns | 1.35x |
| humansize  u64 only, SI, precision=2, no space | 1.38 us | 172 ns | 1.77x |
| byte-unit  u64 (auto unit), format! uses String | 3.99 us | 499 ns | 5.14x |

## Bytes — allocating (`to_string`) — aligned (IEC + space + precision=2), u64 inputs

> This group aligns unit system and spacing. Decimal digit policy can still differ (fixed digits vs trimmed zeros).

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  u64, IEC, precision=2, space (trims zeros) | **540 ns** | **90 ns** | 1.00x |
| byte-unit  u64 only, IEC, fixed 2dp, space | 731 ns | 122 ns | 1.35x |
| indicatif HumanBytes  u64 only, IEC, fixed 2dp, space | 777 ns | 129 ns | 1.44x |
| bytesize  u64 only, IEC, fixed 2dp, space | 804 ns | 134 ns | 1.49x |
| humansize  u64 only, IEC, fixed 2dp, space | 954 ns | 159 ns | 1.77x |
| human-repr  u64, IEC+space (feature) | 1.07 us | 178 ns | 1.98x |

## Bytes — reused buffer (`write!` into `String`), u64 inputs

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| prettier-bytes  u64 only, fixed 2dp, no negatives | **191 ns** | **24 ns** | 0.26x |
| humfmt  i8-u128, any precision | 734 ns | 92 ns | 1.00x |
| bytesize  u64 only (SI), default 1dp, space | 895 ns | 112 ns | 1.22x |
| humansize  u64 only, SI, precision=2, no space | 1.24 us | 155 ns | 1.69x |
| byte-unit  u64 (auto unit), write! + Display | 3.75 us | 468 ns | 5.10x |

## Bytes — reused buffer (`write!` into `String`) — aligned (IEC + space + precision=2), u64 inputs

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  u64, IEC, precision=2, space (trims zeros) | **526 ns** | **88 ns** | 1.00x |
| byte-unit  u64 only, IEC, fixed 2dp, space | 557 ns | 93 ns | 1.06x |
| bytesize  u64 only, IEC, fixed 2dp, space | 630 ns | 105 ns | 1.20x |
| indicatif HumanBytes  u64 only, IEC, fixed 2dp, space | 663 ns | 111 ns | 1.26x |
| humansize  u64 only, IEC, fixed 2dp, space | 838 ns | 140 ns | 1.59x |
| human-repr  u64, IEC+space (feature) | 975 ns | 162 ns | 1.85x |

## Bytes — extended range (u128 > u64::MAX) — humfmt only

> No other benchmarked crate handles values above `u64::MAX`.

| Scenario | Median per-iteration | Time per value |
|---|---:|---:|
| humfmt/u128_extended | 798 ns | 200 ns |

## Bytes — negative values (i64)

> bytesize and prettier-bytes do not participate (unsigned-only). This harness includes humfmt and humansize.

| Scenario | Median per-iteration | Time per value |
|---|---:|---:|
| humfmt/negative_i64 | 342 ns | 86 ns |
| humansize/negative_i64 | 676 ns | 169 ns |

## Numbers — allocating (`to_string`), mixed i64 inputs

> human_format accepts f64 only and always returns an owned `String`. humfmt accepts all integer and float primitives and implements `Display`. numfmt accepts u64/i64/f64 and returns a borrowed `&str` from an internal buffer.

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| numfmt  i64, short scale, precision=2 | **757 ns** | **76 ns** | 0.88x |
| humfmt  i64, precision=1 (default) | 858 ns | 86 ns | 1.00x |
| humfmt  i64, precision=2 | 922 ns | 92 ns | 1.08x |
| human_format  f64 only, precision=2, returns String | 2.35 us | 235 ns | 2.74x |

## Numbers — allocating (`to_string`), u64 inputs (apples-to-apples)

> human_format receives u64 cast to f64. All three crates produce compact `K/M/B` style output.

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| numfmt  u64, short scale, precision=2 | **638 ns** | **80 ns** | 0.92x |
| humfmt  u64, precision=1 | 695 ns | 87 ns | 1.00x |
| humfmt  u64, precision=2 | 754 ns | 94 ns | 1.08x |
| human_format  u64 as f64, precision=2, returns String | 1.63 us | 204 ns | 2.35x |
| human_format  u64 as f64, precision=1, returns String | 1.76 us | 220 ns | 2.53x |

## Numbers — allocating (`to_string`), f64 inputs

> Float path only. human_format accepts f64 natively. human-repr and readable do not produce compact suffixes and are excluded.

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| numfmt  f64, short scale, precision=2 | **622 ns** | **78 ns** | 0.40x |
| human_format  f64, precision=2, returns String | 1.45 us | 182 ns | 0.93x |
| humfmt  f64, precision=2 | 1.56 us | 195 ns | 1.00x |

## Numbers — reused buffer (`write!` into `String`), u64 inputs

> humfmt writes via `Display` with no intermediate allocation. human_format always allocates a `String`; we `push_str` it into the buffer. numfmt returns a `&str` from its internal buffer; we `push_str` it.

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| numfmt  u64, short scale, precision=2, push_str (returns &str) | **551 ns** | **69 ns** | 0.95x |
| humfmt  u64, precision=1, write! | 580 ns | 73 ns | 1.00x |
| humfmt  u64, precision=2, write! | 648 ns | 81 ns | 1.12x |
| human_format  u64 as f64, precision=2, push_str (always allocs) | 1.64 us | 205 ns | 2.82x |

## Duration formatting — allocating

> humantime renders all non-zero units. humfmt caps at `max_units` (default 2). These produce different output for the same input.

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  short, 2 units (default) | **676 ns** | **85 ns** | 1.00x |
| humantime  all non-zero units | 804 ns | 100 ns | 1.19x |
| humfmt  short, 3 units | 869 ns | 109 ns | 1.28x |
| humfmt  long labels, 2 units | 926 ns | 116 ns | 1.37x |

## Relative time — allocating

> timeago returns an owned `String` from `convert()`. humfmt implements `Display` and writes directly with no intermediate allocation.

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  short, 2 units (default) | **737 ns** | **92 ns** | 1.00x |
| humfmt  short, 2 units (explicit) | 746 ns | 93 ns | 1.01x |
| humfmt  long, 2 units | 943 ns | 118 ns | 1.28x |
| timeago  1 unit (default), returns String | 1.01 us | 127 ns | 1.37x |
| timeago  2 units, returns String | 1.57 us | 196 ns | 2.13x |


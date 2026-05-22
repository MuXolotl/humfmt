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
- Humfmt-only benchmark groups are not competitor comparisons; they exist to track feature-cost and regression behaviour inside humfmt.

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
| prettier-bytes  u64 only, fixed 2dp, no negatives | **315 ns** | **39 ns** | 0.43x |
| humfmt  i8-u128, any precision | 729 ns | 91 ns | 1.00x |
| bytesize  u64 only (SI), default 1dp, space | 1.03 us | 128 ns | 1.41x |
| humansize  u64 only, SI, precision=2, no space | 1.39 us | 174 ns | 1.91x |
| byte-unit  u64 (auto unit), format! uses String | 3.79 us | 474 ns | 5.20x |

## Bytes — allocating (`to_string`) — aligned (IEC + space + precision=2), u64 inputs

> This group aligns unit system and spacing. Decimal digit policy can still differ (fixed digits vs trimmed zeros).

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  u64, IEC, precision=2, space (trims zeros) | **536 ns** | **89 ns** | 1.00x |
| byte-unit  u64 only, IEC, fixed 2dp, space | 738 ns | 123 ns | 1.38x |
| indicatif HumanBytes  u64 only, IEC, fixed 2dp, space | 767 ns | 128 ns | 1.43x |
| bytesize  u64 only, IEC, fixed 2dp, space | 803 ns | 134 ns | 1.50x |
| humansize  u64 only, IEC, fixed 2dp, space | 960 ns | 160 ns | 1.79x |
| human-repr  u64, IEC+space (feature) | 1.06 us | 177 ns | 1.98x |

## Bytes — reused buffer (`write!` into `String`), u64 inputs

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| prettier-bytes  u64 only, fixed 2dp, no negatives | **193 ns** | **24 ns** | 0.30x |
| humfmt  i8-u128, any precision | 638 ns | 80 ns | 1.00x |
| bytesize  u64 only (SI), default 1dp, space | 903 ns | 113 ns | 1.42x |
| humansize  u64 only, SI, precision=2, no space | 1.19 us | 149 ns | 1.87x |
| byte-unit  u64 (auto unit), write! + Display | 3.53 us | 442 ns | 5.54x |

## Bytes — reused buffer (`write!` into `String`) — aligned (IEC + space + precision=2), u64 inputs

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  u64, IEC, precision=2, space (trims zeros) | **462 ns** | **77 ns** | 1.00x |
| byte-unit  u64 only, IEC, fixed 2dp, space | 593 ns | 99 ns | 1.28x |
| bytesize  u64 only, IEC, fixed 2dp, space | 642 ns | 107 ns | 1.39x |
| indicatif HumanBytes  u64 only, IEC, fixed 2dp, space | 666 ns | 111 ns | 1.44x |
| humansize  u64 only, IEC, fixed 2dp, space | 844 ns | 141 ns | 1.83x |
| human-repr  u64, IEC+space (feature) | 969 ns | 162 ns | 2.10x |

## Bytes — extended range (u128 > u64::MAX) — humfmt only

> No other benchmarked crate handles values above `u64::MAX`.

| Scenario | Median per-iteration | Time per value |
|---|---:|---:|
| humfmt/u128_extended | 741 ns | 185 ns |

## Bytes — negative values (i64)

> bytesize and prettier-bytes do not participate (unsigned-only). This harness includes humfmt and humansize.

| Scenario | Median per-iteration | Time per value |
|---|---:|---:|
| humfmt/negative_i64 | 340 ns | 85 ns |
| humansize/negative_i64 | 665 ns | 166 ns |

## Numbers — allocating (`to_string`), mixed i64 inputs

> human_format accepts f64 only and always returns an owned `String`. humfmt accepts all integer and float primitives and implements `Display`. numfmt accepts u64/i64/f64 and returns a borrowed `&str` from an internal buffer.

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| numfmt  i64, short scale, precision=2 | **783 ns** | **78 ns** | 0.95x |
| humfmt  i64, precision=1 (default) | 825 ns | 82 ns | 1.00x |
| humfmt  i64, precision=2 | 868 ns | 87 ns | 1.05x |
| human_format  f64 only, precision=2, returns String | 2.50 us | 250 ns | 3.04x |

## Numbers — allocating (`to_string`), u64 inputs (apples-to-apples)

> human_format receives u64 cast to f64. All three crates produce compact `K/M/B` style output.

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| numfmt  u64, short scale, precision=2 | **639 ns** | **80 ns** | 0.96x |
| humfmt  u64, precision=1 | 667 ns | 83 ns | 1.00x |
| humfmt  u64, precision=2 | 699 ns | 87 ns | 1.05x |
| human_format  u64 as f64, precision=2, returns String | 1.73 us | 216 ns | 2.59x |
| human_format  u64 as f64, precision=1, returns String | 1.87 us | 234 ns | 2.80x |

## Numbers — allocating (`to_string`), f64 inputs

> Float path only. human_format accepts f64 natively. human-repr and readable do not produce compact suffixes and are excluded.

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| numfmt  f64, short scale, precision=2 | **618 ns** | **77 ns** | 0.40x |
| human_format  f64, precision=2, returns String | 1.54 us | 193 ns | 0.99x |
| humfmt  f64, precision=2 | 1.55 us | 194 ns | 1.00x |

## Numbers — humfmt option coverage (allocating)

> Humfmt-only group. These rows measure the cost of individual number-formatting options; they are not competitor comparisons.

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  i64, precision=0, rounding=floor | **642 ns** | **64 ns** | 1.00x |
| humfmt  i64, precision=0, rounding=ceil | 644 ns | 64 ns | 1.00x |
| humfmt  i64, force_sign | 813 ns | 81 ns | 1.27x |
| humfmt  i64, significant_digits=3 | 886 ns | 89 ns | 1.38x |
| humfmt  i64, long units, precision=2 | 949 ns | 95 ns | 1.48x |
| humfmt  i64, compact=false + separators | 1.07 us | 107 ns | 1.66x |
| humfmt  i64, custom decimal/group separators | 1.07 us | 107 ns | 1.66x |
| humfmt  f64, compact=false + separators | 1.77 us | 221 ns | 3.45x |
| humfmt  f64, significant_digits=3 | 1.99 us | 248 ns | 3.87x |

## Numbers — extended range (u128 > u64::MAX) — humfmt only

> Competitor crates in this harness either do not accept u128 inputs or do not cover the full u128/i128 range. This group tracks humfmt's extended integer path.

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  u128 extreme, default | **622 ns** | **104 ns** | 1.00x |
| humfmt  u128 extreme, significant_digits=3 | 682 ns | 114 ns | 1.10x |
| humfmt  u128 extreme, precision=2 | 684 ns | 114 ns | 1.10x |
| humfmt  u128 extreme, significant_digits=6 | 943 ns | 157 ns | 1.52x |
| humfmt  u128 extreme, compact=false | 1.59 us | 265 ns | 2.56x |
| humfmt  u128 extreme, compact=false + separators | 2.46 us | 409 ns | 3.95x |

## Numbers — reused buffer (`write!` into `String`), u64 inputs

> humfmt writes via `Display` with no intermediate allocation. human_format always allocates a `String`; we `push_str` it into the buffer. numfmt returns a `&str` from its internal buffer; we `push_str` it.

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| numfmt  u64, short scale, precision=2, push_str (returns &str) | **521 ns** | **65 ns** | 0.93x |
| humfmt  u64, precision=1, write! | 562 ns | 70 ns | 1.00x |
| humfmt  u64, precision=2, write! | 599 ns | 75 ns | 1.07x |
| human_format  u64 as f64, precision=2, push_str (always allocs) | 1.77 us | 221 ns | 3.15x |

## Numbers — reused buffer, humfmt option coverage

> Humfmt-only group. These rows measure reused-buffer formatting for option-heavy and extended-range number scenarios.

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  i64, significant_digits=3, write! | 756 ns | **76 ns** | 1.00x |
| humfmt  i64, compact=false + separators, write! | 911 ns | 91 ns | 1.21x |
| humfmt  u128 extreme, default, write! | **551 ns** | 92 ns | 1.22x |
| humfmt  u128 extreme, significant_digits=3, write! | 611 ns | 102 ns | 1.35x |
| humfmt  f64, precision=2, write! | 1.41 us | 177 ns | 2.34x |

## Duration formatting — allocating

> humantime renders all non-zero units. humfmt caps at `max_units` (default 2). These produce different output for the same input.

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  short, 2 units (default) | **694 ns** | **87 ns** | 1.00x |
| humantime  all non-zero units | 824 ns | 103 ns | 1.19x |
| humfmt  short, 3 units | 844 ns | 106 ns | 1.22x |
| humfmt  long labels, 2 units | 1.02 us | 128 ns | 1.48x |

## Relative time — allocating

> timeago returns an owned `String` from `convert()`. humfmt implements `Display` and writes directly with no intermediate allocation.

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  short, 2 units (explicit) | **730 ns** | **91 ns** | 1.00x |
| humfmt  short, 2 units (default) | 750 ns | 94 ns | 1.03x |
| humfmt  long, 2 units | 955 ns | 119 ns | 1.31x |
| timeago  1 unit (default), returns String | 1.02 us | 127 ns | 1.39x |
| timeago  2 units, returns String | 1.59 us | 198 ns | 2.17x |


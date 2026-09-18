# Benchmarks

This report is generated from Criterion `median.point_estimate` values.

Regenerate locally:

```bash
cargo bench --manifest-path tools/benchmarks/Cargo.toml
cargo run --release --manifest-path tools/benchmarks/Cargo.toml --bin report
```

## Environment

| Item | Value |
|---|---|
| CPU | AMD EPYC 7763 64-Core Processor |
| Cores available | 4 |
| OS | linux x86_64 |
| Rust | rustc 1.98.1 (48a229cea 2026-09-01) |
| Criterion | 0.5.1 |
| Build profile | opt-level 3, LTO off, 16 codegen units |

All timings are Criterion median point estimates, taken with Criterion defaults (100 samples, 3 s warm-up, 5 s measurement). `Time per value` divides a median by the number of values formatted per iteration.

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

- Results are specific to the machine and toolchain listed under Environment.
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
| prettier-bytes  u64 only, fixed 2dp, no negatives | **318 ns** | **40 ns** | 0.43x |
| humfmt  i8-u128, any precision | 745 ns | 93 ns | 1.00x |
| bytesize  u64 only (SI), default 1dp, space | 1.00 us | 125 ns | 1.34x |
| humansize  u64 only, SI, precision=2, no space | 1.35 us | 168 ns | 1.81x |
| byte-unit  u64 (auto unit), format! uses String | 4.48 us | 560 ns | 6.01x |

## Bytes — allocating (`to_string`) — aligned (IEC + space + precision=2), u64 inputs

> This group aligns unit system and spacing. Decimal digit policy can still differ (fixed digits vs trimmed zeros).

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  u64, IEC, precision=2, space (trims zeros) | **540 ns** | **90 ns** | 1.00x |
| byte-unit  u64 only, IEC, fixed 2dp, space | 728 ns | 121 ns | 1.35x |
| bytesize  u64 only, IEC, fixed 2dp, space | 752 ns | 125 ns | 1.39x |
| indicatif HumanBytes  u64 only, IEC, fixed 2dp, space | 761 ns | 127 ns | 1.41x |
| humansize  u64 only, IEC, fixed 2dp, space | 929 ns | 155 ns | 1.72x |
| human-repr  u64, IEC+space (feature) | 1.06 us | 177 ns | 1.96x |

## Bytes — reused buffer (`write!` into `String`), u64 inputs

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| prettier-bytes  u64 only, fixed 2dp, no negatives | **197 ns** | **25 ns** | 0.33x |
| humfmt  i8-u128, any precision | 595 ns | 74 ns | 1.00x |
| bytesize  u64 only (SI), default 1dp, space | 870 ns | 109 ns | 1.46x |
| humansize  u64 only, SI, precision=2, no space | 1.20 us | 150 ns | 2.01x |
| byte-unit  u64 (auto unit), write! + Display | 4.23 us | 528 ns | 7.11x |

## Bytes — reused buffer (`write!` into `String`) — aligned (IEC + space + precision=2), u64 inputs

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  u64, IEC, precision=2, space (trims zeros) | **428 ns** | **71 ns** | 1.00x |
| byte-unit  u64 only, IEC, fixed 2dp, space | 583 ns | 97 ns | 1.36x |
| bytesize  u64 only, IEC, fixed 2dp, space | 618 ns | 103 ns | 1.44x |
| indicatif HumanBytes  u64 only, IEC, fixed 2dp, space | 655 ns | 109 ns | 1.53x |
| humansize  u64 only, IEC, fixed 2dp, space | 831 ns | 139 ns | 1.94x |
| human-repr  u64, IEC+space (feature) | 937 ns | 156 ns | 2.19x |

## Bytes — extended range (u128 > u64::MAX) — humfmt only

> No other benchmarked crate handles values above `u64::MAX`.

| Scenario | Median per-iteration | Time per value |
|---|---:|---:|
| humfmt/u128_extended | 745 ns | 186 ns |

## Bytes — negative values (i64)

> bytesize and prettier-bytes do not participate (unsigned-only). This harness includes humfmt and humansize.

| Scenario | Median per-iteration | Time per value |
|---|---:|---:|
| humfmt/negative_i64 | 338 ns | 84 ns |
| humansize/negative_i64 | 653 ns | 163 ns |

## Numbers — allocating (`to_string`), mixed i64 inputs

> human_format accepts f64 only and always returns an owned `String`. humfmt accepts all integer and float primitives and implements `Display`. numfmt accepts u64/i64/f64 and returns a borrowed `&str` from an internal buffer.

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| numfmt  i64, short scale, precision=2 | **749 ns** | **75 ns** | 0.95x |
| humfmt  i64, precision=1 (default) | 788 ns | 79 ns | 1.00x |
| humfmt  i64, precision=2 | 819 ns | 82 ns | 1.04x |
| human_format  f64 only, precision=2, returns String | 2.50 us | 250 ns | 3.18x |

## Numbers — allocating (`to_string`), u64 inputs (apples-to-apples)

> human_format receives u64 cast to f64. All three crates produce compact `K/M/B` style output.

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| numfmt  u64, short scale, precision=2 | **641 ns** | **80 ns** | 0.99x |
| humfmt  u64, precision=1 | 647 ns | 81 ns | 1.00x |
| humfmt  u64, precision=2 | 670 ns | 84 ns | 1.04x |
| human_format  u64 as f64, precision=2, returns String | 1.71 us | 214 ns | 2.65x |
| human_format  u64 as f64, precision=1, returns String | 1.86 us | 232 ns | 2.87x |

## Numbers — allocating (`to_string`), f64 inputs

> Float path only. human_format accepts f64 natively. human-repr and readable do not produce compact suffixes and are excluded.

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| numfmt  f64, short scale, precision=2 | **628 ns** | **79 ns** | 0.40x |
| human_format  f64, precision=2, returns String | 1.53 us | 191 ns | 0.97x |
| humfmt  f64, precision=2 | 1.58 us | 197 ns | 1.00x |

## Numbers — humfmt option coverage (allocating)

> Humfmt-only group. These rows measure the cost of individual number-formatting options; they are not competitor comparisons.

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  i64, precision=0, rounding=floor | **595 ns** | **60 ns** | 1.00x |
| humfmt  i64, precision=0, rounding=ceil | 612 ns | 61 ns | 1.03x |
| humfmt  i64, force_sign | 778 ns | 78 ns | 1.31x |
| humfmt  i64, custom decimal/group separators | 811 ns | 81 ns | 1.36x |
| humfmt  i64, compact=false + separators | 811 ns | 81 ns | 1.36x |
| humfmt  i64, significant_digits=3 | 892 ns | 89 ns | 1.50x |
| humfmt  i64, long units, precision=2 | 897 ns | 90 ns | 1.51x |
| humfmt  f64, compact=false + separators | 1.85 us | 231 ns | 3.89x |
| humfmt  f64, significant_digits=3 | 2.02 us | 252 ns | 4.23x |

## Numbers — extended range (u128 > u64::MAX) — humfmt only

> Competitor crates in this harness either do not accept u128 inputs or do not cover the full u128/i128 range. This group tracks humfmt's extended integer path.

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  u128 extreme, default | **610 ns** | **102 ns** | 1.00x |
| humfmt  u128 extreme, significant_digits=3 | 687 ns | 115 ns | 1.13x |
| humfmt  u128 extreme, precision=2 | 722 ns | 120 ns | 1.18x |
| humfmt  u128 extreme, significant_digits=6 | 973 ns | 162 ns | 1.60x |
| humfmt  u128 extreme, compact=false | 1.47 us | 245 ns | 2.41x |
| humfmt  u128 extreme, compact=false + separators | 2.38 us | 397 ns | 3.91x |

## Numbers — reused buffer (`write!` into `String`), u64 inputs

> humfmt writes via `Display` with no intermediate allocation. human_format always allocates a `String`; we `push_str` it into the buffer. numfmt returns a `&str` from its internal buffer; we `push_str` it.

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  u64, precision=1, write! | **515 ns** | **64 ns** | 1.00x |
| numfmt  u64, short scale, precision=2, push_str (returns &str) | 524 ns | 66 ns | 1.02x |
| humfmt  u64, precision=2, write! | 543 ns | 68 ns | 1.05x |
| human_format  u64 as f64, precision=2, push_str (always allocs) | 1.72 us | 215 ns | 3.34x |

## Numbers — reused buffer, humfmt option coverage

> Humfmt-only group. These rows measure reused-buffer formatting for option-heavy and extended-range number scenarios.

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  i64, compact=false + separators, write! | 624 ns | **62 ns** | 1.00x |
| humfmt  i64, significant_digits=3, write! | 739 ns | 74 ns | 1.18x |
| humfmt  u128 extreme, default, write! | **522 ns** | 87 ns | 1.39x |
| humfmt  u128 extreme, significant_digits=3, write! | 593 ns | 99 ns | 1.58x |
| humfmt  f64, precision=2, write! | 1.44 us | 180 ns | 2.89x |

## Duration formatting — allocating

> humantime renders all non-zero units. humfmt caps at `max_units` (default 2). These produce different output for the same input.

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  short, 2 units (default) | **717 ns** | **90 ns** | 1.00x |
| humantime  all non-zero units | 875 ns | 109 ns | 1.22x |
| humfmt  short, 3 units | 890 ns | 111 ns | 1.24x |
| humfmt  long labels, 2 units | 979 ns | 122 ns | 1.37x |

## Relative time — allocating

> timeago returns an owned `String` from `convert()`. humfmt implements `Display` and writes directly with no intermediate allocation.

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  short, 2 units (explicit) | **735 ns** | **92 ns** | 1.00x |
| humfmt  short, 2 units (default) | 750 ns | 94 ns | 1.02x |
| humfmt  long, 2 units | 947 ns | 118 ns | 1.29x |
| timeago  1 unit (default), returns String | 1.10 us | 138 ns | 1.50x |
| timeago  2 units, returns String | 1.69 us | 211 ns | 2.30x |


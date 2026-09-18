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
| prettier-bytes  u64 only, fixed 2dp, no negatives | **317 ns** | **40 ns** | 0.45x |
| humfmt  i8-u128, any precision | 711 ns | 89 ns | 1.00x |
| bytesize  u64 only (SI), default 1dp, space | 965 ns | 121 ns | 1.36x |
| humansize  u64 only, SI, precision=2, no space | 1.35 us | 168 ns | 1.89x |
| byte-unit  u64 (auto unit), format! uses String | 4.47 us | 558 ns | 6.29x |

## Bytes — allocating (`to_string`) — aligned (IEC + space + precision=2), u64 inputs

> This group aligns unit system and spacing. Decimal digit policy can still differ (fixed digits vs trimmed zeros).

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  u64, IEC, precision=2, space (trims zeros) | **531 ns** | **89 ns** | 1.00x |
| byte-unit  u64 only, IEC, fixed 2dp, space | 709 ns | 118 ns | 1.34x |
| indicatif HumanBytes  u64 only, IEC, fixed 2dp, space | 742 ns | 124 ns | 1.40x |
| bytesize  u64 only, IEC, fixed 2dp, space | 753 ns | 125 ns | 1.42x |
| humansize  u64 only, IEC, fixed 2dp, space | 922 ns | 154 ns | 1.74x |
| human-repr  u64, IEC+space (feature) | 1.02 us | 170 ns | 1.93x |

## Bytes — reused buffer (`write!` into `String`), u64 inputs

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| prettier-bytes  u64 only, fixed 2dp, no negatives | **201 ns** | **25 ns** | 0.33x |
| humfmt  i8-u128, any precision | 610 ns | 76 ns | 1.00x |
| bytesize  u64 only (SI), default 1dp, space | 872 ns | 109 ns | 1.43x |
| humansize  u64 only, SI, precision=2, no space | 1.22 us | 152 ns | 2.00x |
| byte-unit  u64 (auto unit), write! + Display | 4.18 us | 522 ns | 6.84x |

## Bytes — reused buffer (`write!` into `String`) — aligned (IEC + space + precision=2), u64 inputs

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  u64, IEC, precision=2, space (trims zeros) | **444 ns** | **74 ns** | 1.00x |
| byte-unit  u64 only, IEC, fixed 2dp, space | 586 ns | 98 ns | 1.32x |
| bytesize  u64 only, IEC, fixed 2dp, space | 621 ns | 104 ns | 1.40x |
| indicatif HumanBytes  u64 only, IEC, fixed 2dp, space | 645 ns | 107 ns | 1.45x |
| humansize  u64 only, IEC, fixed 2dp, space | 809 ns | 135 ns | 1.82x |
| human-repr  u64, IEC+space (feature) | 928 ns | 155 ns | 2.09x |

## Bytes — extended range (u128 > u64::MAX) — humfmt only

> No other benchmarked crate handles values above `u64::MAX`.

| Scenario | Median per-iteration | Time per value |
|---|---:|---:|
| humfmt/u128_extended | 724 ns | 181 ns |

## Bytes — negative values (i64)

> bytesize and prettier-bytes do not participate (unsigned-only). This harness includes humfmt and humansize.

| Scenario | Median per-iteration | Time per value |
|---|---:|---:|
| humfmt/negative_i64 | 331 ns | 83 ns |
| humansize/negative_i64 | 629 ns | 157 ns |

## Numbers — allocating (`to_string`), mixed i64 inputs

> human_format accepts f64 only and always returns an owned `String`. humfmt accepts all integer and float primitives and implements `Display`. numfmt accepts u64/i64/f64 and returns a borrowed `&str` from an internal buffer.

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| numfmt  i64, short scale, precision=2 | **736 ns** | **74 ns** | 0.94x |
| humfmt  i64, precision=1 (default) | 782 ns | 78 ns | 1.00x |
| humfmt  i64, precision=2 | 803 ns | 80 ns | 1.03x |
| human_format  f64 only, precision=2, returns String | 2.58 us | 258 ns | 3.30x |

## Numbers — allocating (`to_string`), u64 inputs (apples-to-apples)

> human_format receives u64 cast to f64. All three crates produce compact `K/M/B` style output.

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  u64, precision=1 | **640 ns** | **80 ns** | 1.00x |
| numfmt  u64, short scale, precision=2 | 642 ns | 80 ns | 1.00x |
| humfmt  u64, precision=2 | 667 ns | 83 ns | 1.04x |
| human_format  u64 as f64, precision=2, returns String | 1.80 us | 225 ns | 2.82x |
| human_format  u64 as f64, precision=1, returns String | 1.96 us | 245 ns | 3.06x |

## Numbers — allocating (`to_string`), f64 inputs

> Float path only. human_format accepts f64 natively. human-repr and readable do not produce compact suffixes and are excluded.

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| numfmt  f64, short scale, precision=2 | **654 ns** | **82 ns** | 0.41x |
| humfmt  f64, precision=2 | 1.60 us | 200 ns | 1.00x |
| human_format  f64, precision=2, returns String | 1.66 us | 208 ns | 1.04x |

## Numbers — humfmt option coverage (allocating)

> Humfmt-only group. These rows measure the cost of individual number-formatting options; they are not competitor comparisons.

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  i64, precision=0, rounding=floor | **586 ns** | **59 ns** | 1.00x |
| humfmt  i64, precision=0, rounding=ceil | 599 ns | 60 ns | 1.02x |
| humfmt  i64, force_sign | 771 ns | 77 ns | 1.32x |
| humfmt  i64, custom decimal/group separators | 809 ns | 81 ns | 1.38x |
| humfmt  i64, compact=false + separators | 810 ns | 81 ns | 1.38x |
| humfmt  i64, significant_digits=3 | 875 ns | 88 ns | 1.49x |
| humfmt  i64, long units, precision=2 | 879 ns | 88 ns | 1.50x |
| humfmt  f64, compact=false + separators | 1.79 us | 224 ns | 3.82x |
| humfmt  f64, significant_digits=3 | 1.98 us | 247 ns | 4.23x |

## Numbers — extended range (u128 > u64::MAX) — humfmt only

> Competitor crates in this harness either do not accept u128 inputs or do not cover the full u128/i128 range. This group tracks humfmt's extended integer path.

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  u128 extreme, default | **611 ns** | **102 ns** | 1.00x |
| humfmt  u128 extreme, precision=2 | 682 ns | 114 ns | 1.12x |
| humfmt  u128 extreme, significant_digits=3 | 682 ns | 114 ns | 1.12x |
| humfmt  u128 extreme, significant_digits=6 | 974 ns | 162 ns | 1.59x |
| humfmt  u128 extreme, compact=false | 1.48 us | 247 ns | 2.42x |
| humfmt  u128 extreme, compact=false + separators | 2.41 us | 401 ns | 3.94x |

## Numbers — reused buffer (`write!` into `String`), u64 inputs

> humfmt writes via `Display` with no intermediate allocation. human_format always allocates a `String`; we `push_str` it into the buffer. numfmt returns a `&str` from its internal buffer; we `push_str` it.

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  u64, precision=1, write! | **508 ns** | **64 ns** | 1.00x |
| numfmt  u64, short scale, precision=2, push_str (returns &str) | 525 ns | 66 ns | 1.03x |
| humfmt  u64, precision=2, write! | 541 ns | 68 ns | 1.06x |
| human_format  u64 as f64, precision=2, push_str (always allocs) | 1.85 us | 232 ns | 3.65x |

## Numbers — reused buffer, humfmt option coverage

> Humfmt-only group. These rows measure reused-buffer formatting for option-heavy and extended-range number scenarios.

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  i64, compact=false + separators, write! | 632 ns | **63 ns** | 1.00x |
| humfmt  i64, significant_digits=3, write! | 732 ns | 73 ns | 1.16x |
| humfmt  u128 extreme, default, write! | **524 ns** | 87 ns | 1.38x |
| humfmt  u128 extreme, significant_digits=3, write! | 592 ns | 99 ns | 1.56x |
| humfmt  f64, precision=2, write! | 1.43 us | 179 ns | 2.83x |

## Duration formatting — allocating

> humantime renders all non-zero units. humfmt caps at `max_units` (default 2). These produce different output for the same input.

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  short, 2 units (default) | **695 ns** | **87 ns** | 1.00x |
| humfmt  short, 3 units | 877 ns | 110 ns | 1.26x |
| humantime  all non-zero units | 885 ns | 111 ns | 1.27x |
| humfmt  long labels, 2 units | 966 ns | 121 ns | 1.39x |

## Relative time — allocating

> timeago returns an owned `String` from `convert()`. humfmt implements `Display` and writes directly with no intermediate allocation.

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  short, 2 units (explicit) | **716 ns** | **90 ns** | 1.00x |
| humfmt  short, 2 units (default) | 734 ns | 92 ns | 1.02x |
| humfmt  long, 2 units | 941 ns | 118 ns | 1.31x |
| timeago  1 unit (default), returns String | 1.05 us | 131 ns | 1.46x |
| timeago  2 units, returns String | 1.68 us | 210 ns | 2.35x |


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
| CPU | AMD EPYC 9V74 80-Core Processor |
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
| prettier-bytes  u64 only, fixed 2dp, no negatives | **278 ns** | **35 ns** | 0.51x |
| humfmt  i8-u128, any precision | 545 ns | 68 ns | 1.00x |
| bytesize  u64 only (SI), default 1dp, space | 802 ns | 100 ns | 1.47x |
| humansize  u64 only, SI, precision=2, no space | 1.15 us | 144 ns | 2.11x |
| byte-unit  u64 (auto unit), format! uses String | 3.72 us | 465 ns | 6.83x |

## Bytes — allocating (`to_string`) — aligned (IEC + space + precision=2), u64 inputs

> This group aligns unit system and spacing. Decimal digit policy can still differ (fixed digits vs trimmed zeros).

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  u64, IEC, precision=2, space (trims zeros) | **409 ns** | **68 ns** | 1.00x |
| byte-unit  u64 only, IEC, fixed 2dp, space | 580 ns | 97 ns | 1.42x |
| indicatif HumanBytes  u64 only, IEC, fixed 2dp, space | 624 ns | 104 ns | 1.53x |
| bytesize  u64 only, IEC, fixed 2dp, space | 629 ns | 105 ns | 1.54x |
| humansize  u64 only, IEC, fixed 2dp, space | 752 ns | 125 ns | 1.84x |
| human-repr  u64, IEC+space (feature) | 850 ns | 142 ns | 2.08x |

## Bytes — reused buffer (`write!` into `String`), u64 inputs

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| prettier-bytes  u64 only, fixed 2dp, no negatives | **146 ns** | **18 ns** | 0.31x |
| humfmt  i8-u128, any precision | 471 ns | 59 ns | 1.00x |
| bytesize  u64 only (SI), default 1dp, space | 714 ns | 89 ns | 1.52x |
| humansize  u64 only, SI, precision=2, no space | 971 ns | 121 ns | 2.06x |
| byte-unit  u64 (auto unit), write! + Display | 3.51 us | 439 ns | 7.46x |

## Bytes — reused buffer (`write!` into `String`) — aligned (IEC + space + precision=2), u64 inputs

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  u64, IEC, precision=2, space (trims zeros) | **342 ns** | **57 ns** | 1.00x |
| byte-unit  u64 only, IEC, fixed 2dp, space | 431 ns | 72 ns | 1.26x |
| bytesize  u64 only, IEC, fixed 2dp, space | 487 ns | 81 ns | 1.42x |
| indicatif HumanBytes  u64 only, IEC, fixed 2dp, space | 530 ns | 88 ns | 1.55x |
| humansize  u64 only, IEC, fixed 2dp, space | 651 ns | 109 ns | 1.91x |
| human-repr  u64, IEC+space (feature) | 790 ns | 132 ns | 2.31x |

## Bytes — extended range (u128 > u64::MAX) — humfmt only

> No other benchmarked crate handles values above `u64::MAX`.

| Scenario | Median per-iteration | Time per value |
|---|---:|---:|
| humfmt/u128_extended | 597 ns | 149 ns |

## Bytes — negative values (i64)

> bytesize and prettier-bytes do not participate (unsigned-only). This harness includes humfmt and humansize.

| Scenario | Median per-iteration | Time per value |
|---|---:|---:|
| humfmt/negative_i64 | 255 ns | 64 ns |
| humansize/negative_i64 | 528 ns | 132 ns |

## Numbers — allocating (`to_string`), mixed i64 inputs

> human_format accepts f64 only and always returns an owned `String`. humfmt accepts all integer and float primitives and implements `Display`. numfmt accepts u64/i64/f64 and returns a borrowed `&str` from an internal buffer.

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| numfmt  i64, short scale, precision=2 | **592 ns** | **59 ns** | 0.98x |
| humfmt  i64, precision=1 (default) | 606 ns | 61 ns | 1.00x |
| humfmt  i64, precision=2 | 652 ns | 65 ns | 1.08x |
| human_format  f64 only, precision=2, returns String | 1.92 us | 192 ns | 3.17x |

## Numbers — allocating (`to_string`), u64 inputs (apples-to-apples)

> human_format receives u64 cast to f64. All three crates produce compact `K/M/B` style output.

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  u64, precision=1 | **495 ns** | **62 ns** | 1.00x |
| humfmt  u64, precision=2 | 523 ns | 65 ns | 1.06x |
| numfmt  u64, short scale, precision=2 | 524 ns | 66 ns | 1.06x |
| human_format  u64 as f64, precision=2, returns String | 1.34 us | 167 ns | 2.71x |
| human_format  u64 as f64, precision=1, returns String | 1.45 us | 181 ns | 2.94x |

## Numbers — allocating (`to_string`), f64 inputs

> Float path only. human_format accepts f64 natively. human-repr and readable do not produce compact suffixes and are excluded.

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| numfmt  f64, short scale, precision=2 | **492 ns** | **62 ns** | 0.38x |
| human_format  f64, precision=2, returns String | 1.16 us | 145 ns | 0.89x |
| humfmt  f64, precision=2 | 1.30 us | 162 ns | 1.00x |

## Numbers — humfmt option coverage (allocating)

> Humfmt-only group. These rows measure the cost of individual number-formatting options; they are not competitor comparisons.

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  i64, precision=0, rounding=floor | **477 ns** | **48 ns** | 1.00x |
| humfmt  i64, precision=0, rounding=ceil | 497 ns | 50 ns | 1.04x |
| humfmt  i64, force_sign | 621 ns | 62 ns | 1.30x |
| humfmt  i64, compact=false + separators | 650 ns | 65 ns | 1.36x |
| humfmt  i64, custom decimal/group separators | 650 ns | 65 ns | 1.36x |
| humfmt  i64, long units, precision=2 | 711 ns | 71 ns | 1.49x |
| humfmt  i64, significant_digits=3 | 712 ns | 71 ns | 1.49x |
| humfmt  f64, compact=false + separators | 1.47 us | 184 ns | 3.86x |
| humfmt  f64, significant_digits=3 | 1.63 us | 204 ns | 4.27x |

## Numbers — extended range (u128 > u64::MAX) — humfmt only

> Competitor crates in this harness either do not accept u128 inputs or do not cover the full u128/i128 range. This group tracks humfmt's extended integer path.

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  u128 extreme, default | **474 ns** | **79 ns** | 1.00x |
| humfmt  u128 extreme, precision=2 | 534 ns | 89 ns | 1.13x |
| humfmt  u128 extreme, significant_digits=3 | 549 ns | 91 ns | 1.16x |
| humfmt  u128 extreme, significant_digits=6 | 789 ns | 132 ns | 1.66x |
| humfmt  u128 extreme, compact=false | 1.26 us | 210 ns | 2.65x |
| humfmt  u128 extreme, compact=false + separators | 1.98 us | 330 ns | 4.17x |

## Numbers — reused buffer (`write!` into `String`), u64 inputs

> humfmt writes via `Display` with no intermediate allocation. human_format always allocates a `String`; we `push_str` it into the buffer. numfmt returns a `&str` from its internal buffer; we `push_str` it.

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  u64, precision=1, write! | **398 ns** | **50 ns** | 1.00x |
| humfmt  u64, precision=2, write! | 432 ns | 54 ns | 1.09x |
| numfmt  u64, short scale, precision=2, push_str (returns &str) | 438 ns | 55 ns | 1.10x |
| human_format  u64 as f64, precision=2, push_str (always allocs) | 1.35 us | 168 ns | 3.38x |

## Numbers — reused buffer, humfmt option coverage

> Humfmt-only group. These rows measure reused-buffer formatting for option-heavy and extended-range number scenarios.

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  i64, compact=false + separators, write! | 497 ns | **50 ns** | 1.00x |
| humfmt  i64, significant_digits=3, write! | 602 ns | 60 ns | 1.21x |
| humfmt  u128 extreme, default, write! | **424 ns** | 71 ns | 1.42x |
| humfmt  u128 extreme, significant_digits=3, write! | 485 ns | 81 ns | 1.63x |
| humfmt  f64, precision=2, write! | 1.20 us | 150 ns | 3.01x |

## Duration formatting — allocating

> humantime renders all non-zero units. humfmt caps at `max_units` (default 2). These produce different output for the same input.

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  short, 2 units (default) | **534 ns** | **67 ns** | 1.00x |
| humantime  all non-zero units | 644 ns | 81 ns | 1.21x |
| humfmt  short, 3 units | 701 ns | 88 ns | 1.31x |
| humfmt  long labels, 2 units | 762 ns | 95 ns | 1.43x |

## Relative time — allocating

> timeago returns an owned `String` from `convert()`. humfmt implements `Display` and writes directly with no intermediate allocation.

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  short, 2 units (default) | **550 ns** | **69 ns** | 1.00x |
| humfmt  short, 2 units (explicit) | 551 ns | 69 ns | 1.00x |
| humfmt  long, 2 units | 701 ns | 88 ns | 1.28x |
| timeago  1 unit (default), returns String | 838 ns | 105 ns | 1.52x |
| timeago  2 units, returns String | 1.26 us | 158 ns | 2.30x |


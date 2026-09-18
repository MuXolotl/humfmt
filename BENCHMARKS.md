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
| CPU | AMD EPYC 9V45 96-Core Processor |
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
| prettier-bytes  u64 only, fixed 2dp, no negatives | **212 ns** | **26 ns** | 0.49x |
| humfmt  i8-u128, any precision | 429 ns | 54 ns | 1.00x |
| bytesize  u64 only (SI), default 1dp, space | 618 ns | 77 ns | 1.44x |
| humansize  u64 only, SI, precision=2, no space | 870 ns | 109 ns | 2.03x |
| byte-unit  u64 (auto unit), format! uses String | 2.80 us | 349 ns | 6.52x |

## Bytes — allocating (`to_string`) — aligned (IEC + space + precision=2), u64 inputs

> This group aligns unit system and spacing. Decimal digit policy can still differ (fixed digits vs trimmed zeros).

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  u64, IEC, precision=2, space (trims zeros) | **286 ns** | **48 ns** | 1.00x |
| byte-unit  u64 only, IEC, fixed 2dp, space | 408 ns | 68 ns | 1.43x |
| indicatif HumanBytes  u64 only, IEC, fixed 2dp, space | 440 ns | 73 ns | 1.54x |
| bytesize  u64 only, IEC, fixed 2dp, space | 486 ns | 81 ns | 1.70x |
| humansize  u64 only, IEC, fixed 2dp, space | 576 ns | 96 ns | 2.02x |
| human-repr  u64, IEC+space (feature) | 663 ns | 111 ns | 2.32x |

## Bytes — reused buffer (`write!` into `String`), u64 inputs

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| prettier-bytes  u64 only, fixed 2dp, no negatives | **113 ns** | **14 ns** | 0.33x |
| humfmt  i8-u128, any precision | 346 ns | 43 ns | 1.00x |
| bytesize  u64 only (SI), default 1dp, space | 543 ns | 68 ns | 1.57x |
| humansize  u64 only, SI, precision=2, no space | 787 ns | 98 ns | 2.28x |
| byte-unit  u64 (auto unit), write! + Display | 2.63 us | 329 ns | 7.61x |

## Bytes — reused buffer (`write!` into `String`) — aligned (IEC + space + precision=2), u64 inputs

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  u64, IEC, precision=2, space (trims zeros) | **243 ns** | **40 ns** | 1.00x |
| byte-unit  u64 only, IEC, fixed 2dp, space | 327 ns | 54 ns | 1.35x |
| bytesize  u64 only, IEC, fixed 2dp, space | 388 ns | 65 ns | 1.60x |
| indicatif HumanBytes  u64 only, IEC, fixed 2dp, space | 423 ns | 71 ns | 1.74x |
| humansize  u64 only, IEC, fixed 2dp, space | 525 ns | 88 ns | 2.16x |
| human-repr  u64, IEC+space (feature) | 624 ns | 104 ns | 2.57x |

## Bytes — extended range (u128 > u64::MAX) — humfmt only

> No other benchmarked crate handles values above `u64::MAX`.

| Scenario | Median per-iteration | Time per value |
|---|---:|---:|
| humfmt/u128_extended | 486 ns | 122 ns |

## Bytes — negative values (i64)

> bytesize and prettier-bytes do not participate (unsigned-only). This harness includes humfmt and humansize.

| Scenario | Median per-iteration | Time per value |
|---|---:|---:|
| humfmt/negative_i64 | 189 ns | 47 ns |
| humansize/negative_i64 | 410 ns | 103 ns |

## Numbers — allocating (`to_string`), mixed i64 inputs

> human_format accepts f64 only and always returns an owned `String`. humfmt accepts all integer and float primitives and implements `Display`. numfmt accepts u64/i64/f64 and returns a borrowed `&str` from an internal buffer.

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  i64, precision=1 (default) | **441 ns** | **44 ns** | 1.00x |
| humfmt  i64, precision=2 | 443 ns | 44 ns | 1.00x |
| numfmt  i64, short scale, precision=2 | 445 ns | 44 ns | 1.01x |
| human_format  f64 only, precision=2, returns String | 1.61 us | 161 ns | 3.66x |

## Numbers — allocating (`to_string`), u64 inputs (apples-to-apples)

> human_format receives u64 cast to f64. All three crates produce compact `K/M/B` style output.

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  u64, precision=1 | **370 ns** | **46 ns** | 1.00x |
| numfmt  u64, short scale, precision=2 | 379 ns | 47 ns | 1.02x |
| humfmt  u64, precision=2 | 389 ns | 49 ns | 1.05x |
| human_format  u64 as f64, precision=2, returns String | 1.08 us | 135 ns | 2.92x |
| human_format  u64 as f64, precision=1, returns String | 1.21 us | 152 ns | 3.28x |

## Numbers — allocating (`to_string`), f64 inputs

> Float path only. human_format accepts f64 natively. human-repr and readable do not produce compact suffixes and are excluded.

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| numfmt  f64, short scale, precision=2 | **367 ns** | **46 ns** | 0.34x |
| human_format  f64, precision=2, returns String | 939 ns | 117 ns | 0.88x |
| humfmt  f64, precision=2 | 1.07 us | 134 ns | 1.00x |

## Numbers — humfmt option coverage (allocating)

> Humfmt-only group. These rows measure the cost of individual number-formatting options; they are not competitor comparisons.

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  i64, precision=0, rounding=floor | **352 ns** | **35 ns** | 1.00x |
| humfmt  i64, precision=0, rounding=ceil | 352 ns | 35 ns | 1.00x |
| humfmt  i64, force_sign | 442 ns | 44 ns | 1.26x |
| humfmt  i64, compact=false + separators | 484 ns | 48 ns | 1.38x |
| humfmt  i64, custom decimal/group separators | 486 ns | 49 ns | 1.38x |
| humfmt  i64, long units, precision=2 | 498 ns | 50 ns | 1.42x |
| humfmt  i64, significant_digits=3 | 508 ns | 51 ns | 1.45x |
| humfmt  f64, compact=false + separators | 1.26 us | 158 ns | 4.49x |
| humfmt  f64, significant_digits=3 | 1.29 us | 162 ns | 4.59x |

## Numbers — extended range (u128 > u64::MAX) — humfmt only

> Competitor crates in this harness either do not accept u128 inputs or do not cover the full u128/i128 range. This group tracks humfmt's extended integer path.

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  u128 extreme, default | **375 ns** | **63 ns** | 1.00x |
| humfmt  u128 extreme, significant_digits=3 | 414 ns | 69 ns | 1.10x |
| humfmt  u128 extreme, precision=2 | 420 ns | 70 ns | 1.12x |
| humfmt  u128 extreme, significant_digits=6 | 607 ns | 101 ns | 1.62x |
| humfmt  u128 extreme, compact=false | 959 ns | 160 ns | 2.56x |
| humfmt  u128 extreme, compact=false + separators | 1.48 us | 247 ns | 3.95x |

## Numbers — reused buffer (`write!` into `String`), u64 inputs

> humfmt writes via `Display` with no intermediate allocation. human_format always allocates a `String`; we `push_str` it into the buffer. numfmt returns a `&str` from its internal buffer; we `push_str` it.

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  u64, precision=1, write! | **287 ns** | **36 ns** | 1.00x |
| humfmt  u64, precision=2, write! | 299 ns | 37 ns | 1.04x |
| numfmt  u64, short scale, precision=2, push_str (returns &str) | 320 ns | 40 ns | 1.11x |
| human_format  u64 as f64, precision=2, push_str (always allocs) | 1.06 us | 132 ns | 3.68x |

## Numbers — reused buffer, humfmt option coverage

> Humfmt-only group. These rows measure reused-buffer formatting for option-heavy and extended-range number scenarios.

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  i64, compact=false + separators, write! | 381 ns | **38 ns** | 1.00x |
| humfmt  i64, significant_digits=3, write! | 409 ns | 41 ns | 1.07x |
| humfmt  u128 extreme, default, write! | **284 ns** | 47 ns | 1.24x |
| humfmt  u128 extreme, significant_digits=3, write! | 341 ns | 57 ns | 1.49x |
| humfmt  f64, precision=2, write! | 897 ns | 112 ns | 2.94x |

## Duration formatting — allocating

> humantime renders all non-zero units. humfmt caps at `max_units` (default 2). These produce different output for the same input.

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  short, 2 units (default) | **375 ns** | **47 ns** | 1.00x |
| humantime  all non-zero units | 444 ns | 55 ns | 1.18x |
| humfmt  short, 3 units | 451 ns | 56 ns | 1.20x |
| humfmt  long labels, 2 units | 551 ns | 69 ns | 1.47x |

## Relative time — allocating

> timeago returns an owned `String` from `convert()`. humfmt implements `Display` and writes directly with no intermediate allocation.

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  short, 2 units (default) | **361 ns** | **45 ns** | 1.00x |
| humfmt  short, 2 units (explicit) | 386 ns | 48 ns | 1.07x |
| timeago  1 unit (default), returns String | 547 ns | 68 ns | 1.51x |
| humfmt  long, 2 units | 553 ns | 69 ns | 1.53x |
| timeago  2 units, returns String | 959 ns | 120 ns | 2.66x |


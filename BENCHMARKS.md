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
| CPU | AMD64 Family 25 Model 33 Stepping 2, AuthenticAMD |
| Cores available | 12 |
| OS | windows x86_64 |
| Rust | rustc 1.97.1 (8bab26f4f 2026-07-14) |
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
| prettier-bytes  u64 only, fixed 2dp, no negatives | **456 ns** | **57 ns** | 0.60x |
| humfmt  i8-u128, any precision | 755 ns | 94 ns | 1.00x |
| bytesize  u64 only (SI), default 1dp, space | 980 ns | 122 ns | 1.30x |
| humansize  u64 only, SI, precision=2, no space | 1.20 us | 150 ns | 1.59x |
| byte-unit  u64 (auto unit), format! uses String | 3.49 us | 436 ns | 4.62x |

## Bytes — allocating (`to_string`) — aligned (IEC + space + precision=2), u64 inputs

> This group aligns unit system and spacing. Decimal digit policy can still differ (fixed digits vs trimmed zeros).

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  u64, IEC, precision=2, space (trims zeros) | **537 ns** | **90 ns** | 1.00x |
| byte-unit  u64 only, IEC, fixed 2dp, space | 684 ns | 114 ns | 1.27x |
| bytesize  u64 only, IEC, fixed 2dp, space | 717 ns | 120 ns | 1.34x |
| indicatif HumanBytes  u64 only, IEC, fixed 2dp, space | 718 ns | 120 ns | 1.34x |
| humansize  u64 only, IEC, fixed 2dp, space | 827 ns | 138 ns | 1.54x |
| human-repr  u64, IEC+space (feature) | 1.23 us | 204 ns | 2.28x |

## Bytes — reused buffer (`write!` into `String`), u64 inputs

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| prettier-bytes  u64 only, fixed 2dp, no negatives | **135 ns** | **17 ns** | 0.30x |
| humfmt  i8-u128, any precision | 444 ns | 56 ns | 1.00x |
| bytesize  u64 only (SI), default 1dp, space | 633 ns | 79 ns | 1.43x |
| humansize  u64 only, SI, precision=2, no space | 849 ns | 106 ns | 1.91x |
| byte-unit  u64 (auto unit), write! + Display | 2.95 us | 369 ns | 6.65x |

## Bytes — reused buffer (`write!` into `String`) — aligned (IEC + space + precision=2), u64 inputs

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  u64, IEC, precision=2, space (trims zeros) | **313 ns** | **52 ns** | 1.00x |
| byte-unit  u64 only, IEC, fixed 2dp, space | 410 ns | 68 ns | 1.31x |
| bytesize  u64 only, IEC, fixed 2dp, space | 446 ns | 74 ns | 1.43x |
| indicatif HumanBytes  u64 only, IEC, fixed 2dp, space | 454 ns | 76 ns | 1.45x |
| humansize  u64 only, IEC, fixed 2dp, space | 590 ns | 98 ns | 1.88x |
| human-repr  u64, IEC+space (feature) | 936 ns | 156 ns | 2.99x |

## Bytes — extended range (u128 > u64::MAX) — humfmt only

> No other benchmarked crate handles values above `u64::MAX`.

| Scenario | Median per-iteration | Time per value |
|---|---:|---:|
| humfmt/u128_extended | 708 ns | 177 ns |

## Bytes — negative values (i64)

> bytesize and prettier-bytes do not participate (unsigned-only). This harness includes humfmt and humansize.

| Scenario | Median per-iteration | Time per value |
|---|---:|---:|
| humfmt/negative_i64 | 380 ns | 95 ns |
| humansize/negative_i64 | 581 ns | 145 ns |

## Numbers — allocating (`to_string`), mixed i64 inputs

> human_format accepts f64 only and always returns an owned `String`. humfmt accepts all integer and float primitives and implements `Display`. numfmt accepts u64/i64/f64 and returns a borrowed `&str` from an internal buffer.

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  i64, precision=1 (default) | **878 ns** | **88 ns** | 1.00x |
| humfmt  i64, precision=2 | 924 ns | 92 ns | 1.05x |
| numfmt  i64, short scale, precision=2 | 1.01 us | 101 ns | 1.15x |
| human_format  f64 only, precision=2, returns String | 2.49 us | 249 ns | 2.83x |

## Numbers — allocating (`to_string`), u64 inputs (apples-to-apples)

> human_format receives u64 cast to f64. All three crates produce compact `K/M/B` style output.

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  u64, precision=1 | **738 ns** | **92 ns** | 1.00x |
| humfmt  u64, precision=2 | 748 ns | 94 ns | 1.01x |
| numfmt  u64, short scale, precision=2 | 800 ns | 100 ns | 1.08x |
| human_format  u64 as f64, precision=2, returns String | 1.76 us | 220 ns | 2.38x |
| human_format  u64 as f64, precision=1, returns String | 1.84 us | 229 ns | 2.49x |

## Numbers — allocating (`to_string`), f64 inputs

> Float path only. human_format accepts f64 natively. human-repr and readable do not produce compact suffixes and are excluded.

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| numfmt  f64, short scale, precision=2 | **812 ns** | **102 ns** | 0.62x |
| humfmt  f64, precision=2 | 1.32 us | 165 ns | 1.00x |
| human_format  f64, precision=2, returns String | 1.67 us | 208 ns | 1.26x |

## Numbers — humfmt option coverage (allocating)

> Humfmt-only group. These rows measure the cost of individual number-formatting options; they are not competitor comparisons.

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  i64, precision=0, rounding=floor | **789 ns** | **79 ns** | 1.00x |
| humfmt  i64, precision=0, rounding=ceil | 804 ns | 80 ns | 1.02x |
| humfmt  i64, force_sign | 913 ns | 91 ns | 1.16x |
| humfmt  i64, custom decimal/group separators | 1.05 us | 105 ns | 1.33x |
| humfmt  i64, significant_digits=3 | 1.05 us | 105 ns | 1.33x |
| humfmt  i64, compact=false + separators | 1.05 us | 105 ns | 1.33x |
| humfmt  i64, long units, precision=2 | 1.24 us | 124 ns | 1.57x |
| humfmt  f64, significant_digits=3 | 1.67 us | 208 ns | 2.64x |
| humfmt  f64, compact=false + separators | 1.70 us | 213 ns | 2.70x |

## Numbers — extended range (u128 > u64::MAX) — humfmt only

> Competitor crates in this harness either do not accept u128 inputs or do not cover the full u128/i128 range. This group tracks humfmt's extended integer path.

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  u128 extreme, default | **667 ns** | **111 ns** | 1.00x |
| humfmt  u128 extreme, precision=2 | 734 ns | 122 ns | 1.10x |
| humfmt  u128 extreme, significant_digits=3 | 740 ns | 123 ns | 1.11x |
| humfmt  u128 extreme, significant_digits=6 | 1.12 us | 186 ns | 1.68x |
| humfmt  u128 extreme, compact=false | 1.26 us | 211 ns | 1.90x |
| humfmt  u128 extreme, compact=false + separators | 2.40 us | 401 ns | 3.61x |

## Numbers — reused buffer (`write!` into `String`), u64 inputs

> humfmt writes via `Display` with no intermediate allocation. human_format always allocates a `String`; we `push_str` it into the buffer. numfmt returns a `&str` from its internal buffer; we `push_str` it.

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  u64, precision=1, write! | **440 ns** | **55 ns** | 1.00x |
| humfmt  u64, precision=2, write! | 440 ns | 55 ns | 1.00x |
| numfmt  u64, short scale, precision=2, push_str (returns &str) | 508 ns | 64 ns | 1.16x |
| human_format  u64 as f64, precision=2, push_str (always allocs) | 1.79 us | 224 ns | 4.07x |

## Numbers — reused buffer, humfmt option coverage

> Humfmt-only group. These rows measure reused-buffer formatting for option-heavy and extended-range number scenarios.

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  i64, compact=false + separators, write! | 496 ns | **50 ns** | 1.00x |
| humfmt  i64, significant_digits=3, write! | 670 ns | 67 ns | 1.35x |
| humfmt  u128 extreme, default, write! | **441 ns** | 74 ns | 1.48x |
| humfmt  u128 extreme, significant_digits=3, write! | 500 ns | 83 ns | 1.68x |
| humfmt  f64, precision=2, write! | 1.00 us | 126 ns | 2.53x |

## Duration formatting — allocating

> humantime renders all non-zero units. humfmt caps at `max_units` (default 2). These produce different output for the same input.

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  short, 2 units (default) | **753 ns** | **94 ns** | 1.00x |
| humfmt  short, 3 units | 930 ns | 116 ns | 1.24x |
| humantime  all non-zero units | 970 ns | 121 ns | 1.29x |
| humfmt  long labels, 2 units | 1.37 us | 172 ns | 1.82x |

## Relative time — allocating

> timeago returns an owned `String` from `convert()`. humfmt implements `Display` and writes directly with no intermediate allocation.

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  short, 2 units (explicit) | **854 ns** | **107 ns** | 1.00x |
| humfmt  short, 2 units (default) | 855 ns | 107 ns | 1.00x |
| humfmt  long, 2 units | 1.28 us | 160 ns | 1.50x |
| timeago  1 unit (default), returns String | 1.52 us | 190 ns | 1.78x |
| timeago  2 units, returns String | 2.19 us | 274 ns | 2.56x |


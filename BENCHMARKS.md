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
| prettier-bytes  u64 only, fixed 2dp, no negatives | **995 ns** | **124 ns** | 0.53x |
| humfmt  i8-u128, any precision | 1.87 us | 234 ns | 1.00x |
| bytesize  u64 only (SI), default 1dp, space | 2.13 us | 266 ns | 1.14x |
| humansize  u64 only, SI, precision=2, no space | 2.82 us | 352 ns | 1.50x |
| byte-unit  u64 (auto unit), format! uses String | 7.51 us | 939 ns | 4.01x |

## Bytes — allocating (`to_string`) — aligned (IEC + space + precision=2), u64 inputs

> This group aligns unit system and spacing. Decimal digit policy can still differ (fixed digits vs trimmed zeros).

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  u64, IEC, precision=2, space (trims zeros) | **1.27 us** | **212 ns** | 1.00x |
| byte-unit  u64 only, IEC, fixed 2dp, space | 1.50 us | 250 ns | 1.18x |
| indicatif HumanBytes  u64 only, IEC, fixed 2dp, space | 1.70 us | 284 ns | 1.34x |
| bytesize  u64 only, IEC, fixed 2dp, space | 1.72 us | 287 ns | 1.36x |
| humansize  u64 only, IEC, fixed 2dp, space | 1.96 us | 327 ns | 1.55x |
| human-repr  u64, IEC+space (feature) | 2.39 us | 398 ns | 1.88x |

## Bytes — reused buffer (`write!` into `String`), u64 inputs

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| prettier-bytes  u64 only, fixed 2dp, no negatives | **372 ns** | **47 ns** | 0.33x |
| humfmt  i8-u128, any precision | 1.14 us | 143 ns | 1.00x |
| bytesize  u64 only (SI), default 1dp, space | 1.36 us | 170 ns | 1.19x |
| humansize  u64 only, SI, precision=2, no space | 2.10 us | 263 ns | 1.84x |
| byte-unit  u64 (auto unit), write! + Display | 6.49 us | 811 ns | 5.69x |

## Bytes — reused buffer (`write!` into `String`) — aligned (IEC + space + precision=2), u64 inputs

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  u64, IEC, precision=2, space (trims zeros) | **781 ns** | **130 ns** | 1.00x |
| byte-unit  u64 only, IEC, fixed 2dp, space | 1.02 us | 170 ns | 1.31x |
| indicatif HumanBytes  u64 only, IEC, fixed 2dp, space | 1.08 us | 180 ns | 1.38x |
| bytesize  u64 only, IEC, fixed 2dp, space | 1.12 us | 187 ns | 1.43x |
| humansize  u64 only, IEC, fixed 2dp, space | 1.40 us | 233 ns | 1.79x |
| human-repr  u64, IEC+space (feature) | 1.91 us | 318 ns | 2.44x |

## Bytes — extended range (u128 > u64::MAX) — humfmt only

> No other benchmarked crate handles values above `u64::MAX`.

| Scenario | Median per-iteration | Time per value |
|---|---:|---:|
| humfmt/u128_extended | 1.61 us | 402 ns |

## Bytes — negative values (i64)

> bytesize and prettier-bytes do not participate (unsigned-only). This harness includes humfmt and humansize.

| Scenario | Median per-iteration | Time per value |
|---|---:|---:|
| humfmt/negative_i64 | 877 ns | 219 ns |
| humansize/negative_i64 | 1.39 us | 346 ns |

## Numbers — allocating (`to_string`), mixed i64 inputs

> human_format accepts f64 only and always returns an owned `String`. humfmt accepts all integer and float primitives and implements `Display`. numfmt accepts u64/i64/f64 and returns a borrowed `&str` from an internal buffer.

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  i64, precision=1 (default) | **2.13 us** | **213 ns** | 1.00x |
| numfmt  i64, short scale, precision=2 | 2.20 us | 220 ns | 1.04x |
| humfmt  i64, precision=2 | 2.32 us | 232 ns | 1.09x |
| human_format  f64 only, precision=2, returns String | 5.42 us | 542 ns | 2.55x |

## Numbers — allocating (`to_string`), u64 inputs (apples-to-apples)

> human_format receives u64 cast to f64. All three crates produce compact `K/M/B` style output.

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  u64, precision=1 | **1.72 us** | **215 ns** | 1.00x |
| humfmt  u64, precision=2 | 1.83 us | 228 ns | 1.06x |
| numfmt  u64, short scale, precision=2 | 1.87 us | 233 ns | 1.08x |
| human_format  u64 as f64, precision=2, returns String | 3.83 us | 479 ns | 2.23x |
| human_format  u64 as f64, precision=1, returns String | 3.93 us | 491 ns | 2.28x |

## Numbers — allocating (`to_string`), f64 inputs

> Float path only. human_format accepts f64 natively. human-repr and readable do not produce compact suffixes and are excluded.

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| numfmt  f64, short scale, precision=2 | **1.87 us** | **234 ns** | 0.60x |
| humfmt  f64, precision=2 | 3.10 us | 388 ns | 1.00x |
| human_format  f64, precision=2, returns String | 3.71 us | 463 ns | 1.19x |

## Numbers — reused buffer (`write!` into `String`), u64 inputs

> humfmt writes via `Display` with no intermediate allocation. human_format always allocates a `String`; we `push_str` it into the buffer. numfmt returns a `&str` from its internal buffer; we `push_str` it.

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  u64, precision=1, write! | **1.04 us** | **130 ns** | 1.00x |
| numfmt  u64, short scale, precision=2, push_str (returns &str) | 1.18 us | 148 ns | 1.14x |
| humfmt  u64, precision=2, write! | 1.20 us | 150 ns | 1.15x |
| human_format  u64 as f64, precision=2, push_str (always allocs) | 3.94 us | 492 ns | 3.79x |

## Duration formatting — allocating

> humantime renders all non-zero units. humfmt caps at `max_units` (default 2). These produce different output for the same input.

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  short, 2 units (default) | **1.83 us** | **229 ns** | 1.00x |
| humfmt  short, 3 units | 2.23 us | 279 ns | 1.22x |
| humantime  all non-zero units | 3.43 us | 429 ns | 1.87x |
| humfmt  long labels, 2 units | 3.52 us | 440 ns | 1.92x |

## Relative time — allocating

> timeago returns an owned `String` from `convert()`. humfmt implements `Display` and writes directly with no intermediate allocation.

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  short, 2 units (default) | **2.15 us** | **269 ns** | 1.00x |
| humfmt  short, 2 units (explicit) | 2.23 us | 278 ns | 1.03x |
| humfmt  long, 2 units | 3.03 us | 379 ns | 1.41x |
| timeago  2 units, returns String | 5.28 us | 659 ns | 2.45x |
| timeago  1 unit (default), returns String | 6.31 us | 789 ns | 2.93x |


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
| prettier-bytes  u64 only, fixed 2dp, no negatives | **1.08 us** | **135 ns** | 0.58x |
| humfmt  i8-u128, any precision | 1.87 us | 234 ns | 1.00x |
| bytesize  u64 only (SI), default 1dp, space | 2.25 us | 281 ns | 1.20x |
| humansize  u64 only, SI, precision=2, no space | 2.88 us | 360 ns | 1.54x |
| byte-unit  u64 (auto unit), format! uses String | 8.28 us | 1.03 us | 4.43x |

## Bytes — allocating (`to_string`) — aligned (IEC + space + precision=2), u64 inputs

> This group aligns unit system and spacing. Decimal digit policy can still differ (fixed digits vs trimmed zeros).

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  u64, IEC, precision=2, space (trims zeros) | **1.30 us** | **216 ns** | 1.00x |
| byte-unit  u64 only, IEC, fixed 2dp, space | 1.62 us | 269 ns | 1.25x |
| indicatif HumanBytes  u64 only, IEC, fixed 2dp, space | 1.72 us | 287 ns | 1.33x |
| bytesize  u64 only, IEC, fixed 2dp, space | 1.76 us | 294 ns | 1.36x |
| humansize  u64 only, IEC, fixed 2dp, space | 1.99 us | 331 ns | 1.53x |
| human-repr  u64, IEC+space (feature) | 2.53 us | 422 ns | 1.95x |

## Bytes — reused buffer (`write!` into `String`), u64 inputs

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| prettier-bytes  u64 only, fixed 2dp, no negatives | **374 ns** | **47 ns** | 0.34x |
| humfmt  i8-u128, any precision | 1.09 us | 136 ns | 1.00x |
| bytesize  u64 only (SI), default 1dp, space | 1.46 us | 183 ns | 1.34x |
| humansize  u64 only, SI, precision=2, no space | 2.14 us | 267 ns | 1.96x |
| byte-unit  u64 (auto unit), write! + Display | 6.90 us | 863 ns | 6.32x |

## Bytes — reused buffer (`write!` into `String`) — aligned (IEC + space + precision=2), u64 inputs

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  u64, IEC, precision=2, space (trims zeros) | **732 ns** | **122 ns** | 1.00x |
| byte-unit  u64 only, IEC, fixed 2dp, space | 1.05 us | 176 ns | 1.44x |
| indicatif HumanBytes  u64 only, IEC, fixed 2dp, space | 1.11 us | 185 ns | 1.51x |
| bytesize  u64 only, IEC, fixed 2dp, space | 1.13 us | 189 ns | 1.55x |
| humansize  u64 only, IEC, fixed 2dp, space | 1.42 us | 237 ns | 1.94x |
| human-repr  u64, IEC+space (feature) | 1.99 us | 332 ns | 2.72x |

## Bytes — extended range (u128 > u64::MAX) — humfmt only

> No other benchmarked crate handles values above `u64::MAX`.

| Scenario | Median per-iteration | Time per value |
|---|---:|---:|
| humfmt/u128_extended | 1.64 us | 411 ns |

## Bytes — negative values (i64)

> bytesize and prettier-bytes do not participate (unsigned-only). This harness includes humfmt and humansize.

| Scenario | Median per-iteration | Time per value |
|---|---:|---:|
| humfmt/negative_i64 | 867 ns | 217 ns |
| humansize/negative_i64 | 1.38 us | 345 ns |

## Numbers — allocating (`to_string`), mixed i64 inputs

> human_format accepts f64 only and always returns an owned `String`. humfmt accepts all integer and float primitives and implements `Display`.

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  i64, precision=1 (default) | **2.14 us** | **214 ns** | 1.00x |
| humfmt  i64, precision=2 | 2.31 us | 231 ns | 1.08x |
| human_format  f64 only, EN only, precision=2, returns String | 5.94 us | 594 ns | 2.77x |

## Numbers — allocating (`to_string`), u64 inputs (apples-to-apples)

> human_format receives u64 cast to f64. Both crates produce compact `K/M/B` style output.

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  u64, precision=1 | **1.75 us** | **219 ns** | 1.00x |
| humfmt  u64, precision=2 | 1.87 us | 233 ns | 1.07x |
| human_format  u64 as f64, precision=2, returns String | 4.20 us | 526 ns | 2.40x |
| human_format  u64 as f64, precision=1, returns String | 4.30 us | 538 ns | 2.46x |

## Numbers — allocating (`to_string`), f64 inputs

> Float path only. human_format accepts f64 natively. human-repr and readable do not produce compact suffixes and are excluded.

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  f64, precision=2 | **3.08 us** | **385 ns** | 1.00x |
| human_format  f64, precision=2, returns String | 4.03 us | 503 ns | 1.31x |

## Numbers — reused buffer (`write!` into `String`), u64 inputs

> humfmt writes via `Display` with no intermediate allocation. human_format always allocates a `String`; we `push_str` it into the buffer.

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  u64, precision=1, write! | **1.01 us** | **127 ns** | 1.00x |
| humfmt  u64, precision=2, write! | 1.15 us | 144 ns | 1.14x |
| human_format  u64 as f64, precision=2, push_str (always allocs) | 4.24 us | 530 ns | 4.18x |

## Numbers — locale overhead (humfmt only)

> Measures the cost of locale-aware formatting. Russian and Polish require plural form selection based on the rendered value.

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  English, short | **1.74 us** | **217 ns** | 1.00x |
| humfmt  Polish, short | 1.93 us | 241 ns | 1.11x |
| humfmt  Russian, short | 2.42 us | 302 ns | 1.39x |
| humfmt  Polish, long (plural selection) | 2.57 us | 321 ns | 1.48x |
| humfmt  Russian, long (plural selection) | 2.71 us | 339 ns | 1.56x |
| humfmt  English, long | 5.03 us | 629 ns | 2.90x |

## Duration formatting — allocating

> humantime renders all non-zero units. humfmt caps at `max_units` (default 2). These produce different output for the same input.

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  short, 2 units (default) | **2.09 us** | **261 ns** | 1.00x |
| humfmt  short, 3 units | 2.52 us | 316 ns | 1.21x |
| humfmt  long labels, 2 units | 3.49 us | 436 ns | 1.67x |
| humantime  EN only, all non-zero units | 3.74 us | 467 ns | 1.79x |

## Relative time — allocating

> timeago returns an owned `String` from `convert()`. humfmt implements `Display` and writes directly with no intermediate allocation.

| Implementation | Median per-iteration | Time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  short, 2 units (explicit) | **2.61 us** | **327 ns** | 1.00x |
| humfmt  short, 2 units (default) | 2.77 us | 347 ns | 1.06x |
| humfmt  long, 2 units | 3.55 us | 443 ns | 1.36x |
| timeago  EN, 1 unit (default), returns String | 3.87 us | 484 ns | 1.48x |
| timeago  EN, 2 units, returns String | 5.60 us | 700 ns | 2.14x |


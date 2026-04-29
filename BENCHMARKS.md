# Benchmarks

This report is generated from Criterion `median.point_estimate` values.

Regenerate locally:

```bash
cargo bench --manifest-path tools/benchmarks/Cargo.toml
cargo run --release --manifest-path tools/benchmarks/Cargo.toml --bin report
```

---

## Capability Matrix

| Feature | humfmt | bytesize | byte-unit | prettier-bytes | indicatif (HumanBytes) | human-repr | humantime | timeago | human_format |
|---|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
| Byte sizes | yes | yes | yes | yes | yes | yes | no | no | no |
| Compact numbers | yes | no | no | no | no | yes | no | no | yes |
| Duration formatting | yes | no | no | no | no | yes | yes | yes | no |
| Relative time (ago) | yes | no | no | no | no | no | no | yes | no |
| Ordinals | yes | no | no | no | no | no | no | no | no |
| List formatting | yes | no | no | no | no | no | no | no | no |
| Signed input (negatives) | yes | no | no | no | no | yes | — | — | no |
| u128 / i128 range | yes | no | partial | no | no | yes | — | — | no |
| Float input | yes | no | no | no | no | yes | — | — | yes |
| Long-form labels | yes | no | yes | no | no | no | yes | yes | yes |
| Max-units cap | yes | — | — | — | — | — | no | yes | — |
| Binary (IEC) units | yes | yes | yes | yes | yes | yes | — | — | — |
| Configurable precision | yes | no | yes | no | no | no | — | — | yes |
| Locale-aware | yes | no | no | no | no | no | no | yes | no |
| Custom locale builder | yes | no | no | no | no | no | no | no | no |
| no_std compatible | yes | no | no | yes | no | no | no | no | no |
| Zero-alloc Display | yes | yes | no | yes | yes | yes | yes | no | no |

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

---

## Byte formatting semantics (examples)

These tables show representative outputs for a few byte values using the same configurations as the benchmarks.

### Default-style configuration

| Bytes | humfmt (SI, precision=2) | bytesize (SI, default) | byte-unit (`{:#.2}`) | prettier-bytes |
|---:|---|---|---|---|
| 1536 | `1.54KB` | `1.5 kB` | `1.5 KiB` | `1.54kB` |
| 9876543210 | `9.88GB` | `9.9 GB` | `9876543.21 KB` | `9.88GB` |

### Aligned configuration (IEC + space + precision=2)

| Bytes | humfmt (IEC, precision=2, trims) | indicatif HumanBytes | bytesize (`iec`, `:.2`) | byte-unit (binary, `:.2`) | human-repr (iec+space) |
|---:|---|---|---|---|---|
| 1536 | `1.5 KiB` | `1.50 KiB` | `1.50 KiB` | `1.50 KiB` | `1.5 KiB` |
| 1500 | `1.46 KiB` | `1.46 KiB` | `1.46 KiB` | `1.46 KiB` | `1.5 KiB` |
| 1514000000 | `1.41 GiB` | `1.41 GiB` | `1.41 GiB` | `1.41 GiB` | `1.41 GiB` |

---

## Bytes — allocating (`to_string`), u64 inputs

> prettier-bytes and bytesize are **u64-only**. humfmt accepts i8-i128 and u8-u128.

| Implementation | Median per-iteration | time per value | Relative vs humfmt |
|---|---:|---:|---:|
| prettier-bytes  u64 only, fixed 2dp, no negatives | **1.08 us** | **135 ns** | 0.59x |
| humfmt  i8-u128, any precision | 1.84 us | 230 ns | 1.00x |
| bytesize  u64 only (SI), default 1dp, space | 2.33 us | 291 ns | 1.26x |
| byte-unit  u64 (auto unit), format! uses String | 8.79 us | 1.10 us | 4.77x |

## Bytes — allocating (`to_string`) — aligned (IEC + space + precision=2), u64 inputs

> This group aligns unit system and spacing. Decimal digit policy can still differ (fixed digits vs trimmed zeros).

| Implementation | Median per-iteration | time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  u64, IEC, precision=2, space (trims zeros) | **1.28 us** | **213 ns** | 1.00x |
| byte-unit  u64 only, IEC, fixed 2dp, space | 1.63 us | 272 ns | 1.28x |
| bytesize  u64 only, IEC, fixed 2dp, space | 1.79 us | 299 ns | 1.40x |
| indicatif HumanBytes  u64 only, IEC, fixed 2dp, space | 1.79 us | 299 ns | 1.40x |
| human-repr  u64, IEC+space (feature), decimals are algorithmic | 2.65 us | 442 ns | 2.07x |

## Bytes — reused buffer (`write!` into `String`), u64 inputs

| Implementation | Median per-iteration | time per value | Relative vs humfmt |
|---|---:|---:|---:|
| prettier-bytes  u64 only, fixed 2dp, no negatives | **391 ns** | **49 ns** | 0.36x |
| humfmt  i8-u128, any precision | 1.08 us | 135 ns | 1.00x |
| bytesize  u64 only (SI), default 1dp, space | 1.42 us | 178 ns | 1.32x |
| byte-unit  u64 (auto unit), write! + Display | 6.97 us | 872 ns | 6.47x |

## Bytes — reused buffer (`write!` into `String`) — aligned (IEC + space + precision=2), u64 inputs

| Implementation | Median per-iteration | time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  u64, IEC, precision=2, space (trims zeros) | **733 ns** | **122 ns** | 1.00x |
| byte-unit  u64 only, IEC, fixed 2dp, space | 999 ns | 166 ns | 1.36x |
| indicatif HumanBytes  u64 only, IEC, fixed 2dp, space | 1.11 us | 185 ns | 1.51x |
| bytesize  u64 only, IEC, fixed 2dp, space | 1.15 us | 192 ns | 1.57x |
| human-repr  u64, IEC+space (feature), decimals are algorithmic | 2.03 us | 339 ns | 2.77x |

## Bytes — extended range (u128 > u64::MAX) — humfmt only

> No other benchmarked crate handles values above `u64::MAX`.

| Scenario | Median per-iteration | Time per value |
|---|---:|---:|
| humfmt/u128_extended | 1.63 us | 408 ns |

## Bytes — negative values (i64) — humfmt only

> No other benchmarked crate in this harness supports signed byte inputs.

| Scenario | Median per-iteration | Time per value |
|---|---:|---:|
| humfmt/negative_i64 | 867 ns | 217 ns |

## Numbers — allocating (`to_string`)

> human_format accepts f64 only and returns an owned `String`. humfmt accepts all integer and float primitives.

| Implementation | Median per-iteration | time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  i8-u128 + f32/f64, locale-aware | **4.09 us** | **409 ns** | 1.00x |
| human_format  f64 only, EN only, returns String | 5.87 us | 587 ns | 1.43x |

## Duration formatting — allocating

> humantime renders all non-zero units. humfmt caps at `max_units` (default 2). These produce different output for the same input.

| Implementation | Median per-iteration | time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  short, 2 units (default) | **2.22 us** | **277 ns** | 1.00x |
| humfmt  short, 3 units | 2.57 us | 321 ns | 1.16x |
| humfmt  long labels, 2 units | 3.45 us | 431 ns | 1.56x |
| humantime  EN only, all non-zero units | 3.54 us | 442 ns | 1.60x |

## Relative time — allocating

> timeago returns an owned `String` from `convert()`. humfmt implements `Display` and writes directly with no intermediate allocation.

| Implementation | Median per-iteration | time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  short, 2 units (explicit) | **2.61 us** | **327 ns** | 1.00x |
| humfmt  short, 2 units (default) | 2.65 us | 331 ns | 1.01x |
| humfmt  long, 2 units | 3.55 us | 444 ns | 1.36x |
| timeago  EN, 2 units, returns String | 5.66 us | 708 ns | 2.17x |
| timeago  EN, 1 unit (default), returns String | 6.76 us | 845 ns | 2.59x |


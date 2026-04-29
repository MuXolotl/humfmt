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
| Compact numbers | yes | no | no | no | no | no | yes | no | no | yes |
| Duration formatting | yes | no | no | no | no | no | yes | yes | yes | no |
| Relative time (ago) | yes | no | no | no | no | no | no | no | yes | no |
| Ordinals | yes | no | no | no | no | no | no | no | no | no |
| List formatting | yes | no | no | no | no | no | no | no | no | no |
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

> prettier-bytes, bytesize, humansize, and indicatif are **u64-only** in this harness. humfmt accepts i8-i128 and u8-u128.

| Implementation | Median per-iteration | time per value | Relative vs humfmt |
|---|---:|---:|---:|
| prettier-bytes  u64 only, fixed 2dp, no negatives | **359 ns** | **45 ns** | 0.59x |
| humfmt  i8-u128, any precision | 607 ns | 76 ns | 1.00x |
| bytesize  u64 only (SI), default 1dp, space | 1.03 us | 129 ns | 1.70x |
| humansize  u64 only, SI, precision=2, no space | 1.39 us | 173 ns | 2.28x |
| byte-unit  u64 (auto unit), format! uses String | 4.81 us | 602 ns | 7.93x |

## Bytes — allocating (`to_string`) — aligned (IEC + space + precision=2), u64 inputs

> This group aligns unit system and spacing. Decimal digit policy can still differ (fixed digits vs trimmed zeros).

| Implementation | Median per-iteration | time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  u64, IEC, precision=2, space (trims zeros) | **431 ns** | **72 ns** | 1.00x |
| byte-unit  u64 only, IEC, fixed 2dp, space | 738 ns | 123 ns | 1.71x |
| indicatif HumanBytes  u64 only, IEC, fixed 2dp, space | 786 ns | 131 ns | 1.82x |
| bytesize  u64 only, IEC, fixed 2dp, space | 815 ns | 136 ns | 1.89x |
| humansize  u64 only, IEC, fixed 2dp, space | 951 ns | 159 ns | 2.21x |
| human-repr  u64, IEC+space (feature), decimals are algorithmic | 1.05 us | 175 ns | 2.44x |

## Bytes — reused buffer (`write!` into `String`), u64 inputs

| Implementation | Median per-iteration | time per value | Relative vs humfmt |
|---|---:|---:|---:|
| prettier-bytes  u64 only, fixed 2dp, no negatives | **186 ns** | **23 ns** | 0.37x |
| humfmt  i8-u128, any precision | 499 ns | 62 ns | 1.00x |
| bytesize  u64 only (SI), default 1dp, space | 899 ns | 112 ns | 1.80x |
| humansize  u64 only, SI, precision=2, no space | 1.19 us | 149 ns | 2.38x |
| byte-unit  u64 (auto unit), write! + Display | 4.57 us | 571 ns | 9.15x |

## Bytes — reused buffer (`write!` into `String`) — aligned (IEC + space + precision=2), u64 inputs

| Implementation | Median per-iteration | time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  u64, IEC, precision=2, space (trims zeros) | **352 ns** | **59 ns** | 1.00x |
| byte-unit  u64 only, IEC, fixed 2dp, space | 559 ns | 93 ns | 1.59x |
| bytesize  u64 only, IEC, fixed 2dp, space | 628 ns | 105 ns | 1.79x |
| indicatif HumanBytes  u64 only, IEC, fixed 2dp, space | 656 ns | 109 ns | 1.87x |
| humansize  u64 only, IEC, fixed 2dp, space | 821 ns | 137 ns | 2.34x |
| human-repr  u64, IEC+space (feature), decimals are algorithmic | 945 ns | 157 ns | 2.69x |

## Bytes — extended range (u128 > u64::MAX) — humfmt only

> No other benchmarked crate handles values above `u64::MAX`.

| Scenario | Median per-iteration | Time per value |
|---|---:|---:|
| humfmt/u128_extended | 664 ns | 166 ns |

## Bytes — negative values (i64)

> bytesize and prettier-bytes do not participate (unsigned-only). This harness includes humfmt and humansize.

| Scenario | Median per-iteration | Time per value |
|---|---:|---:|
| humfmt/negative_i64 | 284 ns | 71 ns |
| humansize/negative_i64 | 674 ns | 168 ns |

## Numbers — allocating (`to_string`)

> human_format accepts f64 only and returns an owned `String`. humfmt accepts all integer and float primitives.

| Implementation | Median per-iteration | time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  i8-u128 + f32/f64, locale-aware | **2.20 us** | **220 ns** | 1.00x |
| human_format  f64 only, EN only, returns String | 2.42 us | 242 ns | 1.10x |

## Duration formatting — allocating

> humantime renders all non-zero units. humfmt caps at `max_units` (default 2). These produce different output for the same input.

| Implementation | Median per-iteration | time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  short, 2 units (default) | **726 ns** | **91 ns** | 1.00x |
| humantime  EN only, all non-zero units | 836 ns | 104 ns | 1.15x |
| humfmt  short, 3 units | 929 ns | 116 ns | 1.28x |
| humfmt  long labels, 2 units | 976 ns | 122 ns | 1.34x |

## Relative time — allocating

> timeago returns an owned `String` from `convert()`. humfmt implements `Display` and writes directly with no intermediate allocation.

| Implementation | Median per-iteration | time per value | Relative vs humfmt |
|---|---:|---:|---:|
| humfmt  short, 2 units (explicit) | **850 ns** | **106 ns** | 1.00x |
| humfmt  short, 2 units (default) | 866 ns | 108 ns | 1.02x |
| timeago  EN, 1 unit (default), returns String | 1.04 us | 131 ns | 1.23x |
| humfmt  long, 2 units | 1.08 us | 135 ns | 1.27x |
| timeago  EN, 2 units, returns String | 1.58 us | 197 ns | 1.86x |


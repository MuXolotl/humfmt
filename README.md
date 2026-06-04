<div align="center">

# humfmt

**Ergonomic human-readable formatting toolkit for Rust**

[![CI](https://github.com/MuXolotl/humfmt/actions/workflows/ci.yml/badge.svg)](https://github.com/MuXolotl/humfmt/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/humfmt.svg)](https://crates.io/crates/humfmt)
[![docs.rs](https://docs.rs/humfmt/badge.svg)](https://docs.rs/humfmt)
![License](https://img.shields.io/badge/license-MIT-blue.svg)

</div>

---

`humfmt` turns raw machine values into readable text without turning formatting
into a side quest.

```rust
use humfmt::Humanize;

println!("{}", 1_500_000.human_number());   // 1.5M
println!("{}", 1536_u64.human_bytes());     // 1.5KB
println!("{}", 0.423_f64.human_percent());  // 42.3%
println!("{}", humfmt::ordinal(21));        // 21st
```

**That's it.** Import the trait, call a method, done.

---

## What it does

| Input | Formatter | Output |
|-------|-----------|--------|
| `15320` | `number` | `15.3K` |
| `1536` | `bytes` | `1.5KB` |
| `0.423` | `percent` | `42.3%` |
| `21` | `ordinal` | `21st` |
| `3661s` | `duration` | `1h 1m` |
| `90s` | `ago` | `1m 30s ago` |
| `["red", "green", "blue"]` | `list` | `red, green, and blue` |

All formatters implement `Display` — no intermediate heap strings. Write directly into any buffer.

---

## Quick start

```toml
[dependencies]
humfmt = "0.6"
```

```rust
use humfmt::Humanize;
use core::time::Duration;

// Extension trait — shortest path
println!("{}", 1_500_000.human_number());                     // 1.5M
println!("{}", 1536_u64.human_bytes());                       // 1.5KB
println!("{}", 0.423_f64.human_percent());                    // 42.3%
println!("{}", 42_u32.human_ordinal());                       // 42nd
println!("{}", Duration::from_secs(90).human_ago());          // 1m 30s ago

// Free functions — same result, no trait import needed
println!("{}", humfmt::number(15320));                        // 15.3K
println!("{}", humfmt::bytes(1536));                          // 1.5KB
println!("{}", humfmt::percent(0.423));                       // 42.3%
println!("{}", humfmt::ordinal(21));                          // 21st
println!("{}", humfmt::duration(Duration::from_secs(3661)));  // 1h 1m
println!("{}", humfmt::list(&["red", "green", "blue"]));      // red, green, and blue
```

---

## Customization

Every formatter has a `*_with` variant that takes an options builder:

```rust
use core::time::Duration;
use humfmt::{BytesOptions, DurationOptions, Humanize, NumberOptions, PercentOptions};

// Bytes: binary units, space before suffix
let disk = 1536_u64.human_bytes_with(
    BytesOptions::new().binary().precision(2).space(true)
);
println!("{disk}"); // 1.5 KiB

// Bytes: bits mode for network speeds
let speed = 1_500_000_u64.human_bytes_with(
    BytesOptions::new().bits(true)
);
println!("{speed}"); // 12Mb

// Bytes: always show in megabytes
let fixed = 1_500_000_u64.human_bytes_with(
    BytesOptions::new().unit(humfmt::ByteUnit::MB).precision(3)
);
println!("{fixed}"); // 1.5MB

// Bytes: clamp minimum to KB (500 bytes -> 0.5 KB)
let clamped = 500_u64.human_bytes_with(
    BytesOptions::new().min_unit(humfmt::ByteUnit::KB).precision(2)
);
println!("{clamped}"); // 0.5KB

// Numbers: long-form
let n = 15_320.human_number_with(
    NumberOptions::new().precision(2).long_units()
);
println!("{n}"); // 15.32 thousand

// Numbers: full number with digit grouping
let full = 1_234_567.human_number_with(
    NumberOptions::new().compact(false).separators(true)
);
println!("{full}"); // 1,234,567

// Numbers: significant digits
let sig = 12345.human_number_with(
    NumberOptions::new().significant_digits(3)
);
println!("{sig}"); // 12.3K

// Numbers: forced sign for deltas
let delta = 1500.human_number_with(
    NumberOptions::new().force_sign(true)
);
println!("{delta}"); // +1.5K

// Numbers: rounding modes
use humfmt::RoundingMode;
let floor = 1_900.human_number_with(
    NumberOptions::new().precision(0).rounding(RoundingMode::Floor)
);
println!("{floor}"); // 1K

// Numbers: custom separators
let european = 1_234_567.human_number_with(
    NumberOptions::new()
        .compact(false)
        .separators(true)
        .decimal_separator(',')
        .group_separator(' ')
);
println!("{european}"); // 1 234 567

// Percentages: 2 decimal places, fixed precision
let ratio = 0.425_f64.human_percent_with(
    PercentOptions::new().precision(2).fixed_precision(true)
);
println!("{ratio}"); // 42.50%

// Percentages: forced sign
let change = 0.15_f64.human_percent_with(
    PercentOptions::new().force_sign(true)
);
println!("{change}"); // +15%

// Duration: long-form, 3 units
let elapsed = Duration::from_secs(3665).human_duration_with(
    DurationOptions::new().long_units().max_units(3)
);
println!("{elapsed}"); // 1 hour 1 minute 5 seconds

// Relative time: 3 units
let ago = Duration::from_secs(3665).human_ago_with(
    DurationOptions::new().max_units(3)
);
println!("{ago}"); // 1h 1m 5s ago
```

---

## Lists

```rust
use humfmt::{list, list_with, ListOptions};

// Default: Oxford comma
assert_eq!(
    list(&["red", "green", "blue"]).to_string(),
    "red, green, and blue"
);

// No serial comma
let no_oxford = list_with(
    &["red", "green", "blue"],
    ListOptions::new().no_serial_comma(),
);
assert_eq!(no_oxford.to_string(), "red, green and blue");

// Custom conjunction
let plus = list_with(
    &["red", "green", "blue"],
    ListOptions::new().conjunction("plus").no_serial_comma(),
);
assert_eq!(plus.to_string(), "red, green plus blue");

// Custom separator
let piped = list_with(
    &["red", "green", "blue"],
    ListOptions::new().separator(" | ").conjunction("&"),
);
assert_eq!(piped.to_string(), "red | green & blue");
```

---

## Performance

`humfmt` is designed to be cheap in hot paths:

- **Zero-alloc `Display`** — formatters write directly into the output, no intermediate `String`
- **O(1) scaling** — integer path uses `ilog10`, float path uses IEEE 754 exponent
- **`no_std`** — works without the standard library
- **No dependencies** — core crate has zero required dependencies

See [BENCHMARKS.md](./BENCHMARKS.md) for comparisons against `humansize`, `bytesize`, `byte-unit`, `prettier-bytes`, `human_format`, `numfmt`, `humantime`, `timeago`, and others.

<details>
<summary>Charts</summary>

<p align="center">
  <img alt="Bytes comparison benchmarks" src="assets/benchmarks/bytes_comparison_dark.svg">
</p>

<p align="center">
  <img alt="Numbers comparison benchmarks" src="assets/benchmarks/numbers_dark.svg">
</p>

<p align="center">
  <img alt="Duration and relative-time benchmarks" src="assets/benchmarks/time_comparison_dark.svg">
</p>

</details>

---

## Fuzzing

`humfmt` includes a fuzzing harness using `cargo-fuzz` to catch edge cases in numeric and formatting logic.

### Running fuzz targets

Fuzzing requires the **nightly** toolchain.

```bash
# Install cargo-fuzz (once)
cargo install cargo-fuzz

# Run a specific fuzz target
cargo +nightly fuzz run fuzz_number
cargo +nightly fuzz run fuzz_bytes
cargo +nightly fuzz run fuzz_percent
cargo +nightly fuzz run fuzz_duration
cargo +nightly fuzz run fuzz_ordinal
cargo +nightly fuzz run fuzz_list
```

The fuzz targets are located in `fuzz/fuzz_targets/`.

---

## Feature flags

| Feature | Default | Description |
|---------|---------|-------------|
| `std` | ✓ | Standard library build |
| `chrono` | | `chrono::TimeDelta` / `DateTime` adapters |
| `time` | | `time::Duration` / `OffsetDateTime` adapters |

For `no_std` targets:

```toml
[dependencies]
humfmt = { version = "0.6", default-features = false }
```

With ecosystem integrations:

```toml
[dependencies]
humfmt = { version = "0.6", features = ["chrono", "time"] }
```

---

## Documentation

- **[docs.rs](https://docs.rs/humfmt)** — full API reference
- **[BENCHMARKS.md](./BENCHMARKS.md)** — performance comparisons and methodology
- **[CHANGELOG.md](./CHANGELOG.md)** — what changed and when
- **[TODO.md](./TODO.md)** — planned features and known issues
- **[examples/](./examples)** — runnable examples
- **[tests/](./tests)** — integration and property tests

---

## Philosophy

This crate follows one simple rule:

> Human formatting should feel stupidly easy.

No giant config ceremony. No formatting gymnastics. No "why is this so annoying?" moments.

Just:

```rust
println!("{}", 1_500_000.human_number());
```

and move on with your life.

---

## License

MIT.

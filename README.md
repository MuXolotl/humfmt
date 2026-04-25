# humfmt

Ergonomic human-readable formatting toolkit for Rust.

[![CI](https://github.com/MuXolotl/humfmt/actions/workflows/ci.yml/badge.svg)](https://github.com/MuXolotl/humfmt/actions/workflows/ci.yml)
![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Status](https://img.shields.io/badge/status-actively--developing-brightgreen.svg)

`humfmt` turns raw machine values into readable text without making formatting feel
like work.

It currently includes:

- byte-size rendering (`1536 -> 1.5KB`)
- compact number rendering (`15320 -> 15.3K`)
- locale-aware compact numbers
- ordinal rendering (`21 -> 21st`)
- duration rendering (`3661s -> 1h 1m`)
- relative time rendering (`90s -> 1m 30s ago`)
- builder-style customization
- optional `chrono` and `time` integration
- `no_std`-friendly usage with `alloc`

## Quick Example

```rust
use humfmt::Humanize;

fn main() {
    println!("{}", humfmt::bytes(1536)); // 1.5KB
    println!("{}", humfmt::number(15320)); // 15.3K
    println!("{}", 1_500_000.human_number()); // 1.5M
    println!("{}", humfmt::ordinal(21)); // 21st
    println!("{}", humfmt::duration(core::time::Duration::from_secs(3661))); // 1h 1m
    println!("{}", humfmt::ago(core::time::Duration::from_secs(90))); // 1m 30s ago
}
```

## Customized Formatting

```rust
use core::time::Duration;

use humfmt::{BytesOptions, DurationOptions, Humanize, NumberOptions};

fn main() {
    let disk = 1536_u64.human_bytes_with(BytesOptions::new().binary());
    println!("{disk}"); // 1.5KiB

    let out = 15_320.human_number_with(
        NumberOptions::new()
            .precision(2)
            .long_units(),
    );

    println!("{out}"); // 15.32 thousand

    let elapsed = Duration::from_millis(1500)
        .human_duration_with(DurationOptions::new().long_units());

    println!("{elapsed}"); // 1 second 500 milliseconds

    let relative = Duration::from_secs(3665)
        .human_ago_with(DurationOptions::new().max_units(3));

    println!("{relative}"); // 1h 1m 5s ago
}
```

## Locale Examples

```rust
use humfmt::{locale::Russian, number_with, NumberOptions};

let out = number_with(15_320, NumberOptions::new().locale(Russian));
assert_eq!(out.to_string(), "15,3 тыс.");
```

```rust
use humfmt::{locale::Polish, number_with, ordinal_with, NumberOptions};

let out = number_with(15_320, NumberOptions::new().locale(Polish));
assert_eq!(out.to_string(), "15,3 tys.");
assert_eq!(ordinal_with(21, Polish).to_string(), "21.");
```

```rust
use humfmt::{locale::CustomLocale, number_with, NumberOptions};

let locale = CustomLocale::english()
    .short_suffix(1, "k")
    .separators(',', '.');

let out = number_with(15_320, NumberOptions::new().locale(locale));
assert_eq!(out.to_string(), "15,3k");
```

## Current Features

- compact number formatter
- byte-size formatter
- ordinal formatter
- duration formatter
- relative time formatter
- long and short units
- English, Russian, and Polish locale packs
- custom locale builder for suffix and separator overrides
- doctests and integration tests

## Installation

```toml
[dependencies]
humfmt = "0.1"
```

For `no_std` targets with `alloc` available:

```toml
[dependencies]
humfmt = { version = "0.1", default-features = false }
```

## Feature Flags

- `std` (default): enables the standard-library build
- `default-features = false`: builds the current formatter on `no_std` + `alloc`
- `english`: baseline locale included in the default feature set
- `russian`: enables the `humfmt::locale::Russian` locale pack
- `polish`: enables the `humfmt::locale::Polish` locale pack
- `alloc`: reserved compatibility flag in `0.1.x`
- `chrono`: enables adapters for `chrono::TimeDelta` and `chrono::DateTime`
- `time`: enables adapters for `time::Duration` and `time::OffsetDateTime`

## Development Status

`humfmt` is still early-stage, but the formatter surface is already useful and
growing in a fairly disciplined way. The current direction is to keep the crate
small, predictable, and pleasant to use while expanding its locale and
humanizer coverage.

## Documentation

- examples: [`examples/`](./examples)
- tests: [`tests/`](./tests)
- crates.io: [humfmt](https://crates.io/crates/humfmt)
- docs.rs: [humfmt docs](https://docs.rs/humfmt)

## License

MIT.

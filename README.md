<div align="center">

# humfmt

**Ergonomic human-readable formatting toolkit for Rust**

[![CI](https://github.com/MuXolotl/humfmt/actions/workflows/ci.yml/badge.svg)](https://github.com/MuXolotl/humfmt/actions/workflows/ci.yml)
![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Status](https://img.shields.io/badge/status-actively--developing-brightgreen.svg)

</div>

---

`humfmt` is a lightweight Rust library for turning raw machine values into human-friendly text.

Designed to provide:

- compact number rendering (`15320 -> 15.3K`)
- ordinal rendering (`21 -> 21st`)
- duration rendering (`3661s -> 1h 1m`)
- relative time rendering (`90s -> 1m 30s ago`)
- fluent builder-style customization
- locale-ready suffix formatting
- ergonomic extension trait API
- `no_std`-friendly usage with `alloc`
- zero-macro, zero-nonsense usage

The crate aims to be tiny, intuitive, and pleasant enough that formatting stops feeling like work.

---

## ✨ Quick Example

```rust
use humfmt::Humanize;

fn main() {
    println!("{}", humfmt::number(15320));          // 15.3K
    println!("{}", 1_500_000.human_number());      // 1.5M
    println!("{}", humfmt::ordinal(21));           // 21st
    println!("{}", humfmt::duration(core::time::Duration::from_secs(3661))); // 1h 1m
    println!("{}", humfmt::ago(core::time::Duration::from_secs(90))); // 1m 30s ago
}
```

---

## ⚙️ Customized Formatting

```rust
use core::time::Duration;

use humfmt::{DurationOptions, Humanize, NumberOptions};

fn main() {
    let out = 15_320.human_number_with(
        NumberOptions::new()
            .precision(2)
            .long_units()
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

---

## ✅ Current Features

* [x] Compact number formatter
* [x] Ordinal formatter
* [x] Duration formatter
* [x] Relative time formatter
* [x] Builder-style `NumberOptions`
* [x] `Humanize` extension trait
* [x] Long and short suffix units
* [x] Locale abstraction foundation
* [x] Doctests and integration tests

---

## 🗺️ Planned Roadmap

Upcoming humanizers planned for future releases:

* [ ] `bytes()` — human-readable byte sizes
* [ ] additional locale packs
* [ ] zero-allocation optimization pass

---

## 📦 Installation

Add dependency:

```toml
[dependencies]
humfmt = "0.1"
```

For `no_std` targets with `alloc` available:

```toml
[dependencies]
humfmt = { version = "0.1", default-features = false }
```

---

## Feature Flags

- `std` (default): enables the standard-library build.
- `default-features = false`: builds the current formatter on `no_std` + `alloc`.
- `english` stays in the default set for forward-compatible locale gating in `0.1.x`.
- `alloc`, `russian`, and `polish` are reserved compatibility flags in `0.1.x`; they do not change runtime behavior yet.
- `chrono` and `time` keep their optional dependencies wired and CI-checked, but no public integration API is exposed yet.

---

## 🧪 Development Status

`humfmt` is under active early-stage development.

The current public surface is intentionally small and focused on compact number formatting first.

Expect rapid iteration, formatter additions, and locale improvements.

---

## 📚 Documentation

* examples available in `/examples`
* integration tests available in `/tests`
* rustdoc examples available on all public number APIs
* published crate: <https://crates.io/crates/humfmt>
* API docs: <https://docs.rs/humfmt>

---

## 📄 License

Licensed under MIT.

---

## ⭐ Philosophy

This crate follows one simple rule:

> Human formatting should feel stupidly easy.

No giant config structs.
No formatting gymnastics.
No "why is this so annoying?" moments.

Just:

```rust
println!("{}", 1500000.human_number());
```

and move on with your life.

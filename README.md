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
}
```

---

## ⚙️ Customized Formatting

```rust
use humfmt::{Humanize, NumberOptions};

fn main() {
    let out = 15_320.human_number_with(
        NumberOptions::new()
            .precision(2)
            .long_units()
    );

    println!("{out}"); // 15.32 thousand
}
```

---

## ✅ Current Features

* [x] Compact number formatter
* [x] Builder-style `NumberOptions`
* [x] `Humanize` extension trait
* [x] Long and short suffix units
* [x] Locale abstraction foundation
* [x] Doctests and integration tests

---

## 🗺️ Planned Roadmap

Upcoming humanizers planned for future releases:

* [ ] `bytes()` — human-readable byte sizes
* [ ] `duration()` — compact duration formatting
* [ ] `ago()` — relative time rendering
* [ ] `ordinal()` — 1st / 2nd / 3rd style helpers
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

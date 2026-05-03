# humfmt

Ergonomic human-readable formatting toolkit for Rust.

`humfmt` focuses on a baby-simple API with a fast, allocation-free `Display` core.

## Quick start

The crate exposes two usage styles:

1) **Functions** (`humfmt::number(...)`, `humfmt::bytes(...)`, ...)
2) **Extension trait** (`Humanize`) for ergonomic chaining

```rust
use humfmt::Humanize;

assert_eq!(humfmt::bytes(1536).to_string(), "1.5KB");
assert_eq!(humfmt::number(15320).to_string(), "15.3K");
assert_eq!(1_500_000.human_number().to_string(), "1.5M");
assert_eq!(0.423_f64.human_percent().to_string(), "42.3%");
assert_eq!(humfmt::ordinal(21).to_string(), "21st");
assert_eq!(humfmt::duration(core::time::Duration::from_secs(3661)).to_string(), "1h 1m");
assert_eq!(humfmt::ago(core::time::Duration::from_secs(90)).to_string(), "1m 30s ago");
assert_eq!(humfmt::list(&["red", "green", "blue"]).to_string(), "red, green, and blue");
```

## Allocation model (important)

All formatters implement `Display` and write directly into the provided formatter.
This means:

- `format!("{}", humfmt::number(x))` allocates **because `format!` must produce a `String`**
- `write!(&mut existing_string, "{}", humfmt::number(x))` can avoid new allocations
  when you reuse the buffer

Example:

```rust
use core::fmt::Write as _;

let mut out = String::with_capacity(32);
out.clear();
write!(&mut out, "{}", humfmt::bytes(9_876_543_210_u64)).unwrap();
assert!(!out.is_empty());
```

## Formatters

### Compact numbers (`number`, `NumberOptions`)

Use this for values like `1_500_000 -> 1.5M` and locale-aware separators.

```rust
use humfmt::{number_with, NumberOptions};

let out = number_with(15_320, NumberOptions::new().precision(2));
assert_eq!(out.to_string(), "15.32K");
```

#### Defaults (English)
| Option | Default | Meaning |
|---|---:|---|
| precision | 1 | fractional digits for compact values |
| significant_digits | none | round to N total significant digits |
| compact | true | enable magnitude scaling (`1.5K` vs `1500`) |
| force_sign | false | output `+` for positive numbers |
| rounding | `HalfUp` | HalfUp, Floor, Ceil behaviour |
| long_units | false | `K` vs ` thousand` |
| separators | false | group separator for unscaled output |

#### Notes / edge cases
- Integers support full `i128` / `u128` range.
- Floats support `f32` / `f64`. Non-finite values render as `inf`, `-inf`, `NaN`.
- Small negative floats that round to zero render as `0` (never `-0` or `+0`).
- Rounding may rescale across a boundary (e.g. `999_950 -> 1M`).

### Byte sizes (`bytes`, `BytesOptions`)

Formats byte counts using decimal (SI) or binary (IEC) units.

```rust
use humfmt::{bytes_with, BytesOptions};

assert_eq!(humfmt::bytes(1536_u64).to_string(), "1.5KB");
assert_eq!(bytes_with(1536_u64, BytesOptions::new().binary()).to_string(), "1.5KiB");
```

#### Defaults
| Option | Default | Meaning |
|---|---:|---|
| precision | 1 | fractional digits for scaled values |
| binary | false | SI (1000) vs IEC (1024) |
| bits | false | multiply by 8 and use bit units (`Mb`) |
| long_units | false | `KB` vs ` kilobytes` |
| decimal_separator | `.` | decimal separator for scaled output |
| space | `false` | add a space before short unit labels |

#### Locale-aware decimal separator
Byte unit labels are currently English-only, but the decimal separator is configurable:

```rust
use humfmt::BytesOptions;

let opts = BytesOptions::new().decimal_separator(',');
assert_eq!(humfmt::bytes_with(1536_u64, opts).to_string(), "1,5KB");
```

You can also copy the decimal separator from any `Locale`:

```rust
use humfmt::{bytes_with, BytesOptions};
use humfmt::locale::CustomLocale;

let locale = CustomLocale::english().decimal_separator(',');
let opts = BytesOptions::new().locale(locale);

assert_eq!(bytes_with(1536_u64, opts).to_string(), "1,5KB");
```

#### Optional space before short units
If you prefer `1.5 KB` instead of `1.5KB`, enable spacing:

```rust
use humfmt::BytesOptions;

let opts = BytesOptions::new().space(true);
assert_eq!(humfmt::bytes_with(1536_u64, opts).to_string(), "1.5 KB");
assert_eq!(humfmt::bytes_with(999_u64, opts).to_string(), "999 B");
```

#### Notes / edge cases
- Signed inputs are supported (e.g. `-1536 -> -1.5KB`).
- Precision is clamped to a small maximum to keep formatting cheap and predictable.
- The unit ceiling is `EB` / `EiB`.

### Percentages (`percent`, `PercentOptions`)

Converts a ratio (e.g. `0.423`) into a human-readable percentage (`"42.3%"`).

```rust
use humfmt::{percent_with, PercentOptions};

assert_eq!(humfmt::percent(0.423).to_string(), "42.3%");

let opts = PercentOptions::new().force_sign(true);
assert_eq!(percent_with(0.15_f64, opts).to_string(), "+15%");
```

#### Defaults (English)
| Option | Default | Meaning |
|---|---:|---|
| precision | 1 | fractional digits |
| force_sign | false | output `+` for positive percentages |
| fixed_precision | false | keep trailing zeros (`42.50%`) |

### Ordinals (`ordinal`)

Locale-aware ordinal markers like `1st`, `21.` or `42-й`.

```rust
use humfmt::ordinal;

assert_eq!(ordinal(1).to_string(), "1st");
assert_eq!(ordinal(11).to_string(), "11th");
assert_eq!(ordinal(21).to_string(), "21st");
```

### Durations (`duration`, `DurationOptions`)

Compact or long-form duration formatting for `core::time::Duration`.

```rust
use core::time::Duration;

assert_eq!(
    humfmt::duration(Duration::from_secs(3661)).to_string(),
    "1h 1m"
);
```

#### Defaults (English)
| Option | Default | Meaning |
|---|---:|---|
| max_units | 2 | maximum number of non-zero units to render |
| long_units | false | `h` vs ` hour` |
| locale | English | affects unit labels and wording |

#### Notes / edge cases
- `Duration::ZERO` renders as `0s` (or the long-form equivalent).
- The formatter renders the largest units first (days → nanoseconds).
- The output is intentionally capped (default is 2 units) to stay compact.

### Relative time (`ago`)

`ago` builds on the duration formatter and appends the locale-specific "ago" word.

```rust
use core::time::Duration;

assert_eq!(
    humfmt::ago(Duration::from_secs(90)).to_string(),
    "1m 30s ago"
);
```

### Lists (`list`, `ListOptions`)

Natural-language list formatting:

```rust
use humfmt::list;

assert_eq!(
    list(&["red", "green", "blue"]).to_string(),
    "red, green, and blue"
);
```

#### Defaults (English)
- Separator between items: `", "`
- Conjunction: `"and"`
- Serial comma enabled: `true`

#### Serial comma and custom separators
The serial comma (Oxford comma) is a comma-specific style rule.

If you override the list separator away from commas (for example, using `" | "`),
`humfmt` will not inject a literal comma before the final conjunction even if
serial comma is enabled.

Example:

```rust
use humfmt::{list_with, ListOptions};
use humfmt::locale::CustomLocale;

let locale = CustomLocale::english()
    .list_separator(" | ")
    .and_word("&")
    .serial_comma(true);

let out = list_with(&["red", "green", "blue"], ListOptions::new().locale(locale));
assert_eq!(out.to_string(), "red | green & blue");
```

## Locales

Built-in locale packs exist for:

- English (default)
- Russian (feature `russian`)
- Polish (feature `polish`)

You can also use `CustomLocale` to override separators, suffixes, ordinals,
list style, duration unit labels, and the "ago" word.

## Optional ecosystem integrations

If enabled via feature flags:

- `chrono`: adapt `chrono::TimeDelta` and `chrono::DateTime` into `humfmt` formatters
- `time`: adapt `time::Duration` and `time::OffsetDateTime` into `humfmt` formatters

Both integrations provide:
- `*_checked` functions returning `DurationConversionError`
- convenience APIs returning `NegativeDurationError` for negative inputs

## no_std

`humfmt` supports `no_std`:

```toml
[dependencies]
humfmt = { version = "0.4", default-features = false }
```

## Benchmarks

See `BENCHMARKS.md` for a reproducible comparison report and methodology.

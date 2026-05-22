# humfmt

Ergonomic human-readable formatting toolkit for Rust.

`humfmt` turns raw machine values into readable text. All formatters implement
`Display` and write directly into the output — no intermediate heap strings,
no hidden allocations.

## Quick start

The crate exposes two usage styles for every formatter:

1. **Free functions** — `humfmt::number(15320)` -> `"15.3K"`
2. **Extension trait** — `15320.human_number()` -> `"15.3K"`

```rust
use humfmt::Humanize;
use core::time::Duration;

// Numbers
assert_eq!(humfmt::number(15320).to_string(), "15.3K");
assert_eq!(1_500_000.human_number().to_string(), "1.5M");
assert_eq!(humfmt::number(-12_500).to_string(), "-12.5K");

// Bytes
assert_eq!(humfmt::bytes(1536).to_string(), "1.5KB");
assert_eq!(1024_u64.human_bytes_with(
    humfmt::BytesOptions::new().binary()
).to_string(), "1KiB");

// Percentages
assert_eq!(humfmt::percent(0.423).to_string(), "42.3%");
assert_eq!(0.15_f64.human_percent().to_string(), "15%");

// Ordinals
assert_eq!(humfmt::ordinal(21).to_string(), "21st");
assert_eq!(42_u32.human_ordinal().to_string(), "42nd");

// Durations
assert_eq!(humfmt::duration(Duration::from_secs(3661)).to_string(), "1h 1m");

// Relative time
assert_eq!(humfmt::ago(Duration::from_secs(90)).to_string(), "1m 30s ago");

// Lists
assert_eq!(humfmt::list(&["red", "green", "blue"]).to_string(), "red, green, and blue");
```

## Allocation model

All formatters implement `Display` and write directly into the provided formatter.
This means:

- `format!("{}", humfmt::number(x))` allocates — because `format!` must produce a `String`
- `write!(&mut buf, "{}", humfmt::number(x))` reuses the buffer — no new allocation from the formatter

Example:

```rust
use core::fmt::Write as _;

let mut out = String::with_capacity(32);
out.clear();
write!(&mut out, "{}", humfmt::bytes(9_876_543_210_u64)).unwrap();
// `out` was reused, no heap allocation from the formatter
```

---

## Compact numbers

Turns large values into short forms: `15320` -> `"15.3K"`, `1_500_000` -> `"1.5M"`.

```rust
use humfmt::{number, number_with, NumberOptions};

assert_eq!(number(15320).to_string(), "15.3K");
assert_eq!(number(1_500_000).to_string(), "1.5M");
assert_eq!(number(-12_500).to_string(), "-12.5K");
```

### Defaults

| Option | Default | Meaning |
|---|---|---|
| `precision` | 1 | fractional digits for compact values |
| `significant_digits` | none | round to N total significant digits |
| `compact` | true | enable magnitude scaling (`1.5K` vs `1500`) |
| `force_sign` | false | output `+` for positive numbers |
| `rounding` | `HalfUp` | HalfUp, Floor, Ceil behaviour |
| `long_units` | false | `K` vs ` thousand` |
| `separators` | false | group separator for unscaled output |
| `fixed_precision` | false | keep trailing zeros (`1.50K`) |
| `decimal_separator` | `'.'` | decimal separator character |
| `group_separator` | `','` | digit grouping separator character |

### Suffix table

| Index | Magnitude | Short | Long |
|:---:|---:|---|---|
| 0 | 1 | `""` | `""` |
| 1 | 10^3 | `K` | ` thousand` |
| 2 | 10^6 | `M` | ` million` |
| 3 | 10^9 | `B` | ` billion` |
| 4 | 10^12 | `T` | ` trillion` |
| 5 | 10^15 | `Qa` | ` quadrillion` |
| 6 | 10^18 | `Qi` | ` quintillion` |
| 7 | 10^21 | `Sx` | ` sextillion` |
| 8 | 10^24 | `Sp` | ` septillion` |
| 9 | 10^27 | `Oc` | ` octillion` |
| 10 | 10^30 | `No` | ` nonillion` |
| 11 | 10^33 | `Dc` | ` decillion` |
| 12 | 10^36 | `Ud` | ` undecillion` |

The table goes up to `Ud` (`10^36`), which keeps the full `u128` / `i128`
range compact without falling back to very large `Dc` values.

### Precision

```rust
use humfmt::{number_with, NumberOptions};

assert_eq!(number_with(15_320, NumberOptions::new().precision(0)).to_string(), "15K");
assert_eq!(number_with(15_320, NumberOptions::new().precision(1)).to_string(), "15.3K");
assert_eq!(number_with(15_320, NumberOptions::new().precision(2)).to_string(), "15.32K");
```

Precision is clamped to `0..=6`.

### Significant digits

Instead of fixed decimal places, round to N total significant digits:

```rust
use humfmt::{number_with, NumberOptions};

let opts = NumberOptions::new().significant_digits(3);
assert_eq!(number_with(1234, opts).to_string(), "1.23K");
assert_eq!(number_with(12345, opts).to_string(), "12.3K");
assert_eq!(number_with(123456, opts).to_string(), "123K");
assert_eq!(number_with(1234567, opts).to_string(), "1.23M");
```

With fixed precision, zeros are padded intelligently:

```rust
use humfmt::{number_with, NumberOptions};

let opts = NumberOptions::new().significant_digits(3).fixed_precision(true);
assert_eq!(number_with(1, opts).to_string(), "1.00");
assert_eq!(number_with(10, opts).to_string(), "10.0");
assert_eq!(number_with(100, opts).to_string(), "100");
```

Significant digits are clamped to `1..=39` (the maximum number of decimal
digits in a `u128`). Fractional output is still capped at 6 decimal places,
matching the formatter-wide precision cap.

### Disabling compact scaling

Combine `compact(false)` with `separators(true)` for fully formatted large numbers:

```rust
use humfmt::{number_with, NumberOptions};

let opts = NumberOptions::new().compact(false).separators(true);
assert_eq!(number_with(1_234_567, opts).to_string(), "1,234,567");
assert_eq!(number_with(12_345, opts).to_string(), "12,345");
```

Without separators:

```rust
use humfmt::{number_with, NumberOptions};

let opts = NumberOptions::new().compact(false);
assert_eq!(number_with(1_500_000, opts).to_string(), "1500000");
assert_eq!(number_with(1_500_000.5_f64, opts).to_string(), "1500000.5");
```

### Custom separators

```rust
use humfmt::{number_with, NumberOptions};

// European-style: comma decimal, space group
let opts = NumberOptions::new()
    .compact(false)
    .separators(true)
    .decimal_separator(',')
    .group_separator(' ');
assert_eq!(number_with(1_234_567, opts).to_string(), "1 234 567");
assert_eq!(number_with(1_234_567.5_f64, opts).to_string(), "1 234 567,5");
```

### Forced sign

Useful for delta/change displays:

```rust
use humfmt::{number_with, NumberOptions};

let opts = NumberOptions::new().force_sign(true);
assert_eq!(number_with(1500, opts).to_string(), "+1.5K");
assert_eq!(number_with(42, opts).to_string(), "+42");
assert_eq!(number_with(0, opts).to_string(), "0");
assert_eq!(number_with(-1500, opts).to_string(), "-1.5K");
```

### Rounding modes

```rust
use humfmt::{number_with, NumberOptions, RoundingMode};

let base = NumberOptions::new().precision(0);

assert_eq!(number_with(1_900, base.rounding(RoundingMode::HalfUp)).to_string(), "2K");
assert_eq!(number_with(1_900, base.rounding(RoundingMode::Floor)).to_string(), "1K");
assert_eq!(number_with(1_900, base.rounding(RoundingMode::Ceil)).to_string(), "2K");

let base2 = NumberOptions::new().precision(2);
assert_eq!(number_with(-1234, base2.rounding(RoundingMode::HalfUp)).to_string(), "-1.23K");
assert_eq!(number_with(-1234, base2.rounding(RoundingMode::Floor)).to_string(), "-1.24K");
assert_eq!(number_with(-1234, base2.rounding(RoundingMode::Ceil)).to_string(), "-1.23K");
```

Rounding may rescale across a suffix boundary:

```rust
use humfmt::{number_with, NumberOptions, RoundingMode};

let base = NumberOptions::new().precision(0);
assert_eq!(number_with(999_500, base.rounding(RoundingMode::HalfUp)).to_string(), "1M");
assert_eq!(number_with(999_500, base.rounding(RoundingMode::Floor)).to_string(), "999K");
assert_eq!(number_with(999_500, base.rounding(RoundingMode::Ceil)).to_string(), "1M");
```

### Long-form labels

```rust
use humfmt::{number_with, NumberOptions};

let opts = NumberOptions::new().long_units();
assert_eq!(number_with(15_320, opts).to_string(), "15.3 thousand");
assert_eq!(number_with(1_000_000, opts).to_string(), "1 million");
assert_eq!(number_with(1_000_000_000_000_000_000_000_000_000_000_000_000_u128, opts).to_string(), "1 undecillion");
```

### Edge cases

| Input | Output | Notes |
|---:|---|---|
| `0` | `"0"` | No suffix, no sign |
| `1` | `"1"` | Below threshold |
| `999` | `"999"` | Below threshold |
| `1_000` | `"1K"` | First compact threshold |
| `999_950` | `"1M"` | Rounds up across boundary |
| `-1` | `"-1"` | Sign preserved |
| `-12_500` | `"-12.5K"` | Sign + compact |
| `i128::MIN` | `"-170.1Ud"` | No panic, no overflow |
| `u128::MAX` | `"340.3Ud"` | No panic, no overflow |
| `0.0` | `"0"` | |
| `-0.0` | `"0"` | Negative zero suppressed |
| `-0.004` | `"0"` | Rounds to zero, sign suppressed |
| `f64::INFINITY` | `"inf"` | |
| `f64::NEG_INFINITY` | `"-inf"` | |
| `f64::NAN` | `"NaN"` | |

---

## Byte sizes

Formats byte counts using decimal (SI, 1000-based) or binary (IEC, 1024-based) units.

```rust
use humfmt::{bytes, bytes_with, BytesOptions};

assert_eq!(bytes(1536_u64).to_string(), "1.5KB");
assert_eq!(bytes_with(1536_u64, BytesOptions::new().binary()).to_string(), "1.5KiB");
```

### Defaults

| Option | Default | Meaning |
|---|---|---|
| `precision` | 1 | fractional digits for scaled values |
| `significant_digits` | none | round to N total significant digits |
| `binary` | false | SI (1000) vs IEC (1024) |
| `bits` | false | multiply by 8 and use bit units (`Mb`) |
| `rounding` | `HalfUp` | HalfUp, Floor, Ceil behaviour |
| `long_units` | false | `KB` vs ` kilobytes` |
| `decimal_separator` | `.` | decimal separator for scaled output |
| `space` | false | add a space before short unit labels |
| `fixed_precision` | false | keep trailing zeros (`1.50KB`) |
| `min_unit` | `B` | clamp minimum unit |
| `max_unit` | `EB` | clamp maximum unit |

### SI vs IEC

```rust
use humfmt::{bytes, bytes_with, BytesOptions};

assert_eq!(bytes(1000_u64).to_string(), "1KB");
assert_eq!(bytes(1_500_000_u64).to_string(), "1.5MB");

let bin = BytesOptions::new().binary();
assert_eq!(bytes_with(1024_u64, bin).to_string(), "1KiB");
assert_eq!(bytes_with(1_536_u64, bin).to_string(), "1.5KiB");
```

### Bits mode

```rust
use humfmt::{bytes_with, BytesOptions};

let opts = BytesOptions::new().bits(true);
assert_eq!(bytes_with(1000_u64, opts).to_string(), "8Kb");
assert_eq!(bytes_with(1_500_000_u64, opts).to_string(), "12Mb");

let opts_bin = BytesOptions::new().bits(true).binary();
assert_eq!(bytes_with(1024_u64, opts_bin).to_string(), "8Kib");

let opts_long = BytesOptions::new().bits(true).long_units();
assert_eq!(bytes_with(1_u64, opts_long).to_string(), "8 bits");
assert_eq!(bytes_with(125_u64, opts_long).to_string(), "1 kilobit");
```

### Unit forcing and clamping

```rust
use humfmt::{bytes_with, BytesOptions, ByteUnit};

let opts = BytesOptions::new().unit(ByteUnit::MB).precision(3);
assert_eq!(bytes_with(150_000_u64, opts).to_string(), "0.15MB");
assert_eq!(bytes_with(1_500_000_u64, opts).to_string(), "1.5MB");
assert_eq!(bytes_with(1_500_000_000_u64, opts).to_string(), "1500MB");

let opts = BytesOptions::new().min_unit(ByteUnit::KB).precision(2);
assert_eq!(bytes_with(500_u64, opts).to_string(), "0.5KB");

let opts = BytesOptions::new().max_unit(ByteUnit::GB);
assert_eq!(bytes_with(2_000_000_000_000_u64, opts).to_string(), "2000GB");
```

### Long-form labels

```rust
use humfmt::{bytes_with, BytesOptions};

let opts = BytesOptions::new().long_units();
assert_eq!(bytes_with(1_u64, opts).to_string(), "1 byte");
assert_eq!(bytes_with(1536_u64, opts).to_string(), "1.5 kilobytes");

let bin = BytesOptions::new().binary().long_units();
assert_eq!(bytes_with(1024_u64, bin).to_string(), "1 kibibyte");
```

### Custom decimal separator

```rust
use humfmt::BytesOptions;

let opts = BytesOptions::new().decimal_separator(',');
assert_eq!(humfmt::bytes_with(1536_u64, opts).to_string(), "1,5KB");
```

### Edge cases

| Input | Output | Notes |
|---:|---|---|
| `0` | `"0B"` | Zero bytes |
| `999` | `"999B"` | Below threshold |
| `1000` | `"1KB"` | SI threshold |
| `1024` | `"1KB"` (SI) / `"1KiB"` (IEC) | Threshold difference |
| `-1536` | `"-1.5KB"` | Negative supported |
| `u128::MAX` | `"...EB"` | Largest unit, no overflow |
| `999_950` | `"1MB"` | Rounds up across boundary |

---

## Percentages

Converts a ratio to a percentage: `0.423` -> `"42.3%"`.

The input is a ratio where `1.0` = `100%`. Values outside `0.0..=1.0` are
accepted and rendered as-is.

```rust
use humfmt::{percent, percent_with, PercentOptions};

assert_eq!(percent(0.0_f64).to_string(), "0%");
assert_eq!(percent(0.5_f64).to_string(), "50%");
assert_eq!(percent(1.0_f64).to_string(), "100%");
assert_eq!(percent(0.423_f64).to_string(), "42.3%");
```

### Defaults

| Option | Default | Meaning |
|---|---|---|
| `precision` | 1 | fractional digits |
| `force_sign` | false | output `+` for positive percentages |
| `fixed_precision` | false | keep trailing zeros (`42.50%`) |
| `decimal_separator` | `'.'` | decimal separator character |

### Examples

```rust
use humfmt::{percent_with, PercentOptions};

let two = PercentOptions::new().precision(2);
assert_eq!(percent_with(0.4236_f64, two).to_string(), "42.36%");

let fixed = PercentOptions::new().precision(2).fixed_precision(true);
assert_eq!(percent_with(0.5_f64, fixed).to_string(), "50.00%");

let signed = PercentOptions::new().force_sign(true);
assert_eq!(percent_with(0.42_f64, signed).to_string(), "+42%");

let comma = PercentOptions::new().precision(1).decimal_separator(',');
assert_eq!(percent_with(0.423_f64, comma).to_string(), "42,3%");
```

### Edge cases

| Input | Output | Notes |
|---:|---|---|
| `0.0` | `"0%"` | |
| `-0.0` | `"0%"` | Negative zero suppressed |
| `0.5` | `"50%"` | |
| `1.0` | `"100%"` | |
| `1.5` | `"150%"` | Above 100% accepted |
| `-0.423` | `"-42.3%"` | Negative accepted |
| `-0.0004` | `"0%"` | Rounds to zero, sign suppressed |
| `f64::NAN` | `"NaN%"` | Non-finite preserved |
| `f64::INFINITY` | `"inf%"` | Non-finite preserved |
| `f64::NEG_INFINITY` | `"-inf%"` | Non-finite preserved |

---

## Ordinals

English ordinal markers (with the standard teen exceptions).

```rust
use humfmt::ordinal;

assert_eq!(ordinal(1).to_string(), "1st");
assert_eq!(ordinal(2).to_string(), "2nd");
assert_eq!(ordinal(11).to_string(), "11th");
assert_eq!(ordinal(21).to_string(), "21st");
assert_eq!(ordinal(42).to_string(), "42nd");
assert_eq!(ordinal(103).to_string(), "103rd");
assert_eq!(ordinal(111).to_string(), "111th");
assert_eq!(ordinal(-1).to_string(), "-1st");
```

If you only need the suffix (e.g. for custom rendering pipelines):

```rust
assert_eq!(humfmt::ordinal::ordinal_suffix(21), "st");
assert_eq!(humfmt::ordinal::ordinal_suffix(11), "th");
```

---

## Durations

Compact or long-form duration formatting for `core::time::Duration`.

```rust
use core::time::Duration;
use humfmt::{duration, duration_with, DurationOptions};

assert_eq!(duration(Duration::from_secs(3661)).to_string(), "1h 1m");
assert_eq!(duration(Duration::from_secs(90061)).to_string(), "1d 1h");

let opts = DurationOptions::new().long_units().max_units(3);
assert_eq!(
    duration_with(Duration::from_secs(3665), opts).to_string(),
    "1 hour 1 minute 5 seconds"
);
```

### Defaults

| Option | Default | Meaning |
|---|---|---|
| `max_units` | 2 | maximum number of non-zero units to render |
| `long_units` | false | `h` vs ` hour` |

### Edge cases

| Input | Output | Notes |
|---:|---|---|
| `0s` | `"0s"` | Zero duration |
| `900ms` | `"900ms"` | Below 1s |
| `1500ms` | `"1s 500ms"` | Compound |
| `90s` | `"1m 30s"` | Two units (default) |
| `3661s` | `"1h 1m"` | Seconds truncated |
| `90061s` | `"1d 1h"` | Days included |

---

## Relative time

Builds on the duration formatter and appends `" ago"`.

```rust
use core::time::Duration;
use humfmt::{ago, ago_with, DurationOptions};

assert_eq!(ago(Duration::from_secs(90)).to_string(), "1m 30s ago");
assert_eq!(ago(Duration::from_secs(3661)).to_string(), "1h 1m ago");
assert_eq!(ago(Duration::ZERO).to_string(), "0s ago");

let opts = DurationOptions::new().long_units();
assert_eq!(
    ago_with(Duration::from_millis(1500), opts).to_string(),
    "1 second 500 milliseconds ago"
);
```

---

## Lists

Natural-language list formatting.

```rust
use humfmt::{list, list_with, ListOptions};

assert_eq!(list(&["red", "green", "blue"]).to_string(), "red, green, and blue");
assert_eq!(list(&["red"]).to_string(), "red");
assert_eq!(list::<&str>(&[]).to_string(), "");

assert_eq!(list(&["red", "green"]).to_string(), "red and green");

let no_oxford = list_with(
    &["red", "green", "blue"],
    ListOptions::new().no_serial_comma(),
);
assert_eq!(no_oxford.to_string(), "red, green and blue");

let plus = list_with(
    &["red", "green", "blue"],
    ListOptions::new().conjunction("plus").no_serial_comma(),
);
assert_eq!(plus.to_string(), "red, green plus blue");

let piped = list_with(
    &["red", "green", "blue"],
    ListOptions::new().separator(" | ").conjunction("&"),
);
assert_eq!(piped.to_string(), "red | green & blue");

// Any Display type works
assert_eq!(list(&[1, 2, 3]).to_string(), "1, 2, and 3");
```

### Edge cases

| Input | Output | Notes |
|---:|---|---|
| `[]` | `""` | Empty list |
| `["red"]` | `"red"` | Single item |
| `["red", "green"]` | `"red and green"` | Two items, no comma |
| `["red", "green", "blue"]` | `"red, green, and blue"` | Three items, serial comma |

The serial comma is only injected when the separator is comma-style. Custom
separators like `" | "` will not get a serial comma even if enabled.

---

## Optional integrations

Enable via feature flags:

```toml
[dependencies]
humfmt = { version = "0.6", features = ["chrono"] }
# or
humfmt = { version = "0.6", features = ["time"] }
```

### chrono

Adapts `chrono::TimeDelta` and `chrono::DateTime`:

```rust
use humfmt::chrono as humchrono;

let delta = chrono::TimeDelta::try_seconds(90).unwrap();
assert_eq!(humchrono::duration(delta).unwrap().to_string(), "1m 30s");
assert_eq!(humchrono::ago(delta).unwrap().to_string(), "1m 30s ago");

let then = chrono::DateTime::from_timestamp(0, 0).unwrap();
let now = chrono::DateTime::from_timestamp(3665, 0).unwrap();
assert_eq!(
    humchrono::ago_since(then, now).unwrap().to_string(),
    "1h 1m ago"
);
```

### time

Adapts `time::Duration` and `time::OffsetDateTime`:

```rust
use humfmt::time as humtime;

let delta = time::Duration::seconds(90);
assert_eq!(humtime::duration(delta).unwrap().to_string(), "1m 30s");

let then = time::OffsetDateTime::from_unix_timestamp(0).unwrap();
let now = time::OffsetDateTime::from_unix_timestamp(3665).unwrap();
assert_eq!(
    humtime::ago_since(then, now).unwrap().to_string(),
    "1h 1m ago"
);
```

### Checked variants

Both integrations provide `*_checked` functions that return `DurationConversionError`:

```rust
use humfmt::{chrono as humchrono, DurationConversionError};

let delta = chrono::TimeDelta::try_seconds(-5).unwrap();
assert!(matches!(
    humchrono::duration_checked(delta),
    Err(DurationConversionError::NegativeDuration)
));
```

---

## no_std

`humfmt` supports `no_std`:

```toml
[dependencies]
humfmt = { version = "0.6", default-features = false }
```

This disables the `std` feature. Optional integrations (`chrono`, `time`) can
still be enabled individually.

---

## Benchmarks

See [`BENCHMARKS.md`](../BENCHMARKS.md) for a reproducible comparison report and methodology.

The repository includes a standalone benchmark harness under `tools/benchmarks/`
that compares `humfmt` against `humansize`, `bytesize`, `byte-unit`,
`prettier-bytes`, `human_format`, `numfmt`, `humantime`, `timeago`, and others.

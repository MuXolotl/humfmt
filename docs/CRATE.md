# humfmt

Ergonomic human-readable formatting toolkit for Rust.

`humfmt` turns raw machine values into readable text. All formatters implement
`Display` and write directly into the output — no intermediate heap strings,
no hidden allocations.

## Quick start

The crate exposes two usage styles for every formatter:

1. **Free functions** — `humfmt::number(15320)` → `"15.3K"`
2. **Extension trait** — `15320.human_number()` → `"15.3K"`

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

Turns large values into short forms: `15320` → `"15.3K"`, `1_500_000` → `"1.5M"`.

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
| `locale` | English | separators, suffixes, inflection rules |

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

Values that round to exactly zero output `0` without a sign:

```rust
use humfmt::{number_with, NumberOptions};

let opts = NumberOptions::new().force_sign(true).precision(1);
assert_eq!(number_with(0.004_f64, opts).to_string(), "0");
```

### Rounding modes

```rust
use humfmt::{number_with, NumberOptions, RoundingMode};

let base = NumberOptions::new().precision(0);

// 1900 -> 1.9K -> precision 0
assert_eq!(number_with(1_900, base.rounding(RoundingMode::HalfUp)).to_string(), "2K");
assert_eq!(number_with(1_900, base.rounding(RoundingMode::Floor)).to_string(), "1K");
assert_eq!(number_with(1_900, base.rounding(RoundingMode::Ceil)).to_string(), "2K");

// Negative values: rounding direction is based on sign
// -1234 -> -1.234K at precision 2
let base2 = NumberOptions::new().precision(2);
assert_eq!(number_with(-1234, base2.rounding(RoundingMode::HalfUp)).to_string(), "-1.23K");
assert_eq!(number_with(-1234, base2.rounding(RoundingMode::Floor)).to_string(), "-1.24K");
assert_eq!(number_with(-1234, base2.rounding(RoundingMode::Ceil)).to_string(), "-1.23K");
```

Rounding may rescale across a suffix boundary:

```rust
use humfmt::{number_with, NumberOptions, RoundingMode};

let base = NumberOptions::new().precision(0);
// 999_500 -> 999.5K -> rounds to 1000K -> rescales to 1M
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
```

Combined with fixed precision:

```rust
use humfmt::{number_with, NumberOptions};

let opts = NumberOptions::new().precision(2).fixed_precision(true).long_units();
assert_eq!(number_with(1_000, opts).to_string(), "1.00 thousand");
assert_eq!(number_with(1_500, opts).to_string(), "1.50 thousand");
```

### Digit grouping separators

Separators apply only when the value is not compacted (suffix index 0):

```rust
use humfmt::{number_with, NumberOptions};

// Compacted: separators have no effect (integer part is small)
let opts = NumberOptions::new().separators(true);
assert_eq!(number_with(15_320, opts).to_string(), "15.3K");

// Unscaled: separators work
let unscaled = NumberOptions::new().compact(false).separators(true);
assert_eq!(number_with(12_345, unscaled).to_string(), "12,345");
assert_eq!(number_with(1_234_567, unscaled).to_string(), "1,234,567");

// Negative values
assert_eq!(number_with(-12_345, unscaled).to_string(), "-12,345");
assert_eq!(number_with(-1_234_567, unscaled).to_string(), "-1,234,567");
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
| `i128::MIN` | `"-170.1Dc"` | No panic, no overflow |
| `u128::MAX` | `"340.3Dc"` | No panic, no overflow |
| `0.0` | `"0"` | |
| `-0.0` | `"0"` | Negative zero suppressed |
| `-0.004` | `"0"` | Rounds to zero, sign suppressed |
| `f64::INFINITY` | `"inf"` | Locale-agnostic |
| `f64::NEG_INFINITY` | `"-inf"` | Locale-agnostic |
| `f64::NAN` | `"NaN"` | Locale-agnostic |

Float just below the rescale boundary:

```rust
use humfmt::{number_with, NumberOptions};

// 999_449 at precision=1 stays at 999.9K (does not round to 1M)
assert_eq!(number_with(999_449.0_f64, NumberOptions::new().precision(1)).to_string(), "999.9K");
```

Float scaling matches integer scaling for round values:

```rust
use humfmt::number;

assert_eq!(number(1_000.0_f64).to_string(), "1K");
assert_eq!(number(1_000_000.0_f64).to_string(), "1M");
assert_eq!(number(1_000_000_000.0_f64).to_string(), "1B");
assert_eq!(number(1_000_000_000_000.0_f64).to_string(), "1T");
```

Sign symmetry holds for all float values:

```rust
use humfmt::number;

assert_eq!(number(-1.5_f64).to_string(), "-1.5");
assert_eq!(number(-1_500.0_f64).to_string(), "-1.5K");
assert_eq!(number(-1_000_000.0_f64).to_string(), "-1.5M");
```

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

// SI (decimal, 1000-based) — default
assert_eq!(bytes(1000_u64).to_string(), "1KB");
assert_eq!(bytes(1_500_000_u64).to_string(), "1.5MB");

// IEC (binary, 1024-based)
let bin = BytesOptions::new().binary();
assert_eq!(bytes_with(1024_u64, bin).to_string(), "1KiB");
assert_eq!(bytes_with(1_536_u64, bin).to_string(), "1.5KiB");
```

### Bits mode

Multiply by 8 and use bit units. Useful for network speeds and bandwidth:

```rust
use humfmt::{bytes_with, BytesOptions};

// Decimal bits
let opts = BytesOptions::new().bits(true);
assert_eq!(bytes_with(1000_u64, opts).to_string(), "8Kb");
assert_eq!(bytes_with(1_500_000_u64, opts).to_string(), "12Mb");

// Binary bits
let opts_bin = BytesOptions::new().bits(true).binary();
assert_eq!(bytes_with(1024_u64, opts_bin).to_string(), "8Kib");

// Long-form bit units
let opts_long = BytesOptions::new().bits(true).long_units();
assert_eq!(bytes_with(1_u64, opts_long).to_string(), "8 bits");
assert_eq!(bytes_with(125_u64, opts_long).to_string(), "1 kilobit");
```

### Unit forcing and clamping

Force a specific unit:

```rust
use humfmt::{bytes_with, BytesOptions, ByteUnit};

let opts = BytesOptions::new().unit(ByteUnit::MB).precision(3);
assert_eq!(bytes_with(150_000_u64, opts).to_string(), "0.15MB");
assert_eq!(bytes_with(1_500_000_u64, opts).to_string(), "1.5MB");
assert_eq!(bytes_with(1_500_000_000_u64, opts).to_string(), "1500MB");
```

Clamp minimum unit (500 bytes forced to KB):

```rust
use humfmt::{bytes_with, BytesOptions, ByteUnit};

let opts = BytesOptions::new().min_unit(ByteUnit::KB).precision(2);
assert_eq!(bytes_with(500_u64, opts).to_string(), "0.5KB");
```

Clamp maximum unit (2 TB stays in GB):

```rust
use humfmt::{bytes_with, BytesOptions, ByteUnit};

let opts = BytesOptions::new().max_unit(ByteUnit::GB);
assert_eq!(bytes_with(2_000_000_000_000_u64, opts).to_string(), "2000GB");
```

If min > max, it safely normalizes (min wins):

```rust
use humfmt::{bytes_with, BytesOptions, ByteUnit};

let opts = BytesOptions::new()
    .min_unit(ByteUnit::GB)
    .max_unit(ByteUnit::KB);
assert_eq!(bytes_with(1_500_000_000_000_u64, opts).to_string(), "1500GB");
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

### Space before short units

```rust
use humfmt::{bytes_with, BytesOptions};

let opts = BytesOptions::new().space(true);
assert_eq!(bytes_with(999_u64, opts).to_string(), "999 B");
assert_eq!(bytes_with(1536_u64, opts).to_string(), "1.5 KB");

// Binary with space
let bin = BytesOptions::new().binary().precision(2).space(true);
assert_eq!(bytes_with(1536_u64, bin).to_string(), "1.5 KiB");
```

### Locale-aware decimal separator

Byte unit labels are currently English-only, but the decimal separator is configurable:

```rust
use humfmt::BytesOptions;

// Direct override
let opts = BytesOptions::new().decimal_separator(',');
assert_eq!(humfmt::bytes_with(1536_u64, opts).to_string(), "1,5KB");

// From locale
use humfmt::locale::CustomLocale;
let locale = CustomLocale::english().decimal_separator(',');
let opts = BytesOptions::new().locale(locale);
assert_eq!(humfmt::bytes_with(1536_u64, opts).to_string(), "1,5KB");
```

### Significant digits

```rust
use humfmt::{bytes_with, BytesOptions};

let opts = BytesOptions::new().significant_digits(3);
assert_eq!(bytes_with(1234_u64, opts).to_string(), "1.23KB");
assert_eq!(bytes_with(12345_u64, opts).to_string(), "12.3KB");
assert_eq!(bytes_with(123456_u64, opts).to_string(), "123KB");
```

### Rounding modes

```rust
use humfmt::{bytes_with, BytesOptions, RoundingMode};

let base = BytesOptions::new().precision(0);

// 1500 B = 1.5 KB, precision 0
assert_eq!(bytes_with(1500_u64, base.rounding(RoundingMode::HalfUp)).to_string(), "2KB");
assert_eq!(bytes_with(1500_u64, base.rounding(RoundingMode::Floor)).to_string(), "1KB");
assert_eq!(bytes_with(1500_u64, base.rounding(RoundingMode::Ceil)).to_string(), "2KB");

// Negative values
assert_eq!(bytes_with(-1500_i64, base.rounding(RoundingMode::HalfUp)).to_string(), "-2KB");
assert_eq!(bytes_with(-1500_i64, base.rounding(RoundingMode::Floor)).to_string(), "-2KB");
assert_eq!(bytes_with(-1500_i64, base.rounding(RoundingMode::Ceil)).to_string(), "-1KB");
```

### Fixed precision

```rust
use humfmt::{bytes_with, BytesOptions};

// Trimmed (default)
let trimmed = BytesOptions::new().binary().precision(2).space(true);
assert_eq!(bytes_with(1536_u64, trimmed).to_string(), "1.5 KiB");
assert_eq!(bytes_with(1024_u64, trimmed).to_string(), "1 KiB");

// Fixed (pads trailing zeros)
let fixed = BytesOptions::new().binary().precision(2).space(true).fixed_precision(true);
assert_eq!(bytes_with(1536_u64, fixed).to_string(), "1.50 KiB");
assert_eq!(bytes_with(1024_u64, fixed).to_string(), "1.00 KiB");
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

Rounding up across a unit boundary:

```rust
use humfmt::bytes;

// 999_950 B = 999.95 KB -> rounds to 1000 KB -> rescales to 1 MB
assert_eq!(bytes(999_950_u64).to_string(), "1MB");
```

---

## Percentages

Converts a ratio to a percentage: `0.423` → `"42.3%"`.

The input is a ratio where `1.0` = `100%`. Values outside `0.0..=1.0` are accepted
and rendered as-is.

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
| `locale` | English | decimal separator |

### Precision

```rust
use humfmt::{percent_with, PercentOptions};

assert_eq!(percent_with(0.4236_f64, PercentOptions::new().precision(2)).to_string(), "42.36%");
assert_eq!(percent_with(0.424_f64, PercentOptions::new().precision(0)).to_string(), "42%");
assert_eq!(percent_with(0.425_f64, PercentOptions::new().precision(0)).to_string(), "43%");
```

### Fixed precision

```rust
use humfmt::{percent_with, PercentOptions};

// Trimmed (default)
let trimmed = PercentOptions::new().precision(2);
assert_eq!(percent_with(0.5_f64, trimmed).to_string(), "50%");
assert_eq!(percent_with(0.425_f64, trimmed).to_string(), "42.5%");

// Fixed (pads trailing zeros)
let fixed = PercentOptions::new().precision(2).fixed_precision(true);
assert_eq!(percent_with(0.5_f64, fixed).to_string(), "50.00%");
assert_eq!(percent_with(0.425_f64, fixed).to_string(), "42.50%");
```

### Forced sign

```rust
use humfmt::{percent_with, PercentOptions};

let opts = PercentOptions::new().force_sign(true);
assert_eq!(percent_with(0.42_f64, opts).to_string(), "+42%");
assert_eq!(percent_with(-0.42_f64, opts).to_string(), "-42%");
assert_eq!(percent_with(0.0_f64, opts).to_string(), "0%");
```

Values that round to exactly zero output `0%` without a sign:

```rust
use humfmt::{percent_with, PercentOptions};

let opts = PercentOptions::new().force_sign(true).precision(1);
// 0.0004 * 100 = 0.04% -> rounds to 0.0% -> displays as "0%"
assert_eq!(percent_with(0.0004_f64, opts).to_string(), "0%");
```

### Values above 100%

```rust
use humfmt::percent;

assert_eq!(percent(1.5_f64).to_string(), "150%");
assert_eq!(percent(2.0_f64).to_string(), "200%");
```

### Locale-aware decimal separator

```rust
use humfmt::{percent_with, PercentOptions};
use humfmt::locale::CustomLocale;

let locale = CustomLocale::english().decimal_separator(',');
let opts = PercentOptions::new().precision(1).locale(locale);
assert_eq!(percent_with(0.423_f64, opts).to_string(), "42,3%");
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

Locale-aware ordinal markers.

```rust
use humfmt::{ordinal, ordinal_with};

assert_eq!(ordinal(1).to_string(), "1st");
assert_eq!(ordinal(2).to_string(), "2nd");
assert_eq!(ordinal(3).to_string(), "3rd");
assert_eq!(ordinal(4).to_string(), "4th");
assert_eq!(ordinal(11).to_string(), "11th");
assert_eq!(ordinal(12).to_string(), "12th");
assert_eq!(ordinal(13).to_string(), "13th");
assert_eq!(ordinal(21).to_string(), "21st");
assert_eq!(ordinal(42).to_string(), "42nd");
assert_eq!(ordinal(103).to_string(), "103rd");
assert_eq!(ordinal(111).to_string(), "111th");
```

### Negative values

```rust
use humfmt::ordinal;

assert_eq!(ordinal(-1).to_string(), "-1st");
assert_eq!(ordinal(-12).to_string(), "-12th");
```

### Edge cases by locale

| Input | English | Russian | Polish |
|---:|---|---|---|
| `1` | `"1st"` | `"1-й"` | `"1."` |
| `2` | `"2nd"` | `"2-й"` | `"2."` |
| `3` | `"3rd"` | `"3-й"` | `"3."` |
| `11` | `"11th"` | `"11-й"` | `"11."` |
| `12` | `"12th"` | `"12-й"` | `"12."` |
| `21` | `"21st"` | `"21-й"` | `"21."` |
| `42` | `"42nd"` | `"42-й"` | `"42."` |
| `103` | `"103rd"` | `"103-й"` | `"103."` |
| `111` | `"111th"` | `"111-й"` | `"111."` |
| `-1` | `"-1st"` | `"-1-й"` | `"-1."` |

**Note:** Russian ordinals always return `-й` (masculine). The library has no concept
of grammatical gender since it only receives a number.

---

## Durations

Compact or long-form duration formatting for `core::time::Duration`.

```rust
use core::time::Duration;
use humfmt::{duration, duration_with, DurationOptions};

assert_eq!(duration(Duration::from_secs(3661)).to_string(), "1h 1m");
assert_eq!(duration(Duration::from_secs(90061)).to_string(), "1d 1h");
```

### Defaults

| Option | Default | Meaning |
|---|---|---|
| `max_units` | 2 | maximum number of non-zero units to render |
| `long_units` | false | `h` vs ` hour` |
| `locale` | English | affects unit labels and wording |

### Sub-second durations

```rust
use core::time::Duration;
use humfmt::duration;

assert_eq!(duration(Duration::from_millis(1500)).to_string(), "1s 500ms");
assert_eq!(duration(Duration::from_nanos(1_500)).to_string(), "1us 500ns");
```

### Long-form labels

```rust
use core::time::Duration;
use humfmt::{duration_with, DurationOptions};

let opts = DurationOptions::new().long_units();
assert_eq!(
    duration_with(Duration::from_millis(1500), opts).to_string(),
    "1 second 500 milliseconds"
);
assert_eq!(
    duration_with(Duration::from_secs(90), opts).to_string(),
    "1 minute 30 seconds"
);
```

### Max units

```rust
use core::time::Duration;
use humfmt::{duration_with, DurationOptions};

// Default: 2 units
assert_eq!(
    duration_with(Duration::from_secs(3665), DurationOptions::new()).to_string(),
    "1h 1m"
);

// 3 units
assert_eq!(
    duration_with(Duration::from_secs(3665), DurationOptions::new().max_units(3)).to_string(),
    "1h 1m 5s"
);

// 1 unit
assert_eq!(
    duration_with(Duration::from_secs(3665), DurationOptions::new().max_units(1)).to_string(),
    "1h"
);
```

Up to 7 units (all supported time units):

```rust
use core::time::Duration;
use humfmt::{duration_with, DurationOptions};

let value = Duration::from_nanos(1_001_001_001);
let opts = DurationOptions::new().max_units(7).long_units();
assert_eq!(
    duration_with(value, opts).to_string(),
    "1 second 1 millisecond 1 microsecond 1 nanosecond"
);
```

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

Builds on the duration formatter and appends the locale-specific "ago" word.

```rust
use core::time::Duration;
use humfmt::{ago, ago_with, DurationOptions};

assert_eq!(ago(Duration::from_secs(90)).to_string(), "1m 30s ago");
assert_eq!(ago(Duration::from_secs(3661)).to_string(), "1h 1m ago");
assert_eq!(ago(Duration::ZERO).to_string(), "0s ago");
```

Uses the same `DurationOptions` as the duration formatter:

```rust
use core::time::Duration;
use humfmt::{ago_with, DurationOptions};

let opts = DurationOptions::new().long_units();
assert_eq!(
    ago_with(Duration::from_millis(1500), opts).to_string(),
    "1 second 500 milliseconds ago"
);

let opts3 = DurationOptions::new().max_units(3);
assert_eq!(
    ago_with(Duration::from_secs(3665), opts3).to_string(),
    "1h 1m 5s ago"
);
```

### Locale examples

```rust
use core::time::Duration;
use humfmt::{ago_with, DurationOptions};
use humfmt::locale::Russian;

let opts = DurationOptions::new().locale(Russian).long_units().max_units(3);
assert_eq!(
    ago_with(Duration::from_secs(90), opts).to_string(),
    "1 минута 30 секунд назад"
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
```

### Two items

```rust
use humfmt::list;

assert_eq!(list(&["red", "green"]).to_string(), "red and green");
```

### Disable serial comma

```rust
use humfmt::{list_with, ListOptions};

let out = list_with(
    &["red", "green", "blue"],
    ListOptions::new().no_serial_comma(),
);
assert_eq!(out.to_string(), "red, green and blue");
```

### Custom conjunction

```rust
use humfmt::{list_with, ListOptions};

let out = list_with(
    &["red", "green", "blue"],
    ListOptions::new().conjunction("plus"),
);
assert_eq!(out.to_string(), "red, green, plus blue");
```

### Non-string items

Any type that implements `Display` works:

```rust
use humfmt::list;

assert_eq!(list(&[1, 2, 3]).to_string(), "1, 2, and 3");
```

### Edge cases

| Input | Output | Notes |
|---:|---|---|
| `[]` | `""` | Empty list |
| `["red"]` | `"red"` | Single item |
| `["red", "green"]` | `"red and green"` | Two items, no comma |
| `["red", "green", "blue"]` | `"red, green, and blue"` | Three items, serial comma |

---

## Locales

Built-in locale packs:

- **English** (default)
- **Russian** (feature `russian`)
- **Polish** (feature `polish`)

### Features matrix

| Feature | English | Russian | Polish | CustomLocale |
|---|:---:|:---:|:---:|:---:|
| Compact suffixes | K/M/B/... | тыс./млн/... | tys./mln/... | custom |
| Long-form inflection | no | yes | yes | custom fn |
| Decimal separator | `.` | `,` | `,` | custom |
| Group separator | `,` | ` ` | ` ` | custom |
| List conjunction | "and" | "и" | "i" | custom |
| Serial comma | yes | no | no | custom |
| Ordinal suffix | st/nd/rd/th | -й | . | custom fn |
| Duration labels | s/m/h/... | с/м/ч/... | s/min/godz./... | custom fn |
| Ago word | "ago" | "назад" | "temu" | custom |

### Russian examples

```rust
use core::time::Duration;
use humfmt::{ago_with, duration_with, list_with, number_with, NumberOptions, DurationOptions, ListOptions};
use humfmt::locale::Russian;

// Numbers
assert_eq!(number_with(15_320, NumberOptions::new().locale(Russian)).to_string(), "15,3 тыс.");
assert_eq!(number_with(1_500_000, NumberOptions::new().locale(Russian)).to_string(), "1,5 млн");

// Long-form with inflection
let long = NumberOptions::new().locale(Russian).long_units();
assert_eq!(number_with(1_000, long).to_string(), "1 тысяча");
assert_eq!(number_with(2_000, long).to_string(), "2 тысячи");
assert_eq!(number_with(5_000, long).to_string(), "5 тысяч");
assert_eq!(number_with(11_000, long).to_string(), "11 тысяч");
assert_eq!(number_with(21_000, long).to_string(), "21 тысяча");

// Fractional uses genitive singular
assert_eq!(number_with(1_500, long).to_string(), "1,5 тысячи");

// Separators
let sep = NumberOptions::new().locale(Russian).compact(false).separators(true);
assert_eq!(number_with(1_234_567, sep).to_string(), "1 234 567");

// Duration
let dur_opts = DurationOptions::new().locale(Russian).long_units().max_units(3);
assert_eq!(
    duration_with(Duration::from_secs(3665), dur_opts).to_string(),
    "1 час 1 минута 5 секунд"
);

// Relative time
assert_eq!(
    ago_with(Duration::from_secs(90), dur_opts).to_string(),
    "1 минута 30 секунд назад"
);

// List
let items = list_with(
    &["яблоки", "груши", "сливы"],
    ListOptions::new().locale(Russian),
);
assert_eq!(items.to_string(), "яблоки, груши и сливы");
```

### Polish examples

```rust
use core::time::Duration;
use humfmt::{ago_with, duration_with, number_with, ordinal_with, NumberOptions, DurationOptions};
use humfmt::locale::Polish;

// Numbers
assert_eq!(number_with(15_320, NumberOptions::new().locale(Polish)).to_string(), "15,3 tys.");
assert_eq!(number_with(1_500_000, NumberOptions::new().locale(Polish)).to_string(), "1,5 mln");

// Long-form with inflection (CLDR-aligned)
let long = NumberOptions::new().locale(Polish).long_units();
assert_eq!(number_with(1_000, long).to_string(), "1 tysiąc");
assert_eq!(number_with(2_000, long).to_string(), "2 tysiące");
assert_eq!(number_with(5_000, long).to_string(), "5 tysięcy");
assert_eq!(number_with(12_000, long).to_string(), "12 tysięcy");  // teen exception
assert_eq!(number_with(22_000, long).to_string(), "22 tysiące");

// Fractional uses special form
assert_eq!(number_with(1_500, long).to_string(), "1,5 tysiąca");

// Duration
let dur_opts = DurationOptions::new().locale(Polish).long_units().max_units(3);
assert_eq!(
    duration_with(Duration::from_secs(3665), dur_opts).to_string(),
    "1 godzina 1 minuta 5 sekund"
);

// Relative time
assert_eq!(
    ago_with(Duration::from_secs(90), dur_opts).to_string(),
    "1 minuta 30 sekund temu"
);

// Ordinals
assert_eq!(ordinal_with(21, Polish).to_string(), "21.");
```

### Custom locale

Override suffixes, separators, duration units, ordinals, and list style:

```rust
use core::time::Duration;
use humfmt::{ago_with, number_with, ordinal_with, DurationOptions, NumberOptions};
use humfmt::locale::{CustomLocale, DurationUnit};

fn custom_duration_unit(unit: DurationUnit, count: u128, long: bool) -> &'static str {
    if !long {
        return match unit {
            DurationUnit::Minute => "m",
            DurationUnit::Second => "s",
            _ => "?",
        };
    }

    match unit {
        DurationUnit::Minute if count == 1 => "tick",
        DurationUnit::Minute => "ticks",
        DurationUnit::Second if count == 1 => "tock",
        DurationUnit::Second => "tocks",
        _ => "units",
    }
}

fn custom_ordinal(_: u128) -> &'static str {
    "o"
}

let locale = CustomLocale::english()
    .short_suffix(1, "k")
    .separators(',', '.')
    .list_separator(" | ")
    .duration_unit_fn(custom_duration_unit)
    .ordinal_suffix_fn(custom_ordinal)
    .ago_word("back");

assert_eq!(
    number_with(15_320, NumberOptions::new().locale(locale)).to_string(),
    "15,3k"
);

assert_eq!(ordinal_with(7, locale).to_string(), "7o");

let rel = ago_with(
    Duration::from_secs(90),
    DurationOptions::new().locale(locale).long_units(),
);
assert_eq!(rel.to_string(), "1 tick 30 tocks back");
```

---

## Optional integrations

Enable via feature flags:

```toml
[dependencies]
humfmt = { version = "0.5", features = ["chrono"] }
# or
humfmt = { version = "0.5", features = ["time"] }
```

### chrono

Adapts `chrono::TimeDelta` and `chrono::DateTime`:

```rust
use humfmt::chrono as humchrono;

let delta = chrono::TimeDelta::try_seconds(90).unwrap();
assert_eq!(humchrono::duration(delta).unwrap().to_string(), "1m 30s");
assert_eq!(humchrono::ago(delta).unwrap().to_string(), "1m 30s ago");

// From datetimes
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
humfmt = { version = "0.5", default-features = false }
```

This disables the `std` feature and the `english` locale (which depends on `std`).
You can re-enable specific locales or integrations individually.

---

## Benchmarks

See [`BENCHMARKS.md`](../BENCHMARKS.md) for a reproducible comparison report and methodology.

The repository includes a standalone benchmark harness under `tools/benchmarks/`
that compares `humfmt` against `humansize`, `bytesize`, `byte-unit`, `prettier-bytes`,
`human_format`, `humantime`, `timeago`, and others.

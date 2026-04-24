# humfmt

Ergonomic human-readable formatting toolkit for Rust.

`humfmt` provides tiny, intuitive, zero-friction formatters for converting raw machine values into human-friendly strings.

## Goals

- Dead simple API
- Human-readable output
- Low allocation overhead
- `no_std` friendly core
- Optional localization support
- Clean extension trait sugar

## Planned formatters

- Compact numbers (`15.3K`, `2.1M`)
- Byte sizes (`1 MB`, `2.4 GiB`)
- Durations (`1h 5m 3s`)
- Relative time (`3 minutes ago`)
- Ordinals (`21st`)
- Human-readable lists

## Example API

```rust
use humfmt::Humanize;

println!("{}", humfmt::number(15320));
println!("{}", humfmt::bytes(1048576));
println!("{}", 3661.human_duration());
println!("{}", 21.human_ordinal());
````

## Status

Early development — API is being designed carefully before first stable release.

## License

MIT
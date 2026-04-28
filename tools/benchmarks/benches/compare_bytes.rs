//! Byte-formatting comparison benchmarks.
//!
//! Each crate is benchmarked against the same input values where possible.
//! Crate-level limitations are called out explicitly in comments so the
//! results can be interpreted fairly:
//!
//!   - prettier-bytes: u64 only, fixed 2-decimal precision, no negatives,
//!                     no long units, no locale, no binary/SI toggle at format-time.
//!   - bytesize:       u64 only, no negative values.
//!   - byte-unit:      u64 / u128 depending on feature flags; heavier runtime cost.
//!   - humfmt:         i128 / u128 full range, configurable precision, long units,
//!                     locale-aware, binary and decimal standards.

use std::fmt::Write as _;

use byte_unit::Byte;
use bytesize::ByteSize;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use humfmt::{bytes, bytes_with, BytesOptions};
use prettier_bytes::{ByteFormatter, Standard, Unit};

// Shared values that fit within u64 — used for all-crate fair comparisons.
const VALUES_U64: [u64; 8] = [
    0,
    512,
    1_536,
    65_536,
    1_048_576,
    25_000_000,
    9_876_543_210,
    u32::MAX as u64,
];

// Extended values that exceed u64::MAX — only humfmt can handle these.
// prettier-bytes and bytesize are excluded from this group.
const VALUES_U128_EXTENDED: [u128; 4] = [
    u64::MAX as u128 + 1,
    1_000_000_000_000_000_000_000_u128,    // ~1 ZB (zettabyte range)
    u128::MAX / 2,
    u128::MAX,
];

// Negative values — only humfmt supports these natively.
const VALUES_NEGATIVE: [i64; 4] = [-512, -1_536, -1_048_576, -9_876_543_210];

fn bench_bytes_allocating(c: &mut Criterion) {
    let mut group = c.benchmark_group("bytes/allocating");

    let humfmt_opts = BytesOptions::new().precision(2);

    // prettier-bytes is configured to match humfmt defaults as closely as possible:
    // SI standard, Bytes unit, no space (humfmt also has no space in short mode).
    let prettier = ByteFormatter::new()
        .standard(Standard::SI)
        .unit(Unit::Bytes)
        .space(false);

    // --- All-crate comparison (u64 inputs, SI/decimal) ---

    group.bench_function("humfmt/u64/to_string", |b| {
        b.iter(|| {
            for &v in &VALUES_U64 {
                black_box(bytes_with(black_box(v), humfmt_opts).to_string());
            }
        })
    });

    group.bench_function("bytesize/u64/display_si/to_string", |b| {
        b.iter(|| {
            for &v in &VALUES_U64 {
                let s = ByteSize::b(black_box(v)).display().si().to_string();
                black_box(s);
            }
        })
    });

    group.bench_function("byte_unit/u64/alt_format/#.2", |b| {
        b.iter(|| {
            for &v in &VALUES_U64 {
                let byte = Byte::from_u64(black_box(v));
                let s = format!("{byte:#.2}");
                black_box(s);
            }
        })
    });

    // prettier-bytes: u64 ONLY — cannot be used with u128 or negative values.
    group.bench_function("prettier_bytes/u64/to_string", |b| {
        b.iter(|| {
            for &v in &VALUES_U64 {
                let s = prettier.format(black_box(v)).to_string();
                black_box(s);
            }
        })
    });

    // --- humfmt-only extended range (u128 > u64::MAX) ---
    // prettier-bytes, bytesize, and byte-unit (default config) cannot participate.

    group.bench_function("humfmt/u128_extended/to_string", |b| {
        b.iter(|| {
            for &v in &VALUES_U128_EXTENDED {
                black_box(bytes_with(black_box(v), humfmt_opts).to_string());
            }
        })
    });

    // --- humfmt-only negative values ---
    // No other benchmarked crate supports signed byte values.

    group.bench_function("humfmt/negative_i64/to_string", |b| {
        b.iter(|| {
            for &v in &VALUES_NEGATIVE {
                black_box(bytes(black_box(v)).to_string());
            }
        })
    });

    group.finish();
}

fn bench_bytes_reused_buffer(c: &mut Criterion) {
    let mut group = c.benchmark_group("bytes/reused_buffer");

    let humfmt_opts = BytesOptions::new().precision(2);
    let prettier = ByteFormatter::new()
        .standard(Standard::SI)
        .unit(Unit::Bytes)
        .space(false);

    // We benchmark a single representative buffer capacity (32 bytes) to keep
    // the report concise. The capacity has negligible effect on measured time
    // because all crates write well under 32 bytes per value.
    let cap = 32usize;

    group.bench_with_input(BenchmarkId::new("humfmt/u64/write", cap), &cap, |b, &cap| {
        let mut out = String::with_capacity(cap);
        b.iter(|| {
            for &v in &VALUES_U64 {
                out.clear();
                write!(&mut out, "{}", bytes_with(black_box(v), humfmt_opts)).unwrap();
                black_box(&out);
            }
        })
    });

    group.bench_with_input(
        BenchmarkId::new("bytesize/u64/write_display_si", cap),
        &cap,
        |b, &cap| {
            let mut out = String::with_capacity(cap);
            b.iter(|| {
                for &v in &VALUES_U64 {
                    out.clear();
                    let disp = ByteSize::b(black_box(v)).display().si();
                    write!(&mut out, "{disp}").unwrap();
                    black_box(&out);
                }
            })
        },
    );

    group.bench_with_input(
        BenchmarkId::new("byte_unit/u64/write_alt_format/#.2", cap),
        &cap,
        |b, &cap| {
            let mut out = String::with_capacity(cap);
            b.iter(|| {
                for &v in &VALUES_U64 {
                    out.clear();
                    let byte = Byte::from_u64(black_box(v));
                    write!(&mut out, "{byte:#.2}").unwrap();
                    black_box(&out);
                }
            })
        },
    );

    // prettier-bytes: u64 ONLY.
    group.bench_with_input(
        BenchmarkId::new("prettier_bytes/u64/write", cap),
        &cap,
        |b, &cap| {
            let mut out = String::with_capacity(cap);
            b.iter(|| {
                for &v in &VALUES_U64 {
                    out.clear();
                    let formatted = prettier.format(black_box(v));
                    write!(&mut out, "{formatted}").unwrap();
                    black_box(&out);
                }
            })
        },
    );

    group.finish();
}

criterion_group!(benches, bench_bytes_allocating, bench_bytes_reused_buffer);
criterion_main!(benches);
//! Compact-number formatting comparison benchmarks.
//!
//! Crates under comparison and their key properties:
//!
//!   - humfmt:       all integer primitives + f32/f64, locale-aware (EN/RU/PL),
//!                   long/short suffixes, configurable precision, no_std compatible,
//!                   writes via Display (zero-alloc path available).
//!   - human_format: f64 only, English only, configurable decimals and separator,
//!                   always returns an owned String.
//!
//! Note: no other crate on crates.io produces compact "K/M/B" style output
//! comparable to humfmt. human-repr's human_count and readable's Unsigned
//! produce grouped digits ("1,000") rather than compact suffixes ("1K") and
//! are therefore not included in this comparison.
//!
//! Groups:
//!   - numbers/allocating         — to_string(), representative mixed inputs
//!   - numbers/allocating_int     — to_string(), u64-only inputs (apples-to-apples)
//!   - numbers/allocating_float   — to_string(), f64 inputs (humfmt + human_format)
//!   - numbers/reused_buffer      — write! into pre-allocated String, u64 inputs
//!   - numbers/locale             — locale overhead (humfmt EN / RU / PL)

use std::fmt::Write as _;

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use humfmt::{number, number_with, NumberOptions};

// Mixed inputs: positive and negative, below and above each suffix boundary.
// Used for the default allocating group.
const VALUES_MIXED_I64: [i64; 10] = [
    0, 12, 999, 1_250, 15_320, 999_950, 1_500_000, 75_000_000, -12_500, -1_000_000,
];

// Unsigned-only inputs for apples-to-apples comparison.
// human_format only accepts f64; we cast from u64 so all crates see the same values.
const VALUES_U64: [u64; 8] = [
    0,
    999,
    1_250,
    15_320,
    999_950,
    1_500_000,
    75_000_000,
    9_876_543_210,
];

// Float inputs: includes negative and fractional values.
// Only humfmt and human_format participate here.
const VALUES_F64: [f64; 8] = [
    0.0,
    999.9,
    1_250.5,
    15_320.0,
    999_950.0,
    1_500_000.0,
    75_000_000.0,
    -12_500.5,
];

fn bench_numbers_allocating(c: &mut Criterion) {
    let mut group = c.benchmark_group("numbers/allocating");

    // humfmt default (precision=1).
    group.bench_function("humfmt/i64/default", |b| {
        b.iter(|| {
            for &v in &VALUES_MIXED_I64 {
                black_box(number(black_box(v)).to_string());
            }
        })
    });

    // humfmt precision=2 — matches human_format decimals(2) for fair comparison.
    let humfmt_p2 = NumberOptions::new().precision(2);
    group.bench_function("humfmt/i64/precision2", |b| {
        b.iter(|| {
            for &v in &VALUES_MIXED_I64 {
                black_box(number_with(black_box(v), humfmt_p2).to_string());
            }
        })
    });

    // human_format: f64 only, precision=2.
    // We cast i64 → f64 to feed it the same values.
    let mut hf = human_format::Formatter::new();
    hf.with_decimals(2).with_separator("");
    group.bench_function("human_format/f64/precision2", |b| {
        b.iter(|| {
            for &v in &VALUES_MIXED_I64 {
                black_box(hf.format(black_box(v as f64)));
            }
        })
    });

    group.finish();
}

fn bench_numbers_allocating_int(c: &mut Criterion) {
    let mut group = c.benchmark_group("numbers/allocating_int");

    // u64 inputs only — closest to apples-to-apples since human_format is f64-only
    // and we cast. All use precision=1 to match humfmt default.
    let humfmt_default = NumberOptions::new();
    let humfmt_p2 = NumberOptions::new().precision(2);

    let mut hf1 = human_format::Formatter::new();
    hf1.with_decimals(1).with_separator("");

    let mut hf2 = human_format::Formatter::new();
    hf2.with_decimals(2).with_separator("");

    group.bench_function("humfmt/u64/precision1", |b| {
        b.iter(|| {
            for &v in &VALUES_U64 {
                black_box(number_with(black_box(v), humfmt_default).to_string());
            }
        })
    });

    group.bench_function("humfmt/u64/precision2", |b| {
        b.iter(|| {
            for &v in &VALUES_U64 {
                black_box(number_with(black_box(v), humfmt_p2).to_string());
            }
        })
    });

    group.bench_function("human_format/u64_as_f64/precision1", |b| {
        b.iter(|| {
            for &v in &VALUES_U64 {
                black_box(hf1.format(black_box(v as f64)));
            }
        })
    });

    group.bench_function("human_format/u64_as_f64/precision2", |b| {
        b.iter(|| {
            for &v in &VALUES_U64 {
                black_box(hf2.format(black_box(v as f64)));
            }
        })
    });

    group.finish();
}

fn bench_numbers_allocating_float(c: &mut Criterion) {
    let mut group = c.benchmark_group("numbers/allocating_float");

    // Float path only. human_format accepts f64 natively.
    // humfmt accepts f32 and f64.
    let humfmt_p2 = NumberOptions::new().precision(2);

    let mut hf = human_format::Formatter::new();
    hf.with_decimals(2).with_separator("");

    group.bench_function("humfmt/f64/precision2", |b| {
        b.iter(|| {
            for &v in &VALUES_F64 {
                black_box(number_with(black_box(v), humfmt_p2).to_string());
            }
        })
    });

    group.bench_function("human_format/f64/precision2", |b| {
        b.iter(|| {
            for &v in &VALUES_F64 {
                black_box(hf.format(black_box(v)));
            }
        })
    });

    group.finish();
}

fn bench_numbers_reused_buffer(c: &mut Criterion) {
    let mut group = c.benchmark_group("numbers/reused_buffer");

    // write! into a pre-allocated String — the zero-alloc path.
    // human_format always returns String; we push_str it into the buffer
    // to measure the cost of generating the string regardless.
    let humfmt_default = NumberOptions::new();
    let humfmt_p2 = NumberOptions::new().precision(2);

    let mut hf = human_format::Formatter::new();
    hf.with_decimals(2).with_separator("");

    let cap = 32usize;

    group.bench_with_input(
        BenchmarkId::new("humfmt/u64/precision1/write", cap),
        &cap,
        |b, &cap| {
            let mut out = String::with_capacity(cap);
            b.iter(|| {
                for &v in &VALUES_U64 {
                    out.clear();
                    write!(&mut out, "{}", number_with(black_box(v), humfmt_default)).unwrap();
                    black_box(&out);
                }
            })
        },
    );

    group.bench_with_input(
        BenchmarkId::new("humfmt/u64/precision2/write", cap),
        &cap,
        |b, &cap| {
            let mut out = String::with_capacity(cap);
            b.iter(|| {
                for &v in &VALUES_U64 {
                    out.clear();
                    write!(&mut out, "{}", number_with(black_box(v), humfmt_p2)).unwrap();
                    black_box(&out);
                }
            })
        },
    );

    group.bench_with_input(
        BenchmarkId::new("human_format/u64_as_f64/precision2/write", cap),
        &cap,
        |b, &cap| {
            let mut out = String::with_capacity(cap);
            b.iter(|| {
                for &v in &VALUES_U64 {
                    out.clear();
                    // human_format has no Display impl — push_str is the only option.
                    out.push_str(&hf.format(black_box(v as f64)));
                    black_box(&out);
                }
            })
        },
    );

    group.finish();
}

fn bench_numbers_locale(c: &mut Criterion) {
    let mut group = c.benchmark_group("numbers/locale");

    // Measures the overhead of locale-aware formatting compared to English baseline.
    // Russian and Polish require plural form selection based on the rendered value.
    let en_short = NumberOptions::new();
    let en_long = NumberOptions::new().long_units();
    let ru_short = NumberOptions::new().locale(humfmt::locale::Russian);
    let ru_long = NumberOptions::new()
        .locale(humfmt::locale::Russian)
        .long_units();
    let pl_short = NumberOptions::new().locale(humfmt::locale::Polish);
    let pl_long = NumberOptions::new()
        .locale(humfmt::locale::Polish)
        .long_units();

    group.bench_function("humfmt/english/short", |b| {
        b.iter(|| {
            for &v in &VALUES_U64 {
                black_box(number_with(black_box(v), en_short).to_string());
            }
        })
    });

    group.bench_function("humfmt/english/long", |b| {
        b.iter(|| {
            for &v in &VALUES_U64 {
                black_box(number_with(black_box(v), en_long).to_string());
            }
        })
    });

    group.bench_function("humfmt/russian/short", |b| {
        b.iter(|| {
            for &v in &VALUES_U64 {
                black_box(number_with(black_box(v), ru_short).to_string());
            }
        })
    });

    group.bench_function("humfmt/russian/long", |b| {
        b.iter(|| {
            for &v in &VALUES_U64 {
                black_box(number_with(black_box(v), ru_long).to_string());
            }
        })
    });

    group.bench_function("humfmt/polish/short", |b| {
        b.iter(|| {
            for &v in &VALUES_U64 {
                black_box(number_with(black_box(v), pl_short).to_string());
            }
        })
    });

    group.bench_function("humfmt/polish/long", |b| {
        b.iter(|| {
            for &v in &VALUES_U64 {
                black_box(number_with(black_box(v), pl_long).to_string());
            }
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_numbers_allocating,
    bench_numbers_allocating_int,
    bench_numbers_allocating_float,
    bench_numbers_reused_buffer,
    bench_numbers_locale,
);
criterion_main!(benches);

//! Compact-number formatting comparison benchmarks.
//!
//! Crates under comparison and their key properties:
//!
//!   - humfmt: all integer primitives + f32/f64, configurable precision,
//!     long/short suffixes, no_std compatible, writes via
//!     `Display` (zero-alloc path available).
//!   - human_format: f64 only, configurable decimals and separator, always
//!     returns an owned `String`.
//!   - numfmt: u64/i64/f64 inputs, configurable scale and precision,
//!     writes via `&str` returned from `Formatter::fmt2`.
//!
//! Note: human-repr's `human_count` and readable's `Unsigned` produce grouped
//! digits ("1,000") rather than compact suffixes ("1K") and are therefore not
//! included in this comparison.
//!
//! Groups:
//!   - numbers/allocating — to_string(), representative mixed inputs
//!   - numbers/allocating_int — to_string(), u64-only inputs (apples-to-apples)
//!   - numbers/allocating_float — to_string(), f64 inputs
//!   - numbers/reused_buffer — write! into pre-allocated String, u64 inputs

use std::fmt::Write;

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use humfmt::{number, number_with, NumberOptions};

const VALUES_MIXED_I64: [i64; 10] = [
    0, 12, 999, 1_250, 15_320, 999_950, 1_500_000, 75_000_000, -12_500, -1_000_000,
];

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

    group.bench_function("humfmt/i64/default", |b| {
        b.iter(|| {
            for &v in &VALUES_MIXED_I64 {
                black_box(number(black_box(v)).to_string());
            }
        })
    });

    let humfmt_p2 = NumberOptions::new().precision(2);
    group.bench_function("humfmt/i64/precision2", |b| {
        b.iter(|| {
            for &v in &VALUES_MIXED_I64 {
                black_box(number_with(black_box(v), humfmt_p2).to_string());
            }
        })
    });

    let mut hf = human_format::Formatter::new();
    hf.with_decimals(2).with_separator("");
    group.bench_function("human_format/f64/precision2", |b| {
        b.iter(|| {
            for &v in &VALUES_MIXED_I64 {
                black_box(hf.format(black_box(v as f64)));
            }
        })
    });

    // numfmt: configured for compact-ish output via Scales::short(), precision 2.
    // numfmt's `fmt2` returns a `&str` borrowed from the formatter's internal
    // buffer, which we copy into a `String` to make the comparison fair
    // (every other crate here either returns an owned `String` or implements
    // `Display`).
    let mut nf = numfmt::Formatter::new()
        .scales(numfmt::Scales::short())
        .precision(numfmt::Precision::Decimals(2));
    group.bench_function("numfmt/i64/short_scale_precision2", |b| {
        b.iter(|| {
            for &v in &VALUES_MIXED_I64 {
                let s: String = nf.fmt2(black_box(v)).to_owned();
                black_box(s);
            }
        })
    });

    group.finish();
}

fn bench_numbers_allocating_int(c: &mut Criterion) {
    let mut group = c.benchmark_group("numbers/allocating_int");

    let humfmt_default = NumberOptions::new();
    let humfmt_p2 = NumberOptions::new().precision(2);

    let mut hf1 = human_format::Formatter::new();
    hf1.with_decimals(1).with_separator("");

    let mut hf2 = human_format::Formatter::new();
    hf2.with_decimals(2).with_separator("");

    let mut nf2 = numfmt::Formatter::new()
        .scales(numfmt::Scales::short())
        .precision(numfmt::Precision::Decimals(2));

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

    group.bench_function("numfmt/u64/short_scale_precision2", |b| {
        b.iter(|| {
            for &v in &VALUES_U64 {
                let s: String = nf2.fmt2(black_box(v)).to_owned();
                black_box(s);
            }
        })
    });

    group.finish();
}

fn bench_numbers_allocating_float(c: &mut Criterion) {
    let mut group = c.benchmark_group("numbers/allocating_float");

    let humfmt_p2 = NumberOptions::new().precision(2);

    let mut hf = human_format::Formatter::new();
    hf.with_decimals(2).with_separator("");

    let mut nf = numfmt::Formatter::new()
        .scales(numfmt::Scales::short())
        .precision(numfmt::Precision::Decimals(2));

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

    group.bench_function("numfmt/f64/short_scale_precision2", |b| {
        b.iter(|| {
            for &v in &VALUES_F64 {
                let s: String = nf.fmt2(black_box(v)).to_owned();
                black_box(s);
            }
        })
    });

    group.finish();
}

fn bench_numbers_reused_buffer(c: &mut Criterion) {
    let mut group = c.benchmark_group("numbers/reused_buffer");

    let humfmt_default = NumberOptions::new();
    let humfmt_p2 = NumberOptions::new().precision(2);

    let mut hf = human_format::Formatter::new();
    hf.with_decimals(2).with_separator("");

    let mut nf = numfmt::Formatter::new()
        .scales(numfmt::Scales::short())
        .precision(numfmt::Precision::Decimals(2));

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
                    out.push_str(&hf.format(black_box(v as f64)));
                    black_box(&out);
                }
            })
        },
    );

    group.bench_with_input(
        BenchmarkId::new("numfmt/u64/short_scale_precision2/write", cap),
        &cap,
        |b, &cap| {
            let mut out = String::with_capacity(cap);
            b.iter(|| {
                for &v in &VALUES_U64 {
                    out.clear();
                    out.push_str(nf.fmt2(black_box(v)));
                    black_box(&out);
                }
            })
        },
    );

    group.finish();
}

criterion_group!(
    benches,
    bench_numbers_allocating,
    bench_numbers_allocating_int,
    bench_numbers_allocating_float,
    bench_numbers_reused_buffer,
);
criterion_main!(benches);

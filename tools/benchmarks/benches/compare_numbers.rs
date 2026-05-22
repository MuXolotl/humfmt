//! Compact-number formatting comparison benchmarks.
//!
//! Crates under comparison and their key properties:
//!
//!   - humfmt: all integer primitives + f32/f64, configurable precision,
//!     significant digits, long/short suffixes, separators, no_std compatible,
//!     writes via `Display`.
//!   - human_format: f64 only, configurable decimals and separator, always
//!     returns an owned `String`.
//!   - numfmt: u64/i64/f64 inputs, configurable scale and precision,
//!     returns a `&str` borrowed from the formatter's internal buffer.
//!
//! Note: human-repr's `human_count` and readable's `Unsigned` produce grouped
//! digits ("1,000") rather than compact suffixes ("1K") and are therefore not
//! included in the compact-number comparison.
//!
//! Groups:
//!   - numbers/allocating:
//!     to_string(), representative mixed i64 inputs.
//!   - numbers/allocating_int:
//!     to_string(), u64-only inputs (apples-to-apples).
//!   - numbers/allocating_float:
//!     to_string(), f64 inputs.
//!   - numbers/allocating_humfmt_options:
//!     humfmt-only option coverage for the integer path.
//!   - numbers/allocating_u128_extreme:
//!     humfmt-only extended-range coverage above typical competitor limits.
//!   - numbers/reused_buffer:
//!     write! / push_str into a pre-allocated String, u64 inputs.
//!   - numbers/reused_buffer_humfmt_options:
//!     humfmt-only reused-buffer option coverage.

use std::fmt::Write;

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use humfmt::{number, number_with, NumberOptions, RoundingMode};

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

const VALUES_U128_EXTREME: [u128; 6] = [
    u64::MAX as u128 + 1,
    1_000_000_000_000_000_000_u128,
    1_000_000_000_000_000_000_000_000_000_000_000_u128,
    i128::MAX as u128,
    u128::MAX / 2,
    u128::MAX,
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

    // numfmt's `fmt2` returns a `&str` borrowed from the formatter's internal
    // buffer. In this allocating group we copy it into a String to align with
    // crates that return an owned String or use `to_string()`.
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

fn bench_numbers_allocating_humfmt_options(c: &mut Criterion) {
    let mut group = c.benchmark_group("numbers/allocating_humfmt_options");

    let significant3 = NumberOptions::new().significant_digits(3);
    group.bench_function("i64/significant3", |b| {
        b.iter(|| {
            for &v in &VALUES_MIXED_I64 {
                black_box(number_with(black_box(v), significant3).to_string());
            }
        })
    });

    let long_units = NumberOptions::new().long_units().precision(2);
    group.bench_function("i64/long_units_precision2", |b| {
        b.iter(|| {
            for &v in &VALUES_MIXED_I64 {
                black_box(number_with(black_box(v), long_units).to_string());
            }
        })
    });

    let uncompacted_grouped = NumberOptions::new().compact(false).separators(true);
    group.bench_function("i64/uncompacted_grouped", |b| {
        b.iter(|| {
            for &v in &VALUES_MIXED_I64 {
                black_box(number_with(black_box(v), uncompacted_grouped).to_string());
            }
        })
    });

    let custom_separators = NumberOptions::new()
        .compact(false)
        .separators(true)
        .decimal_separator(',')
        .group_separator(' ');
    group.bench_function("i64/custom_separators_uncompacted", |b| {
        b.iter(|| {
            for &v in &VALUES_MIXED_I64 {
                black_box(number_with(black_box(v), custom_separators).to_string());
            }
        })
    });

    let force_sign = NumberOptions::new().force_sign(true);
    group.bench_function("i64/force_sign", |b| {
        b.iter(|| {
            for &v in &VALUES_MIXED_I64 {
                black_box(number_with(black_box(v), force_sign).to_string());
            }
        })
    });

    let floor = NumberOptions::new()
        .precision(0)
        .rounding(RoundingMode::Floor);
    group.bench_function("i64/rounding_floor_precision0", |b| {
        b.iter(|| {
            for &v in &VALUES_MIXED_I64 {
                black_box(number_with(black_box(v), floor).to_string());
            }
        })
    });

    let ceil = NumberOptions::new()
        .precision(0)
        .rounding(RoundingMode::Ceil);
    group.bench_function("i64/rounding_ceil_precision0", |b| {
        b.iter(|| {
            for &v in &VALUES_MIXED_I64 {
                black_box(number_with(black_box(v), ceil).to_string());
            }
        })
    });

    let float_significant3 = NumberOptions::new().significant_digits(3);
    group.bench_function("f64/significant3", |b| {
        b.iter(|| {
            for &v in &VALUES_F64 {
                black_box(number_with(black_box(v), float_significant3).to_string());
            }
        })
    });

    let float_uncompacted_grouped = NumberOptions::new().compact(false).separators(true);
    group.bench_function("f64/uncompacted_grouped", |b| {
        b.iter(|| {
            for &v in &VALUES_F64 {
                black_box(number_with(black_box(v), float_uncompacted_grouped).to_string());
            }
        })
    });

    group.finish();
}

fn bench_numbers_allocating_u128_extreme(c: &mut Criterion) {
    let mut group = c.benchmark_group("numbers/allocating_u128_extreme");

    group.bench_function("humfmt/default", |b| {
        b.iter(|| {
            for &v in &VALUES_U128_EXTREME {
                black_box(number(black_box(v)).to_string());
            }
        })
    });

    let precision2 = NumberOptions::new().precision(2);
    group.bench_function("humfmt/precision2", |b| {
        b.iter(|| {
            for &v in &VALUES_U128_EXTREME {
                black_box(number_with(black_box(v), precision2).to_string());
            }
        })
    });

    let significant3 = NumberOptions::new().significant_digits(3);
    group.bench_function("humfmt/significant3", |b| {
        b.iter(|| {
            for &v in &VALUES_U128_EXTREME {
                black_box(number_with(black_box(v), significant3).to_string());
            }
        })
    });

    let significant6 = NumberOptions::new().significant_digits(6);
    group.bench_function("humfmt/significant6", |b| {
        b.iter(|| {
            for &v in &VALUES_U128_EXTREME {
                black_box(number_with(black_box(v), significant6).to_string());
            }
        })
    });

    let uncompacted = NumberOptions::new().compact(false);
    group.bench_function("humfmt/uncompacted", |b| {
        b.iter(|| {
            for &v in &VALUES_U128_EXTREME {
                black_box(number_with(black_box(v), uncompacted).to_string());
            }
        })
    });

    let uncompacted_grouped = NumberOptions::new().compact(false).separators(true);
    group.bench_function("humfmt/uncompacted_grouped", |b| {
        b.iter(|| {
            for &v in &VALUES_U128_EXTREME {
                black_box(number_with(black_box(v), uncompacted_grouped).to_string());
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

fn bench_numbers_reused_buffer_humfmt_options(c: &mut Criterion) {
    let mut group = c.benchmark_group("numbers/reused_buffer_humfmt_options");

    let cap = 64usize;

    let significant3 = NumberOptions::new().significant_digits(3);
    group.bench_with_input(
        BenchmarkId::new("i64/significant3/write", cap),
        &cap,
        |b, &cap| {
            let mut out = String::with_capacity(cap);

            b.iter(|| {
                for &v in &VALUES_MIXED_I64 {
                    out.clear();
                    write!(&mut out, "{}", number_with(black_box(v), significant3)).unwrap();
                    black_box(&out);
                }
            })
        },
    );

    let uncompacted_grouped = NumberOptions::new().compact(false).separators(true);
    group.bench_with_input(
        BenchmarkId::new("i64/uncompacted_grouped/write", cap),
        &cap,
        |b, &cap| {
            let mut out = String::with_capacity(cap);

            b.iter(|| {
                for &v in &VALUES_MIXED_I64 {
                    out.clear();
                    write!(
                        &mut out,
                        "{}",
                        number_with(black_box(v), uncompacted_grouped)
                    )
                    .unwrap();
                    black_box(&out);
                }
            })
        },
    );

    let float_precision2 = NumberOptions::new().precision(2);
    group.bench_with_input(
        BenchmarkId::new("f64/precision2/write", cap),
        &cap,
        |b, &cap| {
            let mut out = String::with_capacity(cap);

            b.iter(|| {
                for &v in &VALUES_F64 {
                    out.clear();
                    write!(&mut out, "{}", number_with(black_box(v), float_precision2)).unwrap();
                    black_box(&out);
                }
            })
        },
    );

    let u128_default = NumberOptions::new();
    group.bench_with_input(
        BenchmarkId::new("u128_extreme/default/write", cap),
        &cap,
        |b, &cap| {
            let mut out = String::with_capacity(cap);

            b.iter(|| {
                for &v in &VALUES_U128_EXTREME {
                    out.clear();
                    write!(&mut out, "{}", number_with(black_box(v), u128_default)).unwrap();
                    black_box(&out);
                }
            })
        },
    );

    let u128_significant3 = NumberOptions::new().significant_digits(3);
    group.bench_with_input(
        BenchmarkId::new("u128_extreme/significant3/write", cap),
        &cap,
        |b, &cap| {
            let mut out = String::with_capacity(cap);

            b.iter(|| {
                for &v in &VALUES_U128_EXTREME {
                    out.clear();
                    write!(&mut out, "{}", number_with(black_box(v), u128_significant3)).unwrap();
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
    bench_numbers_allocating_humfmt_options,
    bench_numbers_allocating_u128_extreme,
    bench_numbers_reused_buffer,
    bench_numbers_reused_buffer_humfmt_options,
);
criterion_main!(benches);

use core::time::Duration;
use std::fmt::Write as _;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use humfmt::{
    ago, bytes, duration, list, number, BytesOptions, DurationOptions, ListOptions, NumberOptions,
    RoundingMode,
};

const NUMBER_VALUES_I64: [i64; 10] = [
    0, 12, 999, 1_250, 15_320, 999_950, 1_500_000, 75_000_000, -12_500, -1_000_000,
];

const NUMBER_VALUES_F64: [f64; 10] = [
    0.0,
    0.05,
    999.9,
    1_250.5,
    15_320.0,
    999_950.0,
    1_500_000.0,
    75_000_000.0,
    -12_500.5,
    -1_000_000.25,
];

const NUMBER_VALUES_U128_EXTREME: [u128; 6] = [
    u64::MAX as u128 + 1,
    1_000_000_000_000_000_000_u128,
    1_000_000_000_000_000_000_000_000_000_000_000_u128,
    i128::MAX as u128,
    u128::MAX / 2,
    u128::MAX,
];

const BYTE_VALUES: [u64; 6] = [512, 1536, 65_536, 1_048_576, 25_000_000, 9_876_543_210];

const DURATION_VALUES: [Duration; 6] = [
    Duration::from_millis(900),
    Duration::from_secs(90),
    Duration::from_secs(3661),
    Duration::from_secs(86_400 + 3665),
    Duration::from_millis(1_500),
    Duration::from_nanos(1_234_567),
];

const LIST_VALUES: [&str; 5] = ["red", "green", "blue", "amber", "violet"];

fn bench_numbers(c: &mut Criterion) {
    bench_numbers_allocating_integers(c);
    bench_numbers_allocating_floats(c);
    bench_numbers_allocating_u128_extreme(c);
    bench_numbers_reused_buffer(c);
}

fn bench_numbers_allocating_integers(c: &mut Criterion) {
    let mut group = c.benchmark_group("number/allocating/integer");

    group.bench_function("default", |b| {
        b.iter(|| {
            for value in NUMBER_VALUES_I64 {
                black_box(number(black_box(value)).to_string());
            }
        })
    });

    let precision2 = NumberOptions::new().precision(2);
    group.bench_function("precision2", |b| {
        b.iter(|| {
            for value in NUMBER_VALUES_I64 {
                black_box(humfmt::number_with(black_box(value), precision2).to_string());
            }
        })
    });

    let significant3 = NumberOptions::new().significant_digits(3);
    group.bench_function("significant3", |b| {
        b.iter(|| {
            for value in NUMBER_VALUES_I64 {
                black_box(humfmt::number_with(black_box(value), significant3).to_string());
            }
        })
    });

    let long_units = NumberOptions::new().long_units().precision(2);
    group.bench_function("long_units_precision2", |b| {
        b.iter(|| {
            for value in NUMBER_VALUES_I64 {
                black_box(humfmt::number_with(black_box(value), long_units).to_string());
            }
        })
    });

    let uncompacted_grouped = NumberOptions::new().compact(false).separators(true);
    group.bench_function("uncompacted_grouped", |b| {
        b.iter(|| {
            for value in NUMBER_VALUES_I64 {
                black_box(humfmt::number_with(black_box(value), uncompacted_grouped).to_string());
            }
        })
    });

    let custom_separators = NumberOptions::new()
        .compact(false)
        .separators(true)
        .decimal_separator(',')
        .group_separator(' ');
    group.bench_function("custom_separators_uncompacted", |b| {
        b.iter(|| {
            for value in NUMBER_VALUES_I64 {
                black_box(humfmt::number_with(black_box(value), custom_separators).to_string());
            }
        })
    });

    let force_sign = NumberOptions::new().force_sign(true);
    group.bench_function("force_sign", |b| {
        b.iter(|| {
            for value in NUMBER_VALUES_I64 {
                black_box(humfmt::number_with(black_box(value), force_sign).to_string());
            }
        })
    });

    let floor = NumberOptions::new()
        .precision(0)
        .rounding(RoundingMode::Floor);
    group.bench_function("rounding_floor_precision0", |b| {
        b.iter(|| {
            for value in NUMBER_VALUES_I64 {
                black_box(humfmt::number_with(black_box(value), floor).to_string());
            }
        })
    });

    let ceil = NumberOptions::new()
        .precision(0)
        .rounding(RoundingMode::Ceil);
    group.bench_function("rounding_ceil_precision0", |b| {
        b.iter(|| {
            for value in NUMBER_VALUES_I64 {
                black_box(humfmt::number_with(black_box(value), ceil).to_string());
            }
        })
    });

    group.finish();
}

fn bench_numbers_allocating_floats(c: &mut Criterion) {
    let mut group = c.benchmark_group("number/allocating/float");

    group.bench_function("default", |b| {
        b.iter(|| {
            for value in NUMBER_VALUES_F64 {
                black_box(number(black_box(value)).to_string());
            }
        })
    });

    let precision2 = NumberOptions::new().precision(2);
    group.bench_function("precision2", |b| {
        b.iter(|| {
            for value in NUMBER_VALUES_F64 {
                black_box(humfmt::number_with(black_box(value), precision2).to_string());
            }
        })
    });

    let significant3 = NumberOptions::new().significant_digits(3);
    group.bench_function("significant3", |b| {
        b.iter(|| {
            for value in NUMBER_VALUES_F64 {
                black_box(humfmt::number_with(black_box(value), significant3).to_string());
            }
        })
    });

    let uncompacted_grouped = NumberOptions::new().compact(false).separators(true);
    group.bench_function("uncompacted_grouped", |b| {
        b.iter(|| {
            for value in NUMBER_VALUES_F64 {
                black_box(humfmt::number_with(black_box(value), uncompacted_grouped).to_string());
            }
        })
    });

    let custom_separators = NumberOptions::new()
        .compact(false)
        .separators(true)
        .decimal_separator(',')
        .group_separator(' ');
    group.bench_function("custom_separators_uncompacted", |b| {
        b.iter(|| {
            for value in NUMBER_VALUES_F64 {
                black_box(humfmt::number_with(black_box(value), custom_separators).to_string());
            }
        })
    });

    let force_sign = NumberOptions::new().force_sign(true);
    group.bench_function("force_sign", |b| {
        b.iter(|| {
            for value in NUMBER_VALUES_F64 {
                black_box(humfmt::number_with(black_box(value), force_sign).to_string());
            }
        })
    });

    group.finish();
}

fn bench_numbers_allocating_u128_extreme(c: &mut Criterion) {
    let mut group = c.benchmark_group("number/allocating/u128_extreme");

    group.bench_function("default", |b| {
        b.iter(|| {
            for value in NUMBER_VALUES_U128_EXTREME {
                black_box(number(black_box(value)).to_string());
            }
        })
    });

    let precision2 = NumberOptions::new().precision(2);
    group.bench_function("precision2", |b| {
        b.iter(|| {
            for value in NUMBER_VALUES_U128_EXTREME {
                black_box(humfmt::number_with(black_box(value), precision2).to_string());
            }
        })
    });

    let significant3 = NumberOptions::new().significant_digits(3);
    group.bench_function("significant3", |b| {
        b.iter(|| {
            for value in NUMBER_VALUES_U128_EXTREME {
                black_box(humfmt::number_with(black_box(value), significant3).to_string());
            }
        })
    });

    let significant6 = NumberOptions::new().significant_digits(6);
    group.bench_function("significant6", |b| {
        b.iter(|| {
            for value in NUMBER_VALUES_U128_EXTREME {
                black_box(humfmt::number_with(black_box(value), significant6).to_string());
            }
        })
    });

    let uncompacted = NumberOptions::new().compact(false);
    group.bench_function("uncompacted", |b| {
        b.iter(|| {
            for value in NUMBER_VALUES_U128_EXTREME {
                black_box(humfmt::number_with(black_box(value), uncompacted).to_string());
            }
        })
    });

    let uncompacted_grouped = NumberOptions::new().compact(false).separators(true);
    group.bench_function("uncompacted_grouped", |b| {
        b.iter(|| {
            for value in NUMBER_VALUES_U128_EXTREME {
                black_box(humfmt::number_with(black_box(value), uncompacted_grouped).to_string());
            }
        })
    });

    group.finish();
}

fn bench_numbers_reused_buffer(c: &mut Criterion) {
    let mut group = c.benchmark_group("number/reused_buffer");

    let default_opts = NumberOptions::new();
    group.bench_function("integer_default", |b| {
        let mut out = String::with_capacity(64);

        b.iter(|| {
            for value in NUMBER_VALUES_I64 {
                out.clear();
                write!(
                    &mut out,
                    "{}",
                    humfmt::number_with(black_box(value), default_opts)
                )
                .unwrap();
                black_box(&out);
            }
        })
    });

    let precision2 = NumberOptions::new().precision(2);
    group.bench_function("integer_precision2", |b| {
        let mut out = String::with_capacity(64);

        b.iter(|| {
            for value in NUMBER_VALUES_I64 {
                out.clear();
                write!(
                    &mut out,
                    "{}",
                    humfmt::number_with(black_box(value), precision2)
                )
                .unwrap();
                black_box(&out);
            }
        })
    });

    let significant3 = NumberOptions::new().significant_digits(3);
    group.bench_function("integer_significant3", |b| {
        let mut out = String::with_capacity(64);

        b.iter(|| {
            for value in NUMBER_VALUES_I64 {
                out.clear();
                write!(
                    &mut out,
                    "{}",
                    humfmt::number_with(black_box(value), significant3)
                )
                .unwrap();
                black_box(&out);
            }
        })
    });

    let uncompacted_grouped = NumberOptions::new().compact(false).separators(true);
    group.bench_function("integer_uncompacted_grouped", |b| {
        let mut out = String::with_capacity(64);

        b.iter(|| {
            for value in NUMBER_VALUES_I64 {
                out.clear();
                write!(
                    &mut out,
                    "{}",
                    humfmt::number_with(black_box(value), uncompacted_grouped)
                )
                .unwrap();
                black_box(&out);
            }
        })
    });

    let float_precision2 = NumberOptions::new().precision(2);
    group.bench_function("float_precision2", |b| {
        let mut out = String::with_capacity(64);

        b.iter(|| {
            for value in NUMBER_VALUES_F64 {
                out.clear();
                write!(
                    &mut out,
                    "{}",
                    humfmt::number_with(black_box(value), float_precision2)
                )
                .unwrap();
                black_box(&out);
            }
        })
    });

    let u128_default = NumberOptions::new();
    group.bench_function("u128_extreme_default", |b| {
        let mut out = String::with_capacity(64);

        b.iter(|| {
            for value in NUMBER_VALUES_U128_EXTREME {
                out.clear();
                write!(
                    &mut out,
                    "{}",
                    humfmt::number_with(black_box(value), u128_default)
                )
                .unwrap();
                black_box(&out);
            }
        })
    });

    let u128_significant3 = NumberOptions::new().significant_digits(3);
    group.bench_function("u128_extreme_significant3", |b| {
        let mut out = String::with_capacity(64);

        b.iter(|| {
            for value in NUMBER_VALUES_U128_EXTREME {
                out.clear();
                write!(
                    &mut out,
                    "{}",
                    humfmt::number_with(black_box(value), u128_significant3)
                )
                .unwrap();
                black_box(&out);
            }
        })
    });

    group.finish();
}

fn bench_bytes(c: &mut Criterion) {
    let mut group = c.benchmark_group("bytes");

    group.bench_function("decimal", |b| {
        b.iter(|| {
            for value in BYTE_VALUES {
                black_box(bytes(black_box(value)).to_string());
            }
        })
    });

    let binary_opts = BytesOptions::new().binary();
    group.bench_function("binary", |b| {
        b.iter(|| {
            for value in BYTE_VALUES {
                black_box(humfmt::bytes_with(black_box(value), binary_opts).to_string());
            }
        })
    });

    group.finish();
}

fn bench_duration_and_ago(c: &mut Criterion) {
    let mut group = c.benchmark_group("time");

    group.bench_function("duration_short", |b| {
        b.iter(|| {
            for value in DURATION_VALUES {
                black_box(duration(black_box(value)).to_string());
            }
        })
    });

    let long_opts = DurationOptions::new().long_units().max_units(3);
    group.bench_function("duration_long", |b| {
        b.iter(|| {
            for value in DURATION_VALUES {
                black_box(humfmt::duration_with(black_box(value), long_opts).to_string());
            }
        })
    });

    group.bench_function("ago", |b| {
        b.iter(|| {
            for value in DURATION_VALUES {
                black_box(ago(black_box(value)).to_string());
            }
        })
    });

    group.finish();
}

fn bench_lists(c: &mut Criterion) {
    let mut group = c.benchmark_group("list");

    group.bench_function("default", |b| {
        b.iter(|| black_box(list(black_box(&LIST_VALUES)).to_string()))
    });

    let no_serial_comma = ListOptions::new().no_serial_comma();
    group.bench_function("no_serial_comma", |b| {
        b.iter(|| {
            black_box(humfmt::list_with(black_box(&LIST_VALUES), no_serial_comma).to_string())
        })
    });

    let custom_conjunction = ListOptions::new().conjunction("plus").no_serial_comma();
    group.bench_function("custom_conjunction", |b| {
        b.iter(|| {
            black_box(humfmt::list_with(black_box(&LIST_VALUES), custom_conjunction).to_string())
        })
    });

    let custom_separator = ListOptions::new().separator(" | ").conjunction("&");
    group.bench_function("custom_separator", |b| {
        b.iter(|| {
            black_box(humfmt::list_with(black_box(&LIST_VALUES), custom_separator).to_string())
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_numbers,
    bench_bytes,
    bench_duration_and_ago,
    bench_lists
);
criterion_main!(benches);

use core::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use humfmt::{
    ago, bytes, duration, list, locale::CustomLocale, number, BytesOptions, DurationOptions,
    ListOptions, NumberOptions,
};

const NUMBER_VALUES: [i64; 8] = [
    12, 999, 1_250, 15_320, 999_950, 1_500_000, 75_000_000, -12_500,
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
    let mut group = c.benchmark_group("number");

    group.bench_function("default", |b| {
        b.iter(|| {
            for value in NUMBER_VALUES {
                black_box(number(black_box(value)).to_string());
            }
        })
    });

    let long_opts = NumberOptions::new().long_units().precision(2);
    group.bench_function("long_units", |b| {
        b.iter(|| {
            for value in NUMBER_VALUES {
                black_box(humfmt::number_with(black_box(value), long_opts).to_string());
            }
        })
    });

    #[cfg(feature = "russian")]
    {
        let locale_opts = NumberOptions::new().locale(humfmt::locale::Russian);
        group.bench_function("russian_locale", |b| {
            b.iter(|| {
                for value in NUMBER_VALUES {
                    black_box(humfmt::number_with(black_box(value), locale_opts).to_string());
                }
            })
        });
    }

    let custom_locale = CustomLocale::english()
        .short_suffix(1, "k")
        .separators(',', '.');
    let custom_opts = NumberOptions::new().locale(custom_locale);
    group.bench_function("custom_locale", |b| {
        b.iter(|| {
            for value in NUMBER_VALUES {
                black_box(humfmt::number_with(black_box(value), custom_opts).to_string());
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

    group.bench_function("english", |b| {
        b.iter(|| black_box(list(black_box(&LIST_VALUES)).to_string()))
    });

    let no_serial_comma = ListOptions::new().no_serial_comma();
    group.bench_function("no_serial_comma", |b| {
        b.iter(|| {
            black_box(humfmt::list_with(black_box(&LIST_VALUES), no_serial_comma).to_string())
        })
    });

    let custom_locale = CustomLocale::english().and_word("plus").serial_comma(false);
    let custom_opts = ListOptions::new().locale(custom_locale);
    group.bench_function("custom_locale", |b| {
        b.iter(|| black_box(humfmt::list_with(black_box(&LIST_VALUES), custom_opts).to_string()))
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

//! Duration-formatting comparison benchmarks.
//!
//! Crates under comparison and their key properties:
//!
//!   - humfmt:    `std::time::Duration`, configurable unit count and labels,
//!                locale-aware (EN/RU/PL), long/short unit labels, max-units cap,
//!                no_std compatible, writes via `Display`.
//!   - humantime: `std::time::Duration`, English-only formatting,
//!                renders all non-zero units (no max-units cap),
//!                returns a `FormattedDuration` wrapper that implements `Display`.
//!
//! Note: semantic output differs. `humfmt` caps at 2 units by default ("1h 1m"),
//! while `humantime` emits all non-zero units ("1h 1m 5s 123ms ...").
//! The report calls out these differences explicitly.

use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use humfmt::{duration, duration_with, DurationOptions};

const VALUES: [Duration; 8] = [
    Duration::ZERO,
    Duration::from_millis(500),
    Duration::from_secs(90),
    Duration::from_secs(3_661),
    Duration::from_secs(86_400 + 3_665),
    Duration::from_nanos(1_234_567),
    Duration::from_millis(1_500),
    Duration::from_secs(7 * 86_400 + 3 * 3_600 + 22 * 60 + 15),
];

fn bench_duration(c: &mut Criterion) {
    let mut group = c.benchmark_group("duration/allocating");

    // --- Default short format (humfmt) ---

    group.bench_function("humfmt/short/default", |b| {
        b.iter(|| {
            for &v in &VALUES {
                black_box(duration(black_box(v)).to_string());
            }
        })
    });

    // humantime default: long form, all non-zero units (no max-units equivalent).
    group.bench_function("humantime/default", |b| {
        b.iter(|| {
            for &v in &VALUES {
                black_box(humantime::format_duration(black_box(v)).to_string());
            }
        })
    });

    // --- Long-form labels (humfmt) ---

    let long_opts = DurationOptions::new().long_units();
    group.bench_function("humfmt/long/default", |b| {
        b.iter(|| {
            for &v in &VALUES {
                black_box(duration_with(black_box(v), long_opts).to_string());
            }
        })
    });

    // --- More precision (humfmt max 3 units) ---

    let max3_opts = DurationOptions::new().max_units(3);
    group.bench_function("humfmt/short/max3", |b| {
        b.iter(|| {
            for &v in &VALUES {
                black_box(duration_with(black_box(v), max3_opts).to_string());
            }
        })
    });

    group.finish();
}

fn bench_duration_reused_buffer(c: &mut Criterion) {
    use std::fmt::Write as _;

    let mut group = c.benchmark_group("duration/reused_buffer");

    group.bench_function("humfmt/short/write", |b| {
        let mut buf = String::with_capacity(32);
        b.iter(|| {
            for &v in &VALUES {
                buf.clear();
                write!(&mut buf, "{}", duration(black_box(v))).unwrap();
                black_box(&buf);
            }
        })
    });

    group.bench_function("humantime/write", |b| {
        let mut buf = String::with_capacity(64);
        b.iter(|| {
            for &v in &VALUES {
                buf.clear();
                write!(&mut buf, "{}", humantime::format_duration(black_box(v))).unwrap();
                black_box(&buf);
            }
        })
    });

    group.finish();
}

criterion_group!(benches, bench_duration, bench_duration_reused_buffer);
criterion_main!(benches);

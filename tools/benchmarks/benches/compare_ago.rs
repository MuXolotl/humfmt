//! Relative-time ("N minutes ago") comparison benchmarks.
//!
//! Crates under comparison and their key limitations:
//!
//!   - humfmt:   std::time::Duration, locale-aware (EN/RU/PL), configurable
//!               unit count and labels, zero allocations during Display,
//!               no_std compatible.
//!   - timeago:  std::time::Duration, locale-aware (many languages via trait),
//!               configurable unit range and item count, always allocates a
//!               String on convert().
//!
//! Semantic difference: humfmt shows "1m 30s ago" (two units by default),
//! timeago shows "1 minute ago" (one unit by default, long-form English).
//! We benchmark both under their natural defaults and under num_items(2) for
//! timeago to produce output comparable to humfmt's default two-unit mode.

use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use humfmt::{ago, ago_with, DurationOptions};

const VALUES: [Duration; 8] = [
    Duration::ZERO,
    Duration::from_secs(30),
    Duration::from_secs(90),
    Duration::from_secs(3_661),
    Duration::from_secs(86_400 + 3_665),
    Duration::from_millis(1_500),
    Duration::from_secs(7 * 86_400),
    Duration::from_secs(365 * 86_400),
];

fn bench_ago(c: &mut Criterion) {
    let mut group = c.benchmark_group("ago/allocating");

    // --- Default (humfmt: 2 short units, timeago: 1 long unit) ---

    group.bench_function("humfmt/short/default", |b| {
        b.iter(|| {
            for &v in &VALUES {
                black_box(ago(black_box(v)).to_string());
            }
        })
    });

    // timeago default: 1 unit, long-form English, always allocates.
    group.bench_function("timeago/default/1_unit", |b| {
        let f = timeago::Formatter::new();
        b.iter(|| {
            for &v in &VALUES {
                black_box(f.convert(black_box(v)));
            }
        })
    });

    // --- Aligned comparison: 2 units each ---

    let humfmt_2 = DurationOptions::new().max_units(2);
    group.bench_function("humfmt/short/2_units", |b| {
        b.iter(|| {
            for &v in &VALUES {
                black_box(ago_with(black_box(v), humfmt_2).to_string());
            }
        })
    });

    // timeago with num_items(2) — closest equivalent to humfmt's 2-unit default.
    group.bench_function("timeago/2_units", |b| {
        let mut f = timeago::Formatter::new();
        f.num_items(2);
        b.iter(|| {
            for &v in &VALUES {
                black_box(f.convert(black_box(v)));
            }
        })
    });

    // --- Long-form labels ---

    let humfmt_long = DurationOptions::new().long_units().max_units(2);
    group.bench_function("humfmt/long/2_units", |b| {
        b.iter(|| {
            for &v in &VALUES {
                black_box(ago_with(black_box(v), humfmt_long).to_string());
            }
        })
    });

    group.finish();
}

criterion_group!(benches, bench_ago);
criterion_main!(benches);
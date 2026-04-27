use std::fmt::Write as _;

use byte_unit::Byte;
use bytesize::ByteSize;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use humfmt::{bytes_with, BytesOptions};
use prettier_bytes::{ByteFormatter, Standard, Unit};

const VALUES: [u64; 8] = [
    0,
    512,
    1_536,
    65_536,
    1_048_576,
    25_000_000,
    9_876_543_210,
    u32::MAX as u64,
];

fn bench_bytes_allocating(c: &mut Criterion) {
    let mut group = c.benchmark_group("bytes/allocating");

    let humfmt_opts = BytesOptions::new().precision(2);
    let prettier = ByteFormatter::new()
        .standard(Standard::SI)
        .unit(Unit::Bytes)
        .space(false);

    group.bench_function("humfmt/to_string", |b| {
        b.iter(|| {
            for &v in &VALUES {
                black_box(bytes_with(black_box(v), humfmt_opts).to_string());
            }
        })
    });

    group.bench_function("bytesize/display_si/to_string", |b| {
        b.iter(|| {
            for &v in &VALUES {
                let s = ByteSize::b(black_box(v)).display().si().to_string();
                black_box(s);
            }
        })
    });

    group.bench_function("byte_unit/alt_format/#.2", |b| {
        b.iter(|| {
            for &v in &VALUES {
                let byte = Byte::from_u64(black_box(v));
                let s = format!("{byte:#.2}");
                black_box(s);
            }
        })
    });

    group.bench_function("prettier_bytes/to_string", |b| {
        b.iter(|| {
            for &v in &VALUES {
                let s = prettier.format(black_box(v)).to_string();
                black_box(s);
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

    for cap in [32usize, 64, 128] {
        group.bench_with_input(BenchmarkId::new("humfmt/write", cap), &cap, |b, &cap| {
            let mut out = String::with_capacity(cap);
            b.iter(|| {
                out.clear();
                for &v in &VALUES {
                    out.clear();
                    write!(&mut out, "{}", bytes_with(black_box(v), humfmt_opts)).unwrap();
                    black_box(&out);
                }
            })
        });

        group.bench_with_input(
            BenchmarkId::new("bytesize/write_display_si", cap),
            &cap,
            |b, &cap| {
                let mut out = String::with_capacity(cap);
                b.iter(|| {
                    out.clear();
                    for &v in &VALUES {
                        out.clear();
                        let disp = ByteSize::b(black_box(v)).display().si();
                        write!(&mut out, "{disp}").unwrap();
                        black_box(&out);
                    }
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("byte_unit/write_alt_format/#.2", cap),
            &cap,
            |b, &cap| {
                let mut out = String::with_capacity(cap);
                b.iter(|| {
                    out.clear();
                    for &v in &VALUES {
                        out.clear();
                        let byte = Byte::from_u64(black_box(v));
                        write!(&mut out, "{byte:#.2}").unwrap();
                        black_box(&out);
                    }
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("prettier_bytes/write", cap),
            &cap,
            |b, &cap| {
                let mut out = String::with_capacity(cap);
                b.iter(|| {
                    out.clear();
                    for &v in &VALUES {
                        out.clear();
                        let formatted = prettier.format(black_box(v));
                        write!(&mut out, "{formatted}").unwrap();
                        black_box(&out);
                    }
                })
            },
        );
    }

    group.finish();
}

criterion_group!(benches, bench_bytes_allocating, bench_bytes_reused_buffer);
criterion_main!(benches);
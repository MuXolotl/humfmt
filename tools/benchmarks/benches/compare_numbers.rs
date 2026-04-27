use criterion::{black_box, criterion_group, criterion_main, Criterion};
use humfmt::{number_with, NumberOptions};

const VALUES: [f64; 10] = [
    0.0,
    12.0,
    999.0,
    1_250.0,
    15_320.0,
    999_950.0,
    1_500_000.0,
    75_000_000.0,
    -12_500.0,
    9_876_543_210.0,
];

fn bench_numbers_allocating(c: &mut Criterion) {
    let mut group = c.benchmark_group("numbers/allocating");

    let humfmt_opts = NumberOptions::new().precision(2);
    let mut hf = human_format::Formatter::new();
    hf.with_decimals(2).with_separator("");

    group.bench_function("humfmt/to_string", |b| {
        b.iter(|| {
            for &v in &VALUES {
                let s = number_with(black_box(v), humfmt_opts).to_string();
                black_box(s);
            }
        })
    });

    group.bench_function("human_format/Formatter::format", |b| {
        b.iter(|| {
            for &v in &VALUES {
                let s = hf.format(black_box(v));
                black_box(s);
            }
        })
    });

    group.finish();
}

criterion_group!(benches, bench_numbers_allocating);
criterion_main!(benches);
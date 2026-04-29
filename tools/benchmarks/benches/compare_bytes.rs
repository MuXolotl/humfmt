//! Byte-formatting comparison benchmarks.
//!
//! This harness intentionally measures two patterns:
//! 1) Allocating: `.to_string()`
//! 2) Reused buffer: writing Display output into a pre-allocated `String`
//!
//! We keep two comparison groups for honesty:
//! - `bytes/allocating` + `bytes/reused_buffer`:
//!     SI/decimal, close to humfmt's default output style.
//! - `bytes/allocating_aligned` + `bytes/reused_buffer_aligned`:
//!     IEC/binary + space + "precision 2" where supported, aligned to common CLI output styles.
//!
//! IMPORTANT: Some crates keep a fixed number of decimal digits when precision is set,
//! while humfmt trims trailing zeros by design. The report generator includes output
//! examples to make these differences explicit.

use std::fmt::Write as _;

use byte_unit::{Byte, UnitType};
use bytesize::ByteSize;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use humfmt::{bytes, bytes_with, BytesOptions};
use human_repr::HumanCount;
use humansize::{format_size, format_size_i, FormatSizeOptions, SizeFormatter, BINARY, DECIMAL};
use indicatif::HumanBytes;
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

// Values used for the aligned group (IEC + space + "precision 2").
// Chosen to avoid always producing trailing zero fractional digits.
const VALUES_U64_ALIGNED: [u64; 6] = [
    15,
    1_500,
    1_500_000,
    1_514_000_000,
    1_500_000_000_000,
    1_500_000_000_000_000,
];

// Extended values that exceed u64::MAX — only humfmt can handle these.
const VALUES_U128_EXTENDED: [u128; 4] = [
    u64::MAX as u128 + 1,
    1_000_000_000_000_000_000_000_u128, // ~1 ZB (zettabyte range)
    u128::MAX / 2,
    u128::MAX,
];

// Negative values — humfmt supports these natively; humansize supports signed values via `format_size_i`.
const VALUES_NEGATIVE: [i64; 4] = [-512, -1_536, -1_048_576, -9_876_543_210];

fn bench_bytes_allocating(c: &mut Criterion) {
    let mut group = c.benchmark_group("bytes/allocating");

    let humfmt_opts = BytesOptions::new().precision(2);

    // humansize default-style: SI + precision=2, no forced trailing zeros, no space.
    let humansize_opts = FormatSizeOptions::from(DECIMAL)
        .decimal_places(2)
        .decimal_zeroes(0)
        .space_after_value(false);

    // prettier-bytes configured to match humfmt defaults as closely as possible:
    // SI standard, Bytes unit, no space.
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

    group.bench_function("humansize/u64/decimal_precision2/to_string", |b| {
        b.iter(|| {
            for &v in &VALUES_U64 {
                black_box(format_size(black_box(v), humansize_opts));
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

    group.bench_function("prettier_bytes/u64/to_string", |b| {
        b.iter(|| {
            for &v in &VALUES_U64 {
                let s = prettier.format(black_box(v)).to_string();
                black_box(s);
            }
        })
    });

    // --- humfmt-only extended range (u128 > u64::MAX) ---

    group.bench_function("humfmt/u128_extended/to_string", |b| {
        b.iter(|| {
            for &v in &VALUES_U128_EXTENDED {
                black_box(bytes_with(black_box(v), humfmt_opts).to_string());
            }
        })
    });

    // --- signed negative values (humfmt + humansize) ---

    group.bench_function("humfmt/negative_i64/to_string", |b| {
        b.iter(|| {
            for &v in &VALUES_NEGATIVE {
                black_box(bytes(black_box(v)).to_string());
            }
        })
    });

    group.bench_function("humansize/negative_i64/to_string", |b| {
        b.iter(|| {
            for &v in &VALUES_NEGATIVE {
                black_box(format_size_i(black_box(v), humansize_opts));
            }
        })
    });

    group.finish();
}

fn bench_bytes_allocating_aligned(c: &mut Criterion) {
    let mut group = c.benchmark_group("bytes/allocating_aligned");

    // Align humfmt to common CLI output style:
    // - IEC (binary)
    // - precision 2 (note: trailing zeros are trimmed by humfmt)
    // - space before suffix
    let humfmt_aligned = BytesOptions::new().binary().precision(2).space(true);

    // humansize aligned: IEC + fixed 2dp + space
    let humansize_aligned = FormatSizeOptions::from(BINARY)
        .decimal_places(2)
        .decimal_zeroes(2)
        .space_after_value(true);

    group.bench_function("humfmt/u64/iec_space/to_string", |b| {
        b.iter(|| {
            for &v in &VALUES_U64_ALIGNED {
                black_box(bytes_with(black_box(v), humfmt_aligned).to_string());
            }
        })
    });

    group.bench_function("indicatif/u64/HumanBytes/to_string", |b| {
        b.iter(|| {
            for &v in &VALUES_U64_ALIGNED {
                black_box(HumanBytes(black_box(v)).to_string());
            }
        })
    });

    group.bench_function("humansize/u64/iec_fixed2/to_string", |b| {
        b.iter(|| {
            for &v in &VALUES_U64_ALIGNED {
                black_box(format_size(black_box(v), humansize_aligned));
            }
        })
    });

    group.bench_function("bytesize/u64/iec_precision2/to_string", |b| {
        b.iter(|| {
            for &v in &VALUES_U64_ALIGNED {
                let disp = ByteSize::b(black_box(v)).display().iec();
                let s = format!("{disp:.2}");
                black_box(s);
            }
        })
    });

    group.bench_function("byte_unit/u64/binary_precision2/to_string", |b| {
        b.iter(|| {
            for &v in &VALUES_U64_ALIGNED {
                let adjusted = Byte::from_u64(black_box(v)).get_appropriate_unit(UnitType::Binary);
                let s = format!("{adjusted:.2}");
                black_box(s);
            }
        })
    });

    group.bench_function("human_repr/u64/iec_space/to_string", |b| {
        b.iter(|| {
            for &v in &VALUES_U64_ALIGNED {
                black_box(black_box(v).human_count_bytes().to_string());
            }
        })
    });

    group.finish();
}

fn bench_bytes_reused_buffer(c: &mut Criterion) {
    let mut group = c.benchmark_group("bytes/reused_buffer");

    let humfmt_opts = BytesOptions::new().precision(2);

    let humansize_opts = FormatSizeOptions::from(DECIMAL)
        .decimal_places(2)
        .decimal_zeroes(0)
        .space_after_value(false);

    let prettier = ByteFormatter::new()
        .standard(Standard::SI)
        .unit(Unit::Bytes)
        .space(false);

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
        BenchmarkId::new("humansize/u64/write_decimal_precision2", cap),
        &cap,
        |b, &cap| {
            let mut out = String::with_capacity(cap);
            b.iter(|| {
                for &v in &VALUES_U64 {
                    out.clear();
                    write!(
                        &mut out,
                        "{}",
                        SizeFormatter::new(black_box(v), humansize_opts)
                    )
                    .unwrap();
                    black_box(&out);
                }
            })
        },
    );

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

fn bench_bytes_reused_buffer_aligned(c: &mut Criterion) {
    let mut group = c.benchmark_group("bytes/reused_buffer_aligned");

    let humfmt_aligned = BytesOptions::new().binary().precision(2).space(true);

    let humansize_aligned = FormatSizeOptions::from(BINARY)
        .decimal_places(2)
        .decimal_zeroes(2)
        .space_after_value(true);

    let cap = 32usize;

    group.bench_with_input(
        BenchmarkId::new("humfmt/u64/iec_space/write", cap),
        &cap,
        |b, &cap| {
            let mut out = String::with_capacity(cap);
            b.iter(|| {
                for &v in &VALUES_U64_ALIGNED {
                    out.clear();
                    write!(&mut out, "{}", bytes_with(black_box(v), humfmt_aligned)).unwrap();
                    black_box(&out);
                }
            })
        },
    );

    group.bench_with_input(
        BenchmarkId::new("indicatif/u64/HumanBytes/write", cap),
        &cap,
        |b, &cap| {
            let mut out = String::with_capacity(cap);
            b.iter(|| {
                for &v in &VALUES_U64_ALIGNED {
                    out.clear();
                    write!(&mut out, "{}", HumanBytes(black_box(v))).unwrap();
                    black_box(&out);
                }
            })
        },
    );

    group.bench_with_input(
        BenchmarkId::new("humansize/u64/iec_fixed2/write", cap),
        &cap,
        |b, &cap| {
            let mut out = String::with_capacity(cap);
            b.iter(|| {
                for &v in &VALUES_U64_ALIGNED {
                    out.clear();
                    write!(
                        &mut out,
                        "{}",
                        SizeFormatter::new(black_box(v), humansize_aligned)
                    )
                    .unwrap();
                    black_box(&out);
                }
            })
        },
    );

    group.bench_with_input(
        BenchmarkId::new("bytesize/u64/iec_precision2/write", cap),
        &cap,
        |b, &cap| {
            let mut out = String::with_capacity(cap);
            b.iter(|| {
                for &v in &VALUES_U64_ALIGNED {
                    out.clear();
                    let disp = ByteSize::b(black_box(v)).display().iec();
                    write!(&mut out, "{disp:.2}").unwrap();
                    black_box(&out);
                }
            })
        },
    );

    group.bench_with_input(
        BenchmarkId::new("byte_unit/u64/binary_precision2/write", cap),
        &cap,
        |b, &cap| {
            let mut out = String::with_capacity(cap);
            b.iter(|| {
                for &v in &VALUES_U64_ALIGNED {
                    out.clear();
                    let adjusted = Byte::from_u64(black_box(v)).get_appropriate_unit(UnitType::Binary);
                    write!(&mut out, "{adjusted:.2}").unwrap();
                    black_box(&out);
                }
            })
        },
    );

    group.bench_with_input(
        BenchmarkId::new("human_repr/u64/iec_space/write", cap),
        &cap,
        |b, &cap| {
            let mut out = String::with_capacity(cap);
            b.iter(|| {
                for &v in &VALUES_U64_ALIGNED {
                    out.clear();
                    write!(&mut out, "{}", black_box(v).human_count_bytes()).unwrap();
                    black_box(&out);
                }
            })
        },
    );

    group.finish();
}

criterion_group!(
    benches,
    bench_bytes_allocating,
    bench_bytes_allocating_aligned,
    bench_bytes_reused_buffer,
    bench_bytes_reused_buffer_aligned
);
criterion_main!(benches);
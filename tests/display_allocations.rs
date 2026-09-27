//! Checks that formatting a value does not touch the heap.
//!
//! Every formatter writes into the sink it is handed, so `write!` into a buffer
//! with room to spare must not allocate. `to_string` is not measured here: the
//! `String` it returns is itself the allocation, and a type that can only be
//! rendered through one is a different question.
//!
//! The counter belongs to the whole process, so this file holds a single test:
//! the harness then runs no other test body beside it and the counter can only
//! move when the measured call allocates.

use core::fmt::Result as FmtResult;
use core::fmt::Write as _;
use core::time::Duration;
use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};

use humfmt::{
    ago, ago_with, bytes, bytes_with, duration, duration_with, list, list_with, number,
    number_with, ordinal, percent, percent_with, AgoOptions, ByteUnit, BytesOptions,
    DurationOptions, Humanize, ListOptions, NumberOptions, PercentOptions,
};

/// Holds the widest value under test: `f64::MAX` as a percentage is 312
/// characters, one digit more than the same value written uncompacted.
const CAPACITY: usize = 512;

static ALLOCATIONS: AtomicUsize = AtomicUsize::new(0);

struct CountingAllocator;

unsafe impl GlobalAlloc for CountingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ALLOCATIONS.fetch_add(1, Ordering::Relaxed);
        System.alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout)
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        ALLOCATIONS.fetch_add(1, Ordering::Relaxed);
        System.realloc(ptr, layout, new_size)
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        ALLOCATIONS.fetch_add(1, Ordering::Relaxed);
        System.alloc_zeroed(layout)
    }
}

#[global_allocator]
static ALLOCATOR: CountingAllocator = CountingAllocator;

/// A value to render into a sink, named for the failure message. The closures
/// capture nothing, so each one is a plain function pointer.
type Case = (&'static str, fn(&mut String) -> FmtResult);

#[test]
fn formatting_writes_no_allocations() {
    let cases: [Case; 24] = [
        ("number", |s| write!(s, "{}", number(15_320))),
        ("number with precision", |s| {
            write!(
                s,
                "{}",
                number_with(0.15_f64, NumberOptions::new().precision(1))
            )
        }),
        ("number with significant digits", |s| {
            write!(
                s,
                "{}",
                number_with(12_345, NumberOptions::new().significant_digits(3))
            )
        }),
        ("number with separators and no compaction", |s| {
            write!(
                s,
                "{}",
                number_with(
                    1_234_567,
                    NumberOptions::new().compact(false).separators(true)
                )
            )
        }),
        ("number with fixed precision and a forced sign", |s| {
            write!(
                s,
                "{}",
                number_with(
                    15_320,
                    NumberOptions::new()
                        .precision(2)
                        .fixed_precision(true)
                        .force_sign(true)
                )
            )
        }),
        ("number at u128::MAX", |s| {
            write!(s, "{}", number(u128::MAX))
        }),
        ("number at f64::MAX, uncompacted", |s| {
            write!(
                s,
                "{}",
                number_with(f64::MAX, NumberOptions::new().compact(false))
            )
        }),
        (
            "number at f64::MAX, rounded past the top of the range",
            |s| {
                write!(
                    s,
                    "{}",
                    number_with(
                        f64::MAX,
                        NumberOptions::new().compact(false).significant_digits(1)
                    )
                )
            },
        ),
        ("number padded to a width", |s| {
            write!(s, "{:>12}", number(15_320))
        }),
        ("number through the extension trait", |s| {
            write!(s, "{}", 1_500_000.human_number())
        }),
        ("bytes", |s| write!(s, "{}", bytes(1536_u64))),
        ("bytes with binary units and a space", |s| {
            write!(
                s,
                "{}",
                bytes_with(
                    -1536_i64,
                    BytesOptions::new().binary().space(true).precision(2)
                )
            )
        }),
        ("bytes with a long unit label", |s| {
            write!(s, "{}", bytes_with(1_u64, BytesOptions::new().long_units()))
        }),
        ("bytes with significant digits below one unit", |s| {
            write!(
                s,
                "{}",
                bytes_with(
                    12_345_u64,
                    BytesOptions::new()
                        .min_unit(ByteUnit::MB)
                        .significant_digits(3)
                )
            )
        }),
        ("bytes at u128::MAX in bits", |s| {
            write!(
                s,
                "{}",
                bytes_with(u128::MAX, BytesOptions::new().bits(true))
            )
        }),
        ("bytes through the extension trait", |s| {
            write!(s, "{}", 1536_u64.human_bytes())
        }),
        ("percent", |s| write!(s, "{}", percent(0.423_f64))),
        ("percent with fixed precision", |s| {
            write!(
                s,
                "{}",
                percent_with(
                    0.425_f64,
                    PercentOptions::new().precision(2).fixed_precision(true)
                )
            )
        }),
        ("percent at f64::MAX", |s| {
            write!(
                s,
                "{}",
                percent_with(f64::MAX, PercentOptions::new().precision(0))
            )
        }),
        ("duration", |s| {
            write!(s, "{}", duration(Duration::from_secs(3661)))
        }),
        ("duration with long labels and three units", |s| {
            write!(
                s,
                "{}",
                duration_with(
                    Duration::from_secs(3665),
                    DurationOptions::new().long_units().max_units(3)
                )
            )
        }),
        ("ago", |s| write!(s, "{}", ago(Duration::from_secs(90)))),
        ("ordinal", |s| write!(s, "{}", ordinal(21_u32))),
        ("list", |s| write!(s, "{}", list(&["red", "green", "blue"]))),
    ];

    let mut sink = String::with_capacity(CAPACITY);

    for (label, render) in cases {
        sink.clear();

        let before = ALLOCATIONS.load(Ordering::Relaxed);
        let rendered = render(&mut sink);
        let after = ALLOCATIONS.load(Ordering::Relaxed);

        rendered.expect("a String sink accepts everything");
        assert_eq!(before, after, "{label} allocated while formatting");
    }

    // `just_now` and the custom separators are option-heavy paths with their own
    // branches, and they share the sink with the cases above.
    let just_now = ago_with(
        Duration::ZERO,
        AgoOptions::new().just_now(Duration::from_secs(5)),
    );
    let custom_list = list_with(
        &["red", "green"],
        ListOptions::new().separator(" | ").conjunction("&"),
    );

    sink.clear();

    let before = ALLOCATIONS.load(Ordering::Relaxed);
    let rendered = write!(sink, "{just_now} and {custom_list}");
    let after = ALLOCATIONS.load(Ordering::Relaxed);

    rendered.expect("a String sink accepts everything");
    assert_eq!(
        before, after,
        "option-heavy values allocated while formatting"
    );
}

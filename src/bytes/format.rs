use core::fmt;
use core::fmt::Write;

use super::{traits::BytesValue, BytesOptions};
use crate::common::fmt::{decimal_parts_rounded, write_frac_digits, write_u128};

const DECIMAL_SHORT: [&str; 7] = ["B", "KB", "MB", "GB", "TB", "PB", "EB"];
const BINARY_SHORT: [&str; 7] = ["B", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB"];

const DECIMAL_LONG_SINGULAR: [&str; 7] = [
    "byte", "kilobyte", "megabyte", "gigabyte", "terabyte", "petabyte", "exabyte",
];
const DECIMAL_LONG_PLURAL: [&str; 7] = [
    "bytes",
    "kilobytes",
    "megabytes",
    "gigabytes",
    "terabytes",
    "petabytes",
    "exabytes",
];

const BINARY_LONG_SINGULAR: [&str; 7] = [
    "byte", "kibibyte", "mebibyte", "gibibyte", "tebibyte", "pebibyte", "exbibyte",
];
const BINARY_LONG_PLURAL: [&str; 7] = [
    "bytes",
    "kibibytes",
    "mebibytes",
    "gibibytes",
    "tebibytes",
    "pebibytes",
    "exbibytes",
];

pub fn format_bytes(
    f: &mut fmt::Formatter<'_>,
    value: BytesValue,
    options: &BytesOptions,
) -> fmt::Result {
    let (negative, magnitude) = match value {
        BytesValue::Int(v) if v < 0 => (true, v.unsigned_abs()),
        BytesValue::Int(v) => (false, v as u128),
        BytesValue::UInt(v) => (false, v),
    };

    let base: u128 = if options.binary_value() { 1024 } else { 1000 };
    let max_idx = DECIMAL_SHORT.len() - 1;
    let precision = options.precision_value();

    let (mut idx, mut unit) = compute_unit(magnitude, base, max_idx);
    let mut parts = decimal_parts_rounded(magnitude, unit, precision);

    // Rescale if rounding pushes us over the unit boundary (e.g. 999.95KB -> 1MB).
    if parts.integer >= base && idx < max_idx {
        idx += 1;
        unit *= base;
        parts = decimal_parts_rounded(magnitude, unit, precision);
    }

    if negative && magnitude != 0 {
        f.write_str("-")?;
    }

    // Bytes formatting is not locale-aware, and historically this crate did not add grouping
    // separators, so we keep it that way for determinism.
    write_u128(f, parts.integer, false, ',')?;

    if parts.frac_len != 0 {
        f.write_char('.')?;
        write_frac_digits(f, &parts.frac_digits[..parts.frac_len as usize])?;
    }

    if options.long_units_value() {
        let label = long_label(options.binary_value(), idx, parts.is_exactly_one());
        write!(f, " {label}")
    } else {
        let suffix = short_label(options.binary_value(), idx);
        f.write_str(suffix)
    }
}

fn compute_unit(magnitude: u128, base: u128, max_idx: usize) -> (usize, u128) {
    let mut idx = 0usize;
    let mut unit = 1u128;
    let mut tmp = magnitude;

    while tmp >= base && idx < max_idx {
        tmp /= base;
        idx += 1;
        unit *= base;
    }

    (idx, unit)
}

fn short_label(binary: bool, idx: usize) -> &'static str {
    if binary {
        BINARY_SHORT[idx]
    } else {
        DECIMAL_SHORT[idx]
    }
}

fn long_label(binary: bool, idx: usize, singular: bool) -> &'static str {
    match (binary, singular) {
        (false, true) => DECIMAL_LONG_SINGULAR[idx],
        (false, false) => DECIMAL_LONG_PLURAL[idx],
        (true, true) => BINARY_LONG_SINGULAR[idx],
        (true, false) => BINARY_LONG_PLURAL[idx],
    }
}

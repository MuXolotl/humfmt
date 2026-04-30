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

const DECIMAL_UNITS: [u128; 7] = [
    1,
    1_000,
    1_000_000,
    1_000_000_000,
    1_000_000_000_000,
    1_000_000_000_000_000,
    1_000_000_000_000_000_000,
];

const BINARY_UNITS: [u128; 7] = [
    1,
    1_024,
    1_048_576,
    1_073_741_824,
    1_099_511_627_776,
    1_125_899_906_842_624,
    1_152_921_504_606_846_976,
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

    let max_idx: usize = 6;
    let precision = options.precision;

    let (mut idx, table) = if options.binary {
        let idx = if magnitude == 0 {
            0
        } else {
            ((magnitude.ilog2() / 10) as usize).min(max_idx)
        };
        (idx, &BINARY_UNITS)
    } else {
        let idx = if magnitude == 0 {
            0
        } else {
            ((magnitude.ilog10() / 3) as usize).min(max_idx)
        };
        (idx, &DECIMAL_UNITS)
    };

    let mut unit = table[idx];
    let mut parts = decimal_parts_rounded(magnitude, unit, precision);

    let boundary = if options.binary { 1_024 } else { 1_000 };
    if parts.integer >= boundary && idx < max_idx {
        idx += 1;
        unit = table[idx];
        parts = decimal_parts_rounded(magnitude, unit, precision);
    }

    if negative && magnitude != 0 {
        f.write_str("-")?;
    }

    write_u128(f, parts.integer, false, ',')?;

    if parts.frac_len != 0 {
        f.write_char(options.decimal_separator)?;
        write_frac_digits(f, &parts.frac_digits[..parts.frac_len as usize])?;
    }

    if options.long_units {
        let label = long_label(options.binary, idx, parts.is_exactly_one());
        write!(f, " {label}")
    } else {
        if options.space {
            f.write_char(' ')?;
        }
        let suffix = short_label(options.binary, idx);
        f.write_str(suffix)
    }
}

#[inline]
fn short_label(binary: bool, idx: usize) -> &'static str {
    if binary {
        BINARY_SHORT[idx]
    } else {
        DECIMAL_SHORT[idx]
    }
}

#[inline]
fn long_label(binary: bool, idx: usize, singular: bool) -> &'static str {
    match (binary, singular) {
        (false, true) => DECIMAL_LONG_SINGULAR[idx],
        (false, false) => DECIMAL_LONG_PLURAL[idx],
        (true, true) => BINARY_LONG_SINGULAR[idx],
        (true, false) => BINARY_LONG_PLURAL[idx],
    }
}

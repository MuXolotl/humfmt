use core::fmt;

use super::{traits::BytesValue, BytesOptions};

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
    let raw = to_f64(value);
    let negative = raw.is_sign_negative();
    let abs = raw.abs();
    let base = if options.binary_value() {
        1024.0
    } else {
        1000.0
    };

    let (scaled, idx) = normalize_scaled(abs, base, options.precision_value());
    let rendered = render_scaled(scaled, options.precision_value());

    if negative {
        write!(f, "-")?;
    }

    if options.long_units_value() {
        let label = long_label(options.binary_value(), idx, scaled == 1.0);
        write!(f, "{rendered} {label}")
    } else {
        let suffix = short_label(options.binary_value(), idx);
        write!(f, "{rendered}{suffix}")
    }
}

fn to_f64(value: BytesValue) -> f64 {
    match value {
        BytesValue::Int(v) => v as f64,
        BytesValue::UInt(v) => v as f64,
    }
}

fn normalize_scaled(value: f64, base: f64, precision: u8) -> (f64, usize) {
    let mut scaled = value;
    let mut idx = 0;
    let max_idx = DECIMAL_SHORT.len() - 1;

    while scaled >= base && idx < max_idx {
        scaled /= base;
        idx += 1;
    }

    scaled = round_to(scaled, precision);

    if scaled >= base && idx < max_idx {
        scaled /= base;
        idx += 1;
    }

    (scaled, idx)
}

fn round_to(value: f64, precision: u8) -> f64 {
    let factor = pow10(precision);
    (((value * factor) + 0.5) as u128 as f64) / factor
}

fn pow10(precision: u8) -> f64 {
    let mut factor = 1.0;

    for _ in 0..precision {
        factor *= 10.0;
    }

    factor
}

fn render_scaled(value: f64, precision: u8) -> alloc::string::String {
    let mut out = if is_integer(value) {
        alloc::format!("{:.0}", value)
    } else {
        alloc::format!("{:.*}", precision as usize, value)
    };

    trim_trailing_zeroes(&mut out);
    out
}

fn is_integer(value: f64) -> bool {
    value == (value as u128) as f64
}

fn trim_trailing_zeroes(s: &mut alloc::string::String) {
    if !s.contains('.') {
        return;
    }

    while s.ends_with('0') {
        s.pop();
    }

    if s.ends_with('.') {
        s.pop();
    }
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

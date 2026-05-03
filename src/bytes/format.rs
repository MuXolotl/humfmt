use core::fmt;
use core::fmt::Write;

use super::options::Precision;
use super::{traits::BytesValue, BytesOptions};
use crate::common::fmt::{decimal_parts_rounded, write_frac_digits, write_u128};

// Each entry groups short label, long singular, and long plural for one unit tier.
// Index 0 = bytes, 1 = kilo/kibi, ..., 6 = exa/exbi.
struct UnitLabels {
    short: &'static str,
    long_singular: &'static str,
    long_plural: &'static str,
}

// --- Byte Labels ---

const DECIMAL_LABELS: [UnitLabels; 7] = [
    UnitLabels {
        short: "B",
        long_singular: "byte",
        long_plural: "bytes",
    },
    UnitLabels {
        short: "KB",
        long_singular: "kilobyte",
        long_plural: "kilobytes",
    },
    UnitLabels {
        short: "MB",
        long_singular: "megabyte",
        long_plural: "megabytes",
    },
    UnitLabels {
        short: "GB",
        long_singular: "gigabyte",
        long_plural: "gigabytes",
    },
    UnitLabels {
        short: "TB",
        long_singular: "terabyte",
        long_plural: "terabytes",
    },
    UnitLabels {
        short: "PB",
        long_singular: "petabyte",
        long_plural: "petabytes",
    },
    UnitLabels {
        short: "EB",
        long_singular: "exabyte",
        long_plural: "exabytes",
    },
];

const BINARY_LABELS: [UnitLabels; 7] = [
    UnitLabels {
        short: "B",
        long_singular: "byte",
        long_plural: "bytes",
    },
    UnitLabels {
        short: "KiB",
        long_singular: "kibibyte",
        long_plural: "kibibytes",
    },
    UnitLabels {
        short: "MiB",
        long_singular: "mebibyte",
        long_plural: "mebibytes",
    },
    UnitLabels {
        short: "GiB",
        long_singular: "gibibyte",
        long_plural: "gibibytes",
    },
    UnitLabels {
        short: "TiB",
        long_singular: "tebibyte",
        long_plural: "tebibytes",
    },
    UnitLabels {
        short: "PiB",
        long_singular: "pebibyte",
        long_plural: "pebibytes",
    },
    UnitLabels {
        short: "EiB",
        long_singular: "exbibyte",
        long_plural: "exbibytes",
    },
];

// --- Bit Labels ---

const DECIMAL_BIT_LABELS: [UnitLabels; 7] = [
    UnitLabels {
        short: "b",
        long_singular: "bit",
        long_plural: "bits",
    },
    UnitLabels {
        short: "Kb",
        long_singular: "kilobit",
        long_plural: "kilobits",
    },
    UnitLabels {
        short: "Mb",
        long_singular: "megabit",
        long_plural: "megabits",
    },
    UnitLabels {
        short: "Gb",
        long_singular: "gigabit",
        long_plural: "gigabits",
    },
    UnitLabels {
        short: "Tb",
        long_singular: "terabit",
        long_plural: "terabits",
    },
    UnitLabels {
        short: "Pb",
        long_singular: "petabit",
        long_plural: "petabits",
    },
    UnitLabels {
        short: "Eb",
        long_singular: "exabit",
        long_plural: "exabits",
    },
];

const BINARY_BIT_LABELS: [UnitLabels; 7] = [
    UnitLabels {
        short: "b",
        long_singular: "bit",
        long_plural: "bits",
    },
    UnitLabels {
        short: "Kib",
        long_singular: "kibibit",
        long_plural: "kibibits",
    },
    UnitLabels {
        short: "Mib",
        long_singular: "mebibit",
        long_plural: "mebibits",
    },
    UnitLabels {
        short: "Gib",
        long_singular: "gibibit",
        long_plural: "gibibits",
    },
    UnitLabels {
        short: "Tib",
        long_singular: "tebibit",
        long_plural: "tebibits",
    },
    UnitLabels {
        short: "Pib",
        long_singular: "pebibit",
        long_plural: "pebibits",
    },
    UnitLabels {
        short: "Eib",
        long_singular: "exbibit",
        long_plural: "exbibits",
    },
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
    let (negative, mut magnitude) = match value {
        BytesValue::Int(v) if v < 0 => (true, v.unsigned_abs()),
        BytesValue::Int(v) => (false, v as u128),
        BytesValue::UInt(v) => (false, v),
    };

    if options.bits {
        // Handle potential overflow if formatting extreme u128s in bits mode.
        magnitude = magnitude.saturating_mul(8);
    }

    let min_unit = options.min_unit as usize;
    let max_unit = (options.max_unit as usize).min(6).max(min_unit);

    let (raw_idx, table) = if options.binary {
        let idx = if magnitude == 0 {
            0
        } else {
            ((magnitude.ilog2() / 10) as usize).min(6)
        };
        (idx, &BINARY_UNITS)
    } else {
        let idx = if magnitude == 0 {
            0
        } else {
            ((magnitude.ilog10() / 3) as usize).min(6)
        };
        (idx, &DECIMAL_UNITS)
    };

    let mut idx = raw_idx.clamp(min_unit, max_unit);
    let mut unit = table[idx];
    let rounding = options.rounding;

    let get_parts = |u: u128| match options.precision {
        Precision::Decimals(p) => (
            p,
            decimal_parts_rounded(magnitude, u, p, rounding, negative),
        ),
        Precision::Significant(n) => {
            crate::common::fmt::compute_sigfigs_u128(magnitude, u, n, rounding, negative)
        }
    };

    let (mut precision, mut parts) = get_parts(unit);

    let boundary = if options.binary { 1_024 } else { 1_000 };
    if parts.integer >= boundary && idx < max_unit {
        idx += 1;
        unit = table[idx];
        let res = get_parts(unit);
        precision = res.0;
        parts = res.1;
    }

    if negative && magnitude != 0 {
        f.write_str("-")?;
    }

    write_u128(f, parts.integer, false, ',')?;

    if options.fixed_precision {
        if precision > 0 {
            f.write_char(options.decimal_separator)?;
            let existing = parts.frac_len as usize;
            write_frac_digits(f, &parts.frac_digits[..existing])?;
            for _ in existing..precision as usize {
                f.write_char('0')?;
            }
        }
    } else if parts.frac_len != 0 {
        f.write_char(options.decimal_separator)?;
        write_frac_digits(f, &parts.frac_digits[..parts.frac_len as usize])?;
    }

    let labels = match (options.bits, options.binary) {
        (false, false) => &DECIMAL_LABELS,
        (false, true) => &BINARY_LABELS,
        (true, false) => &DECIMAL_BIT_LABELS,
        (true, true) => &BINARY_BIT_LABELS,
    };

    if options.long_units {
        let label = if parts.is_exactly_one() {
            labels[idx].long_singular
        } else {
            labels[idx].long_plural
        };
        write!(f, " {label}")
    } else {
        if options.space {
            f.write_char(' ')?;
        }
        f.write_str(labels[idx].short)
    }
}

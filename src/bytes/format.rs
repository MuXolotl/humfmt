use core::fmt;
use core::fmt::Write;

use super::{traits::BytesValue, BytesOptions};
use crate::common::fmt::{decimal_parts_rounded, write_frac_digits, write_u128};
use crate::RoundingMode;

// Each entry groups short label, long singular, and long plural for one unit tier.
// Index 0 = bytes, 1 = kilo/kibi, ..., 6 = exa/exbi.
struct UnitLabels {
    short: &'static str,
    long_singular: &'static str,
    long_plural: &'static str,
}

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
    let mut parts =
        decimal_parts_rounded(magnitude, unit, precision, RoundingMode::HalfUp, negative);

    let boundary = if options.binary { 1_024 } else { 1_000 };
    if parts.integer >= boundary && idx < max_idx {
        idx += 1;
        unit = table[idx];
        parts = decimal_parts_rounded(magnitude, unit, precision, RoundingMode::HalfUp, negative);
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

    let labels = if options.binary {
        &BINARY_LABELS
    } else {
        &DECIMAL_LABELS
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
